use std::path::Path;
use zbus::blocking::Connection;
use zbus::zvariant::{SerializeDict, Type};

pub fn set_badge(count: i32) -> Result<(), String> {
    let desktop_id = resolve_desktop_id()?;
    let app_uri = format!("application://{desktop_id}");

    let props = if count <= 0 {
        BadgeProps {
            count: None,
            count_visible: false,
        }
    } else {
        BadgeProps {
            count: Some(count.min(99) as i64),
            count_visible: true,
        }
    };

    let conn = Connection::session().map_err(|e| format!("D-Bus session connection failed: {e}"))?;
    conn.emit_signal(
        None,
        "/com/canonical/unity/launcherentry/1",
        "com.canonical.Unity.LauncherEntry",
        "Update",
        &(app_uri, props),
    )
    .map_err(|e| format!("Unity LauncherEntry Update failed: {e}"))?;

    Ok(())
}

fn resolve_desktop_id() -> Result<String, String> {
    if let Ok(path) = std::env::var("GIO_LAUNCHED_DESKTOP_FILE") {
        if let Some(name) = Path::new(&path)
            .file_name()
            .and_then(|name| name.to_str())
            .filter(|name| name.ends_with(".desktop"))
        {
            return Ok(name.to_string());
        }
    }

    if let Ok(id) = std::env::var("GAPPLICATION_ID") {
        if !id.is_empty() {
            return Ok(format!("{id}.desktop"));
        }
    }

    Err(
        "Cannot resolve .desktop file id; set GAPPLICATION_ID in the Linux runner or launch from a .desktop entry"
            .into(),
    )
}

#[derive(SerializeDict, Type)]
#[zvariant(signature = "a{sv}")]
struct BadgeProps {
    count: Option<i64>,
    #[zvariant(rename = "count-visible")]
    count_visible: bool,
}

pub fn request_badge_permission() -> Result<bool, String> {
    Ok(true)
}

pub fn is_badge_permission_granted() -> Result<bool, String> {
    Ok(true)
}
