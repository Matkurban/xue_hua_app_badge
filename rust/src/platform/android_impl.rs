#[path = "android/context.rs"]
mod context;

pub fn set_badge(count: i32) -> Result<(), String> {
    context::call_badge_helper(count)
}

pub fn request_badge_permission() -> Result<bool, String> {
    context::call_request_badge_permission()
}

pub fn is_badge_permission_granted() -> Result<bool, String> {
    context::call_is_badge_permission_granted()
}
