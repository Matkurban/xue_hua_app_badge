# GeneratedPluginRegistrant constructs this class by name.
-keep class com.kurban.xue_hua_app_badge.XueHuaAppBadgePlugin {
    public <init>();
}

# ShortcutBadger 1.1.22 stores these as Class literals and calls Class.newInstance().
# Its own proguard.txt omits most of them and names AsusHomeBadger incorrectly.
# newInstance() also needs the interface methods, which R8 will not see as called.
-keep class me.leolin.shortcutbadger.impl.AdwHomeBadger,
            me.leolin.shortcutbadger.impl.ApexHomeBadger,
            me.leolin.shortcutbadger.impl.AsusHomeBadger,
            me.leolin.shortcutbadger.impl.DefaultBadger,
            me.leolin.shortcutbadger.impl.EverythingMeHomeBadger,
            me.leolin.shortcutbadger.impl.HuaweiHomeBadger,
            me.leolin.shortcutbadger.impl.NewHtcHomeBadger,
            me.leolin.shortcutbadger.impl.NovaHomeBadger,
            me.leolin.shortcutbadger.impl.OPPOHomeBader,
            me.leolin.shortcutbadger.impl.SamsungHomeBadger,
            me.leolin.shortcutbadger.impl.SonyHomeBadger,
            me.leolin.shortcutbadger.impl.VivoHomeBadger,
            me.leolin.shortcutbadger.impl.ZTEHomeBadger,
            me.leolin.shortcutbadger.impl.ZukHomeBadger {
    <init>(...);
    public void executeBadge(android.content.Context, android.content.ComponentName, int);
    public java.util.List getSupportLaunchers();
}
