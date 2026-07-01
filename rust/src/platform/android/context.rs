use jni::errors::LogContextErrorAndDefault;
use jni::objects::{Global, JClass, JObject, JValue};
use jni::sys::jint;
use jni::{EnvUnowned, JavaVM, errors::Result as JniResult, jni_sig, jni_str};
use std::ffi::c_void;
use std::sync::{Arc, Mutex, Once, OnceLock};

/// Keeps the application Context alive after passing its raw pointer to ndk-context.
static CONTEXT_HOLDER: OnceLock<Global<JObject>> = OnceLock::new();
static ACTIVITY: Mutex<Option<Arc<Global<JObject>>>> = Mutex::new(None);
static ANDROID_CONTEXT_INIT: Once = Once::new();

fn ensure_initialized() -> Result<(), String> {
    if CONTEXT_HOLDER.get().is_some() {
        Ok(())
    } else {
        Err("android context was not initialized".into())
    }
}

fn java_vm() -> Result<JavaVM, String> {
    ensure_initialized()?;
    let ctx = ndk_context::android_context();
    Ok(unsafe { JavaVM::from_raw(ctx.vm().cast()) })
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_com_flutter_1rust_1bridge_xue_1hua_1app_1badge_XueHuaAppBadgePlugin_initAndroid<
    'local,
>(
    mut unowned_env: EnvUnowned<'local>,
    _class: JClass<'local>,
    context: JObject<'local>,
) {
    ANDROID_CONTEXT_INIT.call_once(|| {
        unowned_env
            .with_env(|env| -> JniResult<()> {
                let global_ref = env.new_global_ref(context)?;
                let vm = env.get_java_vm()?;
                let vm_ptr = vm.get_raw() as *mut c_void;
                let ctx_ptr = global_ref.as_obj().as_raw() as *mut c_void;
                unsafe {
                    ndk_context::initialize_android_context(vm_ptr, ctx_ptr);
                }
                let _ = CONTEXT_HOLDER.set(global_ref);
                Ok(())
            })
            .resolve_with::<LogContextErrorAndDefault, _>(|| {
                "[xue_hua_app_badge] initAndroid".to_string()
            });
    });
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_com_flutter_1rust_1bridge_xue_1hua_1app_1badge_XueHuaAppBadgePlugin_initActivity<
    'local,
>(
    mut unowned_env: EnvUnowned<'local>,
    _class: JClass<'local>,
    activity: JObject<'local>,
) {
    unowned_env
        .with_env(|env| -> JniResult<()> {
            let global_ref = env.new_global_ref(activity)?;
            *ACTIVITY.lock().unwrap_or_else(|e| e.into_inner()) = Some(Arc::new(global_ref));
            Ok(())
        })
        .resolve_with::<LogContextErrorAndDefault, _>(|| {
            "[xue_hua_app_badge] initActivity".to_string()
        });
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_com_flutter_1rust_1bridge_xue_1hua_1app_1badge_XueHuaAppBadgePlugin_clearActivity<
    'local,
>(
    _unowned_env: EnvUnowned<'local>,
    _class: JClass<'local>,
) {
    *ACTIVITY.lock().unwrap_or_else(|e| e.into_inner()) = None;
}

pub fn call_badge_helper(count: i32) -> Result<(), String> {
    let vm = java_vm()?;
    let ctx_raw = ndk_context::android_context().context().cast();

    let badge_count = count.min(99).max(0);

    let applied = vm
        .attach_current_thread(|env| -> JniResult<bool> {
            let context = unsafe { JObject::from_raw(env, ctx_raw) };
            env.call_static_method(
                jni_str!("com/flutter_rust_bridge/xue_hua_app_badge/BadgeHelper"),
                jni_str!("applyBadge"),
                jni_sig!("(Landroid/content/Context;I)Z"),
                &[JValue::Object(&context), JValue::Int(badge_count as jint)],
            )?
            .z()
        })
        .map_err(|e| format!("BadgeHelper.applyBadge JNI call failed: {e}"))?;

    if applied {
        Ok(())
    } else {
        Err("BadgeHelper.applyBadge returned false".into())
    }
}

pub fn call_is_badge_permission_granted() -> Result<bool, String> {
    let vm = java_vm()?;
    let ctx_raw = ndk_context::android_context().context().cast();

    vm.attach_current_thread(|env| -> JniResult<bool> {
        let context = unsafe { JObject::from_raw(env, ctx_raw) };
        env.call_static_method(
            jni_str!("com/flutter_rust_bridge/xue_hua_app_badge/PermissionHelper"),
            jni_str!("isBadgePermissionGranted"),
            jni_sig!("(Landroid/content/Context;)Z"),
            &[JValue::Object(&context)],
        )?
        .z()
    })
    .map_err(|e| format!("PermissionHelper.isBadgePermissionGranted JNI call failed: {e}"))
}

pub fn call_request_badge_permission() -> Result<bool, String> {
    let vm = java_vm()?;
    let activity = ACTIVITY
        .lock()
        .unwrap_or_else(|e| e.into_inner())
        .clone()
        .ok_or("Android activity not available; ensure Flutter activity is attached")?;

    vm.attach_current_thread(|env| -> JniResult<bool> {
        env.call_static_method(
            jni_str!("com/flutter_rust_bridge/xue_hua_app_badge/PermissionHelper"),
            jni_str!("requestBadgePermission"),
            jni_sig!("(Landroid/app/Activity;)Z"),
            &[JValue::Object(activity.as_ref())],
        )?
        .z()
    })
    .map_err(|e| format!("PermissionHelper.requestBadgePermission JNI call failed: {e}"))
}
