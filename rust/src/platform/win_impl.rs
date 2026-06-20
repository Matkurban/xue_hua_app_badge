use std::sync::{Mutex, OnceLock};

use windows::core::{Interface, PCWSTR};
use windows::Win32::Foundation::{BOOL, COLORREF, HICON, HWND, RECT};
use windows::Win32::Graphics::Gdi::{
    CreateBitmap, CreateCompatibleDC, CreateDIBSection, CreateFontW, CreateSolidBrush, DeleteDC,
    DeleteObject, DrawTextW, Ellipse, GetDC, ReleaseDC, SelectObject, SetBkMode, SetTextColor,
    BITMAPINFO, BITMAPINFOHEADER, BI_RGB, DIB_RGB_COLORS, DT_CENTER, DT_SINGLELINE, DT_VCENTER,
    FONT_CHARSET, FONT_CLIP_PRECISION, FONT_QUALITY, FW_BOLD, HBITMAP, HDC, HFONT, HGDIOBJ,
    ICONINFO, OUT_DEFAULT_PRECIS, TRANSPARENT,
};
use windows::Win32::System::Com::{CoCreateInstance, CoInitializeEx, CLSCTX_INPROC_SERVER, COINIT_APARTMENTTHREADED};
use windows::Win32::UI::Shell::{ITaskbarList3, TaskbarList};
use windows::Win32::UI::WindowsAndMessaging::{CreateIconIndirect, DestroyIcon, GetActiveWindow};

struct TaskbarState {
    taskbar: ITaskbarList3,
    overlay_icon: Option<HICON>,
}

static TASKBAR_STATE: OnceLock<Mutex<TaskbarState>> = OnceLock::new();

pub fn set_badge(count: i32, window_handle: Option<i64>) -> Result<(), String> {
    let hwnd = resolve_hwnd(window_handle)?;
    if hwnd.0.is_null() {
        return Err("No valid window handle found".into());
    }

    let state = taskbar_state()?;

    if count == 0 {
        clear_overlay(state, hwnd)?;
        return Ok(());
    }

    let label = super::format_badge_label(count);
    let icon = create_badge_icon(&label)?;
    apply_overlay(state, hwnd, icon, &label)
}

fn resolve_hwnd(window_handle: Option<i64>) -> Result<HWND, String> {
    if let Some(handle) = window_handle {
        let hwnd = HWND(handle as *mut _);
        if hwnd.0.is_null() {
            return Err("Provided window handle is null".into());
        }
        return Ok(hwnd);
    }

    unsafe {
        let hwnd = GetActiveWindow();
        Ok(hwnd)
    }
}

fn taskbar_state() -> Result<&'static Mutex<TaskbarState>, String> {
    TASKBAR_STATE.get_or_try_init(|| {
        unsafe {
            let _ = CoInitializeEx(None, COINIT_APARTMENTTHREADED);
        }

        let taskbar: ITaskbarList3 =
            unsafe { CoCreateInstance(&TaskbarList, None, CLSCTX_INPROC_SERVER) }
                .map_err(|e| format!("CoCreateInstance(TaskbarList) failed: {e}"))?;

        unsafe {
            taskbar
                .HrInit()
                .map_err(|e| format!("ITaskbarList3::HrInit failed: {e}"))?;
        }

        Ok(Mutex::new(TaskbarState {
            taskbar,
            overlay_icon: None,
        }))
    })
}

fn clear_overlay(state: &Mutex<TaskbarState>, hwnd: HWND) -> Result<(), String> {
    let mut guard = state
        .lock()
        .map_err(|_| "Taskbar state lock poisoned".to_string())?;

    if let Some(icon) = guard.overlay_icon.take() {
        unsafe {
            let _ = DestroyIcon(icon);
        }
    }

    unsafe {
        guard
            .taskbar
            .SetOverlayIcon(hwnd, HICON::default(), PCWSTR::null())
            .map_err(|e| format!("SetOverlayIcon(clear) failed: {e}"))?;
    }

    Ok(())
}

fn apply_overlay(
    state: &Mutex<TaskbarState>,
    hwnd: HWND,
    icon: HICON,
    description: &str,
) -> Result<(), String> {
    let mut guard = state
        .lock()
        .map_err(|_| "Taskbar state lock poisoned".to_string())?;

    if let Some(previous) = guard.overlay_icon.replace(icon) {
        unsafe {
            let _ = DestroyIcon(previous);
        }
    }

    let description = windows_string(description);
    unsafe {
        guard
            .taskbar
            .SetOverlayIcon(hwnd, icon, PCWSTR::from_raw(description.as_ptr()))
            .map_err(|e| format!("SetOverlayIcon failed: {e}"))?;
    }

    Ok(())
}

fn create_badge_icon(label: &str) -> Result<HICON, String> {
    const SIZE: i32 = 16;

    unsafe {
        let screen_dc = GetDC(None);
        if screen_dc.is_invalid() {
            return Err("GetDC failed".into());
        }

        let mem_dc = CreateCompatibleDC(screen_dc);
        if mem_dc.is_invalid() {
            ReleaseDC(None, screen_dc);
            return Err("CreateCompatibleDC failed".into());
        }

        let bmi = BITMAPINFO {
            bmiHeader: BITMAPINFOHEADER {
                biSize: std::mem::size_of::<BITMAPINFOHEADER>() as u32,
                biWidth: SIZE,
                biHeight: -SIZE,
                biPlanes: 1,
                biBitCount: 32,
                biCompression: BI_RGB.0,
                ..Default::default()
            },
            ..Default::default()
        };

        let mut bits: *mut core::ffi::c_void = std::ptr::null_mut();
        let color_bitmap = CreateDIBSection(
            Some(mem_dc),
            &bmi,
            DIB_RGB_COLORS,
            &mut bits,
            None,
            0,
        )
        .map_err(|e| format!("CreateDIBSection failed: {e}"))?;

        if bits.is_null() {
            cleanup_dc(mem_dc, screen_dc, color_bitmap, HBITMAP::default());
            return Err("CreateDIBSection returned null bits".into());
        }

        std::ptr::write_bytes(bits as *mut u8, 0, (SIZE * SIZE * 4) as usize);

        let old_bitmap = SelectObject(mem_dc, color_bitmap);

        let red_brush = CreateSolidBrush(COLORREF(0x0000_00FF));
        let old_brush = SelectObject(mem_dc, red_brush);
        let _ = Ellipse(mem_dc, 0, 0, SIZE, SIZE);
        SelectObject(mem_dc, old_brush);
        let _ = DeleteObject(red_brush);

        let face_name = windows_string("Segoe UI");
        let font = CreateFontW(
            10,
            0,
            0,
            0,
            FW_BOLD.0 as i32,
            0,
            0,
            0,
            FONT_CHARSET(1),
            OUT_DEFAULT_PRECIS,
            FONT_CLIP_PRECISION(0),
            FONT_QUALITY(4),
            0,
            PCWSTR::from_raw(face_name.as_ptr()),
        );
        if font.is_invalid() {
            cleanup_dc(mem_dc, screen_dc, color_bitmap, HBITMAP::default());
            return Err("CreateFontW failed".into());
        }

        let old_font = SelectObject(mem_dc, font);
        SetBkMode(mem_dc, TRANSPARENT);
        SetTextColor(mem_dc, COLORREF(0x00FF_FFFF));

        let mut text = windows_string(label);
        let mut rect = RECT {
            left: 0,
            top: 0,
            right: SIZE,
            bottom: SIZE,
        };
        DrawTextW(
            mem_dc,
            &mut text,
            &mut rect,
            DT_CENTER | DT_VCENTER | DT_SINGLELINE,
        );

        set_opaque_alpha(bits, SIZE, SIZE);

        SelectObject(mem_dc, old_font);
        let _ = DeleteObject(font);
        SelectObject(mem_dc, old_bitmap);

        let mask_bitmap = create_mask_from_alpha(bits, SIZE, SIZE)?;
        let icon_info = ICONINFO {
            fIcon: BOOL(1),
            xHotspot: 0,
            yHotspot: 0,
            hbmMask: mask_bitmap,
            hbmColor: color_bitmap,
        };

        let icon = CreateIconIndirect(&icon_info)
            .map_err(|e| format!("CreateIconIndirect failed: {e}"))?;

        let _ = DeleteObject(mask_bitmap);
        let _ = DeleteObject(color_bitmap);
        let _ = DeleteDC(mem_dc);
        ReleaseDC(None, screen_dc);

        Ok(icon)
    }
}

unsafe fn set_opaque_alpha(bits: *mut core::ffi::c_void, width: i32, height: i32) {
    let pixels = std::slice::from_raw_parts_mut(bits as *mut u8, (width * height * 4) as usize);
    for chunk in pixels.chunks_exact_mut(4) {
        if chunk[0] != 0 || chunk[1] != 0 || chunk[2] != 0 {
            chunk[3] = 255;
        }
    }
}

unsafe fn create_mask_from_alpha(
    bits: *mut core::ffi::c_void,
    width: i32,
    height: i32,
) -> Result<HBITMAP, String> {
    let pixels = std::slice::from_raw_parts(bits as *const u8, (width * height * 4) as usize);
    let row_stride = ((width + 15) / 16) * 2;
    let mut mask = vec![0u8; (row_stride * height) as usize];

    for y in 0..height {
        for x in 0..width {
            let alpha = pixels[((y * width + x) * 4 + 3) as usize];
            if alpha < 128 {
                let byte_index = (y * row_stride + (x / 8)) as usize;
                mask[byte_index] |= 0x80 >> (x % 8);
            }
        }
    }

    let mask_bitmap = CreateBitmap(width, height, 1, 1, Some(mask.as_ptr() as *const _));
    if mask_bitmap.is_invalid() {
        return Err("CreateBitmap(mask) failed".into());
    }

    Ok(mask_bitmap)
}

unsafe fn cleanup_dc(mem_dc: HDC, screen_dc: HDC, color_bitmap: HBITMAP, mask_bitmap: HBITMAP) {
    if !color_bitmap.is_invalid() {
        let _ = DeleteObject(color_bitmap);
    }
    if !mask_bitmap.is_invalid() {
        let _ = DeleteObject(mask_bitmap);
    }
    if !mem_dc.is_invalid() {
        let _ = DeleteDC(mem_dc);
    }
    if !screen_dc.is_invalid() {
        ReleaseDC(None, screen_dc);
    }
}

fn windows_string(value: &str) -> Vec<u16> {
    value.encode_utf16().chain(std::iter::once(0)).collect()
}

pub fn request_badge_permission() -> Result<bool, String> {
    Ok(true)
}

pub fn is_badge_permission_granted() -> Result<bool, String> {
    Ok(true)
}
