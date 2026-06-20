#[cfg(target_os = "windows")]
pub mod win_impl;

#[cfg(target_os = "macos")]
pub mod macos_impl;

#[cfg(target_os = "linux")]
pub mod linux_impl;

#[cfg(target_os = "ios")]
pub mod ios_impl;

#[cfg(target_os = "android")]
pub mod android_impl;

pub(crate) fn format_badge_label(count: i32) -> String {
    if count <= 0 {
        String::new()
    } else if count > 99 {
        "99+".to_string()
    } else {
        count.to_string()
    }
}
