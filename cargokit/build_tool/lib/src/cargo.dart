/// This is copied from Cargokit (which is the official way to use it currently)
/// Details: https://fzyzcjy.github.io/flutter_rust_bridge/manual/integrate/builtin

import 'dart:io';

import 'package:path/path.dart' as path;
import 'package:toml/toml.dart';
import 'package:yaml/yaml.dart';

class ManifestException {
  ManifestException(this.message, {required this.fileName});

  final String? fileName;
  final String message;

  @override
  String toString() {
    if (fileName != null) {
      return 'Failed to parse package manifest at $fileName: $message';
    } else {
      return 'Failed to parse package manifest: $message';
    }
  }
}

class CrateInfo {
  CrateInfo({
    required this.packageName,
    required this.crateVersion,
    required this.packageVersion,
  });

  final String packageName;
  final String crateVersion;
  final String packageVersion;

  static CrateInfo parseManifest(String manifest, {final String? fileName}) {
    final toml = TomlDocument.parse(manifest);
    final package = toml.toMap()['package'];
    if (package == null) {
      throw ManifestException('Missing package section', fileName: fileName);
    }
    final name = package['name'];
    if (name == null) {
      throw ManifestException('Missing package name', fileName: fileName);
    }
    final version = package['version'];
    if (version == null) {
      throw ManifestException('Missing package version', fileName: fileName);
    }
    return CrateInfo(
      packageName: name as String,
      crateVersion: version,
      packageVersion: version,
    );
  }

  static CrateInfo load(String manifestDir) {
    final manifestFile = File(path.join(manifestDir, 'Cargo.toml'));
    final manifest = manifestFile.readAsStringSync();
    final crateInfo = parseManifest(manifest, fileName: manifestFile.path);
    final pubspecFile = File(path.join(manifestDir, '..', 'pubspec.yaml'));
    if (!pubspecFile.existsSync()) {
      return crateInfo;
    }

    final yaml = loadYaml(pubspecFile.readAsStringSync());
    if (yaml is YamlMap) {
      final version = yaml['version'];
      if (version is String && version.isNotEmpty) {
        return CrateInfo(
          packageName: crateInfo.packageName,
          crateVersion: crateInfo.crateVersion,
          packageVersion: version,
        );
      }
    }

    return crateInfo;
  }
}
