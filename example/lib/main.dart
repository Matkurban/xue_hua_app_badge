import 'package:material_ui/material_ui.dart';
import 'package:xue_hua_app_badge/xue_hua_app_badge.dart';

void main() {
  WidgetsFlutterBinding.ensureInitialized();
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
      if (await XueHuaAppBadge.instance.isPermissionGranted()) {
        return;
      }
      final granted = await XueHuaAppBadge.instance.requestPermission();
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

  Future<void> _updateBadge(int count) async {
    setState(() {
      _badgeCount = count;
      _lastError = null;
    });

    try {
      await XueHuaAppBadge.instance.set(count);
    } catch (error) {
      if (mounted) {
        setState(() => _lastError = error.toString());
      }
    }
  }

  Future<void> _clearBadge() async {
    setState(() {
      _badgeCount = 0;
      _lastError = null;
    });

    try {
      await XueHuaAppBadge.instance.remove();
    } catch (error) {
      if (mounted) {
        setState(() => _lastError = error.toString());
      }
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
              Text(
                'Badge count: $_badgeCount',
                key: const Key('badge_count'),
                style: Theme.of(context).textTheme.headlineMedium,
              ),
              const SizedBox(height: 24),
              Wrap(
                spacing: 12,
                runSpacing: 12,
                alignment: WrapAlignment.center,
                children: [
                  FilledButton(
                    key: const Key('badge_increment'),
                    onPressed: () => _updateBadge(_badgeCount + 1),
                    child: const Text('+1'),
                  ),
                  FilledButton(
                    key: const Key('badge_decrement'),
                    onPressed: _badgeCount > 0
                        ? () => _updateBadge(_badgeCount - 1)
                        : null,
                    child: const Text('-1'),
                  ),
                  OutlinedButton(
                    key: const Key('badge_clear'),
                    onPressed: _clearBadge,
                    child: const Text('Clear'),
                  ),
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
                    key: const Key('badge_permission_hint'),
                    style: TextStyle(
                      color: Theme.of(context).colorScheme.tertiary,
                    ),
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
                    key: const Key('badge_error'),
                    style: TextStyle(
                      color: Theme.of(context).colorScheme.error,
                    ),
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
