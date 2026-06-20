#[path = "android/context.rs"]
mod context;

pub fn set_badge(count: i32) -> Result<(), String> {
    if !context::is_initialized() {
        return Err(
            "Android context not initialized; add XueHuaAppBadgePlugin to your Flutter app"
                .into(),
        );
    }

    context::call_badge_helper(count)
}

pub fn request_badge_permission() -> Result<bool, String> {
    if !context::is_initialized() {
        return Err(
            "Android context not initialized; add XueHuaAppBadgePlugin to your Flutter app"
                .into(),
        );
    }

    context::call_request_badge_permission()
}

pub fn is_badge_permission_granted() -> Result<bool, String> {
    if !context::is_initialized() {
        return Err(
            "Android context not initialized; add XueHuaAppBadgePlugin to your Flutter app"
                .into(),
        );
    }

    context::call_is_badge_permission_granted()
}