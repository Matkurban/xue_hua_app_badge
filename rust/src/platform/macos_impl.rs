use dispatch2::DispatchQueue;
use objc2::MainThreadMarker;
use objc2_app_kit::NSApplication;
use objc2_foundation::NSString;
use std::sync::{Arc, Mutex};

pub fn set_badge(count: i32) -> Result<(), String> {
    run_on_main_thread(move || set_badge_on_main_thread(count))
}

fn run_on_main_thread<F>(f: F) -> Result<(), String>
where
    F: FnOnce() -> Result<(), String> + Send + 'static,
{
    if MainThreadMarker::new().is_some() {
        return f();
    }

    let result = Arc::new(Mutex::new(None::<Result<(), String>>));
    let result_clone = Arc::clone(&result);

    DispatchQueue::main().exec_sync(move || {
        *result_clone.lock().unwrap() = Some(f());
    });

    let output = result
        .lock()
        .unwrap()
        .take()
        .unwrap_or_else(|| Err("Main thread dispatch failed".into()));
    output
}

fn set_badge_on_main_thread(count: i32) -> Result<(), String> {
    let mtm = MainThreadMarker::new().ok_or("NSApplication APIs require the main thread")?;
    let app = NSApplication::sharedApplication(mtm);
    let dock_tile = app.dockTile();

    let label = super::format_badge_label(count);
    if label.is_empty() {
        dock_tile.setBadgeLabel(None);
    } else {
        let ns_label = NSString::from_str(&label);
        dock_tile.setBadgeLabel(Some(&ns_label));
    }

    dock_tile.display();
    Ok(())
}
