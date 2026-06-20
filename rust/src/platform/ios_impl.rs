use block2::RcBlock;
use core::ptr::NonNull;
use dispatch2::DispatchQueue;
use objc2::runtime::Bool;
use objc2::MainThreadMarker;
use objc2_foundation::NSError;
use objc2_ui_kit::{UIApplication, UIDevice};
use objc2_user_notifications::{
    UNAuthorizationOptions, UNAuthorizationStatus, UNNotificationSettings, UNUserNotificationCenter,
};
use std::sync::{Arc, Condvar, Mutex};

pub fn set_badge(count: i32) -> Result<(), String> {
    run_on_main_thread(move || set_badge_on_main_thread(count))
}

pub fn request_badge_permission() -> Result<bool, String> {
    run_on_main_thread(request_badge_permission_on_main_thread)
}

pub fn is_badge_permission_granted() -> Result<bool, String> {
    run_on_main_thread(is_badge_permission_granted_on_main_thread)
}

fn run_on_main_thread<F, T>(f: F) -> Result<T, String>
where
    F: FnOnce() -> Result<T, String> + Send + 'static,
    T: Send + 'static,
{
    if MainThreadMarker::new().is_some() {
        return f();
    }

    let result = Arc::new(Mutex::new(None::<Result<T, String>>));
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
    let mtm = MainThreadMarker::new().ok_or("UIKit APIs require the main thread")?;
    let badge_number = super::badge_number(count);

    if ios_16_or_later(mtm) {
        set_badge_modern(badge_number)
    } else {
        set_badge_legacy(mtm, badge_number)
    }
}

fn ios_16_or_later(mtm: MainThreadMarker) -> bool {
    let device = UIDevice::currentDevice(mtm);
    let version_str = device.systemVersion().to_string();
    version_str
        .split('.')
        .next()
        .and_then(|major| major.parse::<u32>().ok())
        .map(|major| major >= 16)
        .unwrap_or(false)
}

fn set_badge_modern(badge_number: i32) -> Result<(), String> {
    let center = UNUserNotificationCenter::currentNotificationCenter();
    let wait = Arc::new((Mutex::new(None), Condvar::new()));
    let wait_clone = Arc::clone(&wait);

    let block = RcBlock::new(move |error: *mut NSError| {
        let output = if error.is_null() {
            Ok(())
        } else {
            Err(format!("setBadgeCount failed: {error:?}"))
        };
        let (lock, cvar) = &*wait_clone;
        *lock.lock().unwrap() = Some(output);
        cvar.notify_one();
    });

    center.setBadgeCount_withCompletionHandler(badge_number as _, Some(&block));

    let (lock, cvar) = &*wait;
    let mut guard = lock.lock().unwrap();
    while guard.is_none() {
        guard = cvar.wait(guard).unwrap();
    }
    guard.take().unwrap()
}

#[allow(deprecated)]
fn set_badge_legacy(mtm: MainThreadMarker, badge_number: i32) -> Result<(), String> {
    let app = UIApplication::sharedApplication(mtm);
    app.setApplicationIconBadgeNumber(badge_number as _);
    Ok(())
}

fn request_badge_permission_on_main_thread() -> Result<bool, String> {
    let mtm = MainThreadMarker::new().ok_or("UIKit APIs require the main thread")?;
    if !ios_16_or_later(mtm) {
        return Ok(true);
    }

    let center = UNUserNotificationCenter::currentNotificationCenter();
    let wait = Arc::new((Mutex::new(None::<bool>), Condvar::new()));
    let wait_clone = Arc::clone(&wait);

    let block = RcBlock::new(move |granted: Bool, _error: *mut NSError| {
        let (lock, cvar) = &*wait_clone;
        *lock.lock().unwrap() = Some(granted.as_bool());
        cvar.notify_one();
    });

    center.requestAuthorizationWithOptions_completionHandler(UNAuthorizationOptions::Badge, &block);

    let (lock, cvar) = &*wait;
    let mut guard = lock.lock().unwrap();
    while guard.is_none() {
        guard = cvar.wait(guard).unwrap();
    }
    Ok(guard.take().unwrap())
}

fn is_badge_permission_granted_on_main_thread() -> Result<bool, String> {
    let mtm = MainThreadMarker::new().ok_or("UIKit APIs require the main thread")?;
    if !ios_16_or_later(mtm) {
        return Ok(true);
    }

    let center = UNUserNotificationCenter::currentNotificationCenter();
    let wait = Arc::new((Mutex::new(None::<bool>), Condvar::new()));
    let wait_clone = Arc::clone(&wait);

    let block = RcBlock::new(move |settings_ptr: NonNull<UNNotificationSettings>| {
        let settings = unsafe { settings_ptr.as_ref() };
        let status = settings.authorizationStatus();
        let granted = status == UNAuthorizationStatus::Authorized
            || status == UNAuthorizationStatus::Provisional;
        let (lock, cvar) = &*wait_clone;
        *lock.lock().unwrap() = Some(granted);
        cvar.notify_one();
    });

    center.getNotificationSettingsWithCompletionHandler(&block);

    let (lock, cvar) = &*wait;
    let mut guard = lock.lock().unwrap();
    while guard.is_none() {
        guard = cvar.wait(guard).unwrap();
    }
    Ok(guard.take().unwrap())
}
