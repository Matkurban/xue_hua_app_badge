/// This is copied from Cargokit (which is the official way to use it currently)
/// Details: https://fzyzcjy.github.io/flutter_rust_bridge/manual/integrate/builtin

import 'dart:async';
import 'dart:io';

import 'package:ed25519_edwards/ed25519_edwards.dart';
import 'package:github/github.dart';
import 'package:http/http.dart' as http;
import 'package:logging/logging.dart';
import 'package:path/path.dart' as path;

import 'artifacts_provider.dart';
import 'builder.dart';
import 'cargo.dart';
import 'crate_hash.dart';
import 'options.dart';
import 'rustup.dart';
import 'target.dart';

final _log = Logger('precompile_binaries');

class PrecompileBinaries {
  PrecompileBinaries({
    required this.privateKey,
    required this.githubToken,
    required this.repositorySlug,
    required this.manifestDir,
    required this.targets,
    this.androidSdkLocation,
    this.androidNdkVersion,
    this.androidMinSdkVersion,
    this.tempDir,
    this.glibcVersion,
  });

  final PrivateKey privateKey;
  final String githubToken;
  final RepositorySlug repositorySlug;
  final String manifestDir;
  final List<Target> targets;
  final String? androidSdkLocation;
  final String? androidNdkVersion;
  final int? androidMinSdkVersion;
  final String? tempDir;
  final String? glibcVersion;

  static String fileName(Target target, String name) {
    return '${target.rust}_$name';
  }

  static String signatureFileName(Target target, String name) {
    return '${target.rust}_$name.sig';
  }

  Future<void> run() async {
    final crateInfo = CrateInfo.load(manifestDir);
    final packageVersion = crateInfo.packageVersion;

    final targets = List.of(this.targets);
    if (targets.isEmpty) {
      targets.addAll([
        ...Target.buildableTargets(),
        if (androidSdkLocation != null) ...Target.androidTargets(),
      ]);
    }

    _log.info('Precompiling binaries for $targets');

    final hash = CrateHash.compute(manifestDir);
    _log.info('Computed crate hash: $hash');

    final String tagName = 'precompiled_$packageVersion';

    final github = GitHub(auth: Authentication.withToken(githubToken));
    final repo = github.repositories;
    final release = await _getOrCreateRelease(
      repo: repo,
      tagName: tagName,
      packageName: crateInfo.packageName,
      packageVersion: packageVersion,
      hash: hash,
    );
    final releaseAssets = await _listReleaseAssets(
      repo: repo,
      release: release,
    );

    final tempDir = this.tempDir != null
        ? Directory(this.tempDir!)
        : Directory.systemTemp.createTempSync('precompiled_');

    tempDir.createSync(recursive: true);

    final crateOptions = CargokitCrateOptions.load(
      manifestDir: manifestDir,
    );

    final buildEnvironment = BuildEnvironment(
      configuration: BuildConfiguration.release,
      crateOptions: crateOptions,
      targetTempDir: tempDir.path,
      manifestDir: manifestDir,
      crateInfo: crateInfo,
      isAndroid: androidSdkLocation != null,
      androidSdkPath: androidSdkLocation,
      androidNdkVersion: androidNdkVersion,
      androidMinSdkVersion: androidMinSdkVersion,
      javaHome: Platform.environment['JAVA_HOME'],
      glibcVersion: glibcVersion,
    );

    final rustup = Rustup();

    for (final target in targets) {
      final artifactNames = getArtifactNames(
        target: target,
        libraryName: crateInfo.packageName,
        remote: true,
      );

      _log.info('Building for $target');

      final builder =
          RustBuilder(target: target, environment: buildEnvironment);
      builder.prepare(rustup);
      final res = await builder.build();

      final assets = <CreateReleaseAsset>[];
      for (final name in artifactNames) {
        final file = File(path.join(res, name));
        if (!file.existsSync()) {
          throw Exception('Missing artifact: ${file.path}');
        }

        final data = file.readAsBytesSync();
        final create = CreateReleaseAsset(
          name: PrecompileBinaries.fileName(target, name),
          contentType: "application/octet-stream",
          assetData: data,
        );
        final signature = sign(privateKey, data);
        final signatureCreate = CreateReleaseAsset(
          name: signatureFileName(target, name),
          contentType: "application/octet-stream",
          assetData: signature,
        );
        bool verified = verify(public(privateKey), data, signature);
        if (!verified) {
          throw Exception('Signature verification failed');
        }
        assets.add(create);
        assets.add(signatureCreate);
      }
      _log.info('Uploading assets: ${assets.map((e) => e.name)}');
      for (final asset in assets) {
        await _uploadOrReplaceAsset(
          repo: repo,
          release: release,
          releaseAssets: releaseAssets,
          asset: asset,
        );
      }
    }

    _log.info('Cleaning up');
    tempDir.deleteSync(recursive: true);
  }

  Future<Release> _getOrCreateRelease({
    required RepositoriesService repo,
    required String tagName,
    required String packageName,
    required String packageVersion,
    required String hash,
  }) async {
    final releaseName = 'Precompiled binaries $packageVersion';
    final releaseBody = 'Precompiled binaries for crate $packageName, '
        'package version $packageVersion. Latest uploaded crate hash: $hash.';
    Release release;
    try {
      _log.info('Fetching release $tagName');
      release = await _retryGitHubOperation(
        'fetching release $tagName',
        () => repo.getReleaseByTagName(repositorySlug, tagName),
        shouldRetry: (error) =>
            error is! ReleaseNotFound && _isRetryableGitHubError(error),
      );
    } on ReleaseNotFound {
      _log.info('Release not found - creating release $tagName');
      release = await _retryGitHubOperation(
        'creating release $tagName',
        () => repo.createRelease(
          repositorySlug,
          CreateRelease.from(
            tagName: tagName,
            name: releaseName,
            targetCommitish: null,
            isDraft: false,
            isPrerelease: false,
            body: releaseBody,
          ),
        ),
      );
    }
    return _retryGitHubOperation(
      'updating release $tagName metadata',
      () => repo.editRelease(
        repositorySlug,
        release,
        name: releaseName,
        body: releaseBody,
        draft: false,
        preRelease: false,
      ),
    );
  }

  Future<List<ReleaseAsset>> _listReleaseAssets({
    required RepositoriesService repo,
    required Release release,
  }) {
    return _retryGitHubOperation(
      'listing release assets for ${release.tagName}',
      () => repo.listReleaseAssets(repositorySlug, release).toList(),
    );
  }

  Future<void> _uploadOrReplaceAsset({
    required RepositoriesService repo,
    required Release release,
    required List<ReleaseAsset> releaseAssets,
    required CreateReleaseAsset asset,
  }) async {
    try {
      await _uploadAsset(
        repo: repo,
        release: release,
        releaseAssets: releaseAssets,
        asset: asset,
      );
    } on Exception catch (error) {
      if (!_isAssetAlreadyExistsError(error)) {
        rethrow;
      }

      _log.warning(
        'Asset ${asset.name} already exists, refreshing assets and replacing it.',
      );
      final latestAssets = await _listReleaseAssets(
        repo: repo,
        release: release,
      );
      releaseAssets
        ..clear()
        ..addAll(latestAssets);

      ReleaseAsset? conflictingAsset;
      for (final existingAsset in releaseAssets) {
        if (existingAsset.name == asset.name) {
          conflictingAsset = existingAsset;
          break;
        }
      }

      if (conflictingAsset == null) {
        rethrow;
      }

      await _retryGitHubOperation(
        'deleting conflicting asset ${conflictingAsset.name}',
        () => repo.deleteReleaseAsset(repositorySlug, conflictingAsset!),
      );
      releaseAssets.removeWhere(
        (candidate) => candidate.id == conflictingAsset!.id,
      );

      await _uploadAsset(
        repo: repo,
        release: release,
        releaseAssets: releaseAssets,
        asset: asset,
      );
    }
  }

  Future<void> _uploadAsset({
    required RepositoriesService repo,
    required Release release,
    required List<ReleaseAsset> releaseAssets,
    required CreateReleaseAsset asset,
  }) async {
    final uploadedAssets = await _retryGitHubOperation(
      'uploading asset ${asset.name}',
      () => repo.uploadReleaseAssets(release, [asset]),
      shouldRetry: (error) =>
          _isRetryableGitHubError(error) && !_isAssetAlreadyExistsError(error),
    );

    releaseAssets.removeWhere((candidate) => candidate.name == asset.name);
    releaseAssets.addAll(uploadedAssets);
  }

  Future<T> _retryGitHubOperation<T>(
    String description,
    Future<T> Function() operation, {
    int maxAttempts = 5,
    bool Function(Object error)? shouldRetry,
  }) async {
    for (var attempt = 1; attempt <= maxAttempts; attempt++) {
      try {
        return await operation();
      } catch (error) {
        final retry = (shouldRetry ?? _isRetryableGitHubError)(error);
        if (!retry || attempt == maxAttempts) {
          rethrow;
        }

        final delaySeconds = 1 << (attempt - 1);
        _log.warning(
          '$description failed on attempt $attempt/$maxAttempts: $error. '
          'Retrying in ${delaySeconds}s.',
        );
        await Future.delayed(Duration(seconds: delaySeconds));
      }
    }

    throw StateError('Unreachable retry state for $description');
  }

  bool _isRetryableGitHubError(Object error) {
    return error is http.ClientException ||
        error is SocketException ||
        error is HttpException ||
        error is ServerError ||
        error is NotReady ||
        error is UnknownError;
  }

  bool _isAssetAlreadyExistsError(Object error) {
    if (error is! ValidationFailed) {
      return false;
    }
    final message = error.message ?? error.toString();
    return message.contains('Resource: ReleaseAsset') &&
        message.contains('Code: already_exists');
  }
}
