#[cfg(target_os = "windows")]
pub mod win_impl;

#[cfg(target_os = "macos")]
pub mod macos_impl;

#[cfg(target_os = "linux")]
pub mod linux_impl;

#[cfg(target_os = "ios")]
mod ios_async;

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

pub(crate) fn badge_number(count: i32) -> i32 {
    if count <= 0 {
        0
    } else if count > 99 {
        99
    } else {
        count
    }
}

#[cfg(test)]
mod tests {
    use super::{badge_number, format_badge_label};

    #[test]
    fn format_badge_label_empty_for_zero_or_negative() {
        assert_eq!(format_badge_label(0), "");
        assert_eq!(format_badge_label(-1), "");
    }

    #[test]
    fn format_badge_label_shows_count_up_to_99() {
        assert_eq!(format_badge_label(1), "1");
        assert_eq!(format_badge_label(99), "99");
    }

    #[test]
    fn format_badge_label_shows_99_plus_above_99() {
        assert_eq!(format_badge_label(100), "99+");
    }

    #[test]
    fn badge_number_clamps_to_valid_range() {
        assert_eq!(badge_number(-5), 0);
        assert_eq!(badge_number(0), 0);
        assert_eq!(badge_number(42), 42);
        assert_eq!(badge_number(99), 99);
        assert_eq!(badge_number(150), 99);
    }
}
