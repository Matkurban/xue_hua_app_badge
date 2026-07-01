use block2::RcBlock;
use core::ptr::NonNull;
use dispatch2::DispatchQueue;
use objc2::MainThreadMarker;
use objc2::runtime::Bool;
use objc2_foundation::NSError;
use objc2_ui_kit::{UIApplication, UIDevice};
use objc2_user_notifications::{
    UNAuthorizationOptions, UNAuthorizationStatus, UNNotificationSetting, UNNotificationSettings,
    UNUserNotificationCenter,
};
use std::sync::{Arc, Mutex};

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
    super::ios_async::wait_for_async_completion(|complete| {
        let complete = Arc::new(Mutex::new(Some(complete)));
        let complete_clone = Arc::clone(&complete);
        let center = UNUserNotificationCenter::currentNotificationCenter();
        let block = RcBlock::new(move |error: *mut NSError| {
            let output = if error.is_null() {
                Ok(())
            } else {
                Err(format!("setBadgeCount failed: {error:?}"))
            };
            if let Some(done) = complete_clone.lock().unwrap().take() {
                done(output);
            }
        });
        center.setBadgeCount_withCompletionHandler(badge_number as _, Some(&block));
    })
    .and_then(|inner| inner)
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

    super::ios_async::wait_for_async_completion(|complete| {
        let complete = Arc::new(Mutex::new(Some(complete)));
        let complete_clone = Arc::clone(&complete);
        let center = UNUserNotificationCenter::currentNotificationCenter();
        let block = RcBlock::new(move |granted: Bool, _error: *mut NSError| {
            if let Some(done) = complete_clone.lock().unwrap().take() {
                done(granted.as_bool());
            }
        });
        center.requestAuthorizationWithOptions_completionHandler(
            UNAuthorizationOptions::Badge,
            &block,
        );
    })
}

fn is_badge_permission_granted_on_main_thread() -> Result<bool, String> {
    let mtm = MainThreadMarker::new().ok_or("UIKit APIs require the main thread")?;
    if !ios_16_or_later(mtm) {
        return Ok(true);
    }

    super::ios_async::wait_for_async_completion(|complete| {
        let complete = Arc::new(Mutex::new(Some(complete)));
        let complete_clone = Arc::clone(&complete);
        let center = UNUserNotificationCenter::currentNotificationCenter();
        let block = RcBlock::new(move |settings_ptr: NonNull<UNNotificationSettings>| {
            let settings = unsafe { settings_ptr.as_ref() };
            if let Some(done) = complete_clone.lock().unwrap().take() {
                done(badge_permission_granted(settings));
            }
        });
        center.getNotificationSettingsWithCompletionHandler(&block);
    })
}

fn badge_permission_granted(settings: &UNNotificationSettings) -> bool {
    let status = settings.authorizationStatus();
    let authorized =
        status == UNAuthorizationStatus::Authorized || status == UNAuthorizationStatus::Provisional;
    authorized && settings.badgeSetting() == UNNotificationSetting::Enabled
}
