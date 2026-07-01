use core_foundation::runloop::{CFRunLoopRunInMode, kCFRunLoopDefaultMode};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

const DEFAULT_TIMEOUT: Duration = Duration::from_secs(30);
const POLL_INTERVAL_SECS: f64 = 0.01;

/// Waits for a main-queue async callback by pumping the RunLoop instead of
/// blocking the main thread on a Condvar (which would deadlock UIKit handlers).
pub fn wait_for_async_completion<T>(
    register: impl FnOnce(Box<dyn FnOnce(T) + Send>),
) -> Result<T, String>
where
    T: Send + 'static,
{
    let slot = Arc::new(Mutex::new(None::<T>));
    let slot_clone = Arc::clone(&slot);

    register(Box::new(move |value: T| {
        if let Ok(mut guard) = slot_clone.lock() {
            *guard = Some(value);
        }
    }));

    pump_main_run_loop_until(
        || slot.lock().map(|guard| guard.is_some()).unwrap_or(false),
        DEFAULT_TIMEOUT,
    )?;

    slot.lock()
        .map_err(|_| "Async completion mutex poisoned".to_string())?
        .take()
        .ok_or_else(|| "Async completion returned no value".into())
}

fn pump_main_run_loop_until(
    mut done: impl FnMut() -> bool,
    timeout: Duration,
) -> Result<(), String> {
    let deadline = Instant::now() + timeout;
    while !done() {
        if Instant::now() >= deadline {
            return Err("Async operation timed out".into());
        }
        unsafe {
            CFRunLoopRunInMode(kCFRunLoopDefaultMode, POLL_INTERVAL_SECS, 1);
        }
    }
    Ok(())
}
