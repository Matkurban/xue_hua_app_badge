import 'package:flutter/material.dart';
import 'package:xue_hua_app_badge/xue_hua_app_badge.dart';

Future<void> main() async {
  WidgetsFlutterBinding.ensureInitialized();
  await XueHuaAppBadge.initialize();
  runApp(const MyApp());
}

class MyApp extends StatefulWidget {
  const MyApp({super.key});

  @override
  State<MyApp> createState() => _MyAppState();
}

class _MyAppState extends State<MyApp> {
  int _badgeCount = 0;
  String? _lastError;
  String? _permissionHint;

  @override
  void initState() {
    super.initState();
    WidgetsBinding.instance.addPostFrameCallback((_) {
      _ensureBadgePermission();
    });
  }

  Future<void> _ensureBadgePermission() async {
    try {
      if (XueHuaAppBadge.isPermissionGranted()) {
        return;
      }
      final granted = XueHuaAppBadge.requestPermission();
      if (!granted && mounted) {
        setState(() {
          _permissionHint = 'Badge permission was denied. Enable notifications in system settings.';
        });
      }
    } catch (error) {
      if (mounted) {
        setState(() => _lastError = error.toString());
      }
    }
  }

  void _updateBadge(int count) {
    setState(() {
      _badgeCount = count;
      _lastError = null;
    });

    try {
      XueHuaAppBadge.set(count);
    } catch (error) {
      setState(() => _lastError = error.toString());
    }
  }

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      home: Scaffold(
        appBar: AppBar(title: const Text('xue_hua_app_badge')),
        body: Center(
          child: Column(
            mainAxisAlignment: MainAxisAlignment.center,
            children: [
              Text('Badge count: $_badgeCount', style: Theme.of(context).textTheme.headlineMedium),
              const SizedBox(height: 24),
              Wrap(
                spacing: 12,
                runSpacing: 12,
                alignment: WrapAlignment.center,
                children: [
                  FilledButton(
                    onPressed: () => _updateBadge(_badgeCount + 1),
                    child: const Text('+1'),
                  ),
                  FilledButton(
                    onPressed: _badgeCount > 0 ? () => _updateBadge(_badgeCount - 1) : null,
                    child: const Text('-1'),
                  ),
                  OutlinedButton(onPressed: () => _updateBadge(0), child: const Text('Clear')),
                ],
              ),
              const SizedBox(height: 24),
              const Padding(
                padding: EdgeInsets.symmetric(horizontal: 24),
                child: Text(
                  'macOS: dock badge · Windows: taskbar overlay · '
                  'Linux: Unity LauncherEntry (GNOME/KDE/Ubuntu) · '
                  'iOS / Android: call requestPermission() before set()',
                  textAlign: TextAlign.center,
                ),
              ),
              if (_permissionHint != null) ...[
                const SizedBox(height: 16),
                Padding(
                  padding: const EdgeInsets.symmetric(horizontal: 24),
                  child: Text(
                    _permissionHint!,
                    style: TextStyle(color: Theme.of(context).colorScheme.tertiary),
                    textAlign: TextAlign.center,
                  ),
                ),
              ],
              if (_lastError != null) ...[
                const SizedBox(height: 16),
                Padding(
                  padding: const EdgeInsets.symmetric(horizontal: 24),
                  child: SelectableText(
                    _lastError!,
                    style: TextStyle(color: Theme.of(context).colorScheme.error),
                    textAlign: TextAlign.center,
                  ),
                ),
              ],
            ],
          ),
        ),
      ),
    );
  }
}
