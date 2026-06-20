#[flutter_rust_bridge::frb(sync)]
pub fn set_badge(count: i32, #[allow(unused_variables)] window_handle: Option<i64>) -> Result<(), String> {
    if count < 0 {
        return Err("Badge count must be >= 0".into());
    }
    #[cfg(target_os = "windows")]
    {
        return crate::platform::win_impl::set_badge(count, window_handle);
    }
    #[cfg(target_os = "macos")]
    {
        return crate::platform::macos_impl::set_badge(count);
    }
    #[cfg(target_os = "linux")]
    {
        return crate::platform::linux_impl::set_badge(count);
    }
    #[cfg(target_os = "ios")]
    {
        return crate::platform::ios_impl::set_badge(count);
    }
    #[cfg(target_os = "android")]
    {
        return crate::platform::android_impl::set_badge(count);
    }
    #[cfg(not(any(
        target_os = "windows",
        target_os = "macos",
        target_os = "linux",
        target_os = "ios",
        target_os = "android"
    )))]
    {
        Err("Platform not supported".into())
    }
}

#[flutter_rust_bridge::frb(sync)]
pub fn remove_badge(window_handle: Option<i64>) -> Result<(), String> {
    set_badge(0, window_handle)
}
