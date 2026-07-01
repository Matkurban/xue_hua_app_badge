#[flutter_rust_bridge::frb(sync)]
pub fn set_badge(
    count: i32,
    #[allow(unused_variables)] window_handle: Option<i64>,
) -> Result<(), String> {
    crate::runtime::ensure_initialized()?;
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
    crate::runtime::ensure_initialized()?;
    set_badge(0, window_handle)
}

#[flutter_rust_bridge::frb(sync)]
pub fn request_badge_permission() -> Result<bool, String> {
    crate::runtime::ensure_initialized()?;
    #[cfg(target_os = "windows")]
    {
        return crate::platform::win_impl::request_badge_permission();
    }
    #[cfg(target_os = "macos")]
    {
        return crate::platform::macos_impl::request_badge_permission();
    }
    #[cfg(target_os = "linux")]
    {
        return crate::platform::linux_impl::request_badge_permission();
    }
    #[cfg(target_os = "ios")]
    {
        return crate::platform::ios_impl::request_badge_permission();
    }
    #[cfg(target_os = "android")]
    {
        return crate::platform::android_impl::request_badge_permission();
    }
    #[cfg(not(any(
        target_os = "windows",
        target_os = "macos",
        target_os = "linux",
        target_os = "ios",
        target_os = "android"
    )))]
    {
        Ok(true)
    }
}

#[flutter_rust_bridge::frb(sync)]
pub fn is_badge_permission_granted() -> Result<bool, String> {
    crate::runtime::ensure_initialized()?;
    #[cfg(target_os = "windows")]
    {
        return crate::platform::win_impl::is_badge_permission_granted();
    }
    #[cfg(target_os = "macos")]
    {
        return crate::platform::macos_impl::is_badge_permission_granted();
    }
    #[cfg(target_os = "linux")]
    {
        return crate::platform::linux_impl::is_badge_permission_granted();
    }
    #[cfg(target_os = "ios")]
    {
        return crate::platform::ios_impl::is_badge_permission_granted();
    }
    #[cfg(target_os = "android")]
    {
        return crate::platform::android_impl::is_badge_permission_granted();
    }
    #[cfg(not(any(
        target_os = "windows",
        target_os = "macos",
        target_os = "linux",
        target_os = "ios",
        target_os = "android"
    )))]
    {
        Ok(true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_negative_count() {
        crate::runtime::mark_initialized();
        let err = set_badge(-1, None).unwrap_err();
        assert_eq!(err, "Badge count must be >= 0");
    }

    #[test]
    fn requires_initialize_before_set() {
        if crate::runtime::ensure_initialized().is_err() {
            let err = set_badge(0, None).unwrap_err();
            assert_eq!(
                err,
                "Call XueHuaAppBadge.initialize() before using badge APIs"
            );
        }
    }
}
