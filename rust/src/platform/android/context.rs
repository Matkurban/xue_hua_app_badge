use jni::objects::{Global, JClass, JObject, JValue};
use jni::sys::jint;
use jni::{errors::Result as JniResult, jni_sig, jni_str, EnvUnowned, JavaVM};
use std::sync::{Arc, Mutex, OnceLock};

static CONTEXT: OnceLock<Global<JObject>> = OnceLock::new();
static ACTIVITY: Mutex<Option<Arc<Global<JObject>>>> = Mutex::new(None);
static JVM: OnceLock<JavaVM> = OnceLock::new();

#[no_mangle]
pub extern "system" fn Java_com_flutter_1rust_1bridge_xue_1hua_1app_1badge_XueHuaAppBadgePlugin_initAndroid<
    'local,
>(
    mut unowned_env: EnvUnowned<'local>,
    _class: JClass<'local>,
    context: JObject<'local>,
) {
    let _ = unowned_env.with_env(|env| -> JniResult<()> {
        let global_ref = env.new_global_ref(context)?;
        let vm = env.get_java_vm()?;
        let _ = JVM.set(vm);
        let _ = CONTEXT.set(global_ref);
        Ok(())
    });
}

#[no_mangle]
pub extern "system" fn Java_com_flutter_1rust_1bridge_xue_1hua_1app_1badge_XueHuaAppBadgePlugin_initActivity<
    'local,
>(
    mut unowned_env: EnvUnowned<'local>,
    _class: JClass<'local>,
    activity: JObject<'local>,
) {
    let _ = unowned_env.with_env(|env| -> JniResult<()> {
        let global_ref = env.new_global_ref(activity)?;
        *ACTIVITY.lock().unwrap() = Some(Arc::new(global_ref));
        Ok(())
    });
}

#[no_mangle]
pub extern "system" fn Java_com_flutter_1rust_1bridge_xue_1hua_1app_1badge_XueHuaAppBadgePlugin_clearActivity<
    'local,
>(
    _unowned_env: EnvUnowned<'local>,
    _class: JClass<'local>,
) {
    *ACTIVITY.lock().unwrap() = None;
}

pub fn is_initialized() -> bool {
    CONTEXT.get().is_some()
}

pub fn call_badge_helper(count: i32) -> Result<(), String> {
    let vm = JVM
        .get()
        .ok_or("Android JavaVM not initialized; ensure XueHuaAppBadgePlugin is registered")?;
    let context = CONTEXT
        .get()
        .ok_or("Android context not initialized; ensure XueHuaAppBadgePlugin is registered")?;

    let badge_count = count.min(99).max(0);

    let applied = vm
        .attach_current_thread(|env| -> JniResult<bool> {
            env.call_static_method(
                jni_str!("com/flutter_rust_bridge/xue_hua_app_badge/BadgeHelper"),
                jni_str!("applyBadge"),
                jni_sig!("(Landroid/content/Context;I)Z"),
                &[
                    JValue::Object(context.as_ref()),
                    JValue::Int(badge_count as jint),
                ],
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
    let vm = JVM
        .get()
        .ok_or("Android JavaVM not initialized; ensure XueHuaAppBadgePlugin is registered")?;
    let context = CONTEXT
        .get()
        .ok_or("Android context not initialized; ensure XueHuaAppBadgePlugin is registered")?;

    vm.attach_current_thread(|env| -> JniResult<bool> {
        env.call_static_method(
            jni_str!("com/flutter_rust_bridge/xue_hua_app_badge/PermissionHelper"),
            jni_str!("isBadgePermissionGranted"),
            jni_sig!("(Landroid/content/Context;)Z"),
            &[JValue::Object(context.as_ref())],
        )?
        .z()
    })
    .map_err(|e| format!("PermissionHelper.isBadgePermissionGranted JNI call failed: {e}"))
}

pub fn call_request_badge_permission() -> Result<bool, String> {
    let vm = JVM
        .get()
        .ok_or("Android JavaVM not initialized; ensure XueHuaAppBadgePlugin is registered")?;
    let activity = ACTIVITY
        .lock()
        .unwrap()
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
