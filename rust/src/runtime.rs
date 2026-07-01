use std::sync::OnceLock;

static INITIALIZED: OnceLock<()> = OnceLock::new();

pub fn mark_initialized() {
    let _ = INITIALIZED.set(());
}

pub fn ensure_initialized() -> Result<(), String> {
    if INITIALIZED.get().is_some() {
        Ok(())
    } else {
        Err("Call XueHuaAppBadge.initialize() before using badge APIs".into())
    }
}
