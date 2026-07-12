use std::sync::{Mutex, Once, OnceLock};

use windows::Win32::Foundation::{COLORREF, HWND, RECT};
use windows::Win32::Graphics::Gdi::{
    ANTIALIASED_QUALITY, BI_RGB, BITMAPINFO, BITMAPINFOHEADER, CreateBitmap, CreateCompatibleDC,
    CreateDIBSection, CreateFontW, DIB_RGB_COLORS, DT_CALCRECT, DT_NOCLIP, DT_SINGLELINE, DeleteDC,
    DeleteObject, DrawTextW, FONT_CHARSET, FONT_CLIP_PRECISION, FW_BOLD, GetDC, GetDeviceCaps,
    HBITMAP, HDC, LOGPIXELSX, OUT_DEFAULT_PRECIS, ReleaseDC, SelectObject, SetBkMode, SetTextColor,
    TRANSPARENT,
};
use windows::Win32::System::Com::{
    CLSCTX_INPROC_SERVER, COINIT_APARTMENTTHREADED, CoCreateInstance, CoInitializeEx,
};
use windows::Win32::UI::HiDpi::{GetDpiForWindow, GetSystemMetricsForDpi};
use windows::Win32::UI::Input::KeyboardAndMouse::GetActiveWindow;
use windows::Win32::UI::Shell::{ITaskbarList3, TaskbarList};
use windows::Win32::UI::WindowsAndMessaging::{
    CreateIconIndirect, DestroyIcon, HICON, ICONINFO, SM_CXSMICON,
};
use windows::core::{AgileReference, PCWSTR};

struct TaskbarState {
    taskbar: AgileReference<ITaskbarList3>,
    overlay_icon: Option<HICON>,
}

// HICON is a GDI handle; access is serialized via Mutex.
unsafe impl Send for TaskbarState {}

static INIT: Once = Once::new();
static TASKBAR_STATE: OnceLock<Mutex<TaskbarState>> = OnceLock::new();
static INIT_ERROR: OnceLock<String> = OnceLock::new();

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
    let icon = create_badge_icon(hwnd, &label)?;
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
    INIT.call_once(|| match create_taskbar_state() {
        Ok(state) => {
            let _ = TASKBAR_STATE.set(Mutex::new(state));
        }
        Err(e) => {
            let _ = INIT_ERROR.set(e);
        }
    });

    if let Some(err) = INIT_ERROR.get() {
        return Err(err.clone());
    }

    TASKBAR_STATE
        .get()
        .ok_or_else(|| "Taskbar init failed".into())
}

fn create_taskbar_state() -> Result<TaskbarState, String> {
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

    let taskbar = AgileReference::new(&taskbar)
        .map_err(|e| format!("AgileReference::new failed: {e}"))?;

    Ok(TaskbarState {
        taskbar,
        overlay_icon: None,
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

    let taskbar = guard
        .taskbar
        .resolve()
        .map_err(|e| format!("AgileReference::resolve failed: {e}"))?;

    unsafe {
        taskbar
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

    let taskbar = guard
        .taskbar
        .resolve()
        .map_err(|e| format!("AgileReference::resolve failed: {e}"))?;

    let description = windows_string(description);
    unsafe {
        taskbar
            .SetOverlayIcon(hwnd, icon, PCWSTR::from_raw(description.as_ptr()))
            .map_err(|e| format!("SetOverlayIcon failed: {e}"))?;
    }

    Ok(())
}

fn create_badge_icon(hwnd: HWND, label: &str) -> Result<HICON, String> {
    unsafe {
        let screen_dc = GetDC(None);
        if screen_dc.is_invalid() {
            return Err("GetDC failed".into());
        }

        let mut dpi = GetDpiForWindow(hwnd);
        if dpi == 0 {
            dpi = GetDeviceCaps(Some(screen_dc), LOGPIXELSX) as u32;
        }
        if dpi == 0 {
            dpi = 96;
        }

        // Prefer the system small-icon metric; never go below the classic 16@96dpi scale.
        let mut size = GetSystemMetricsForDpi(SM_CXSMICON, dpi);
        let dpi_size = (16 * dpi as i32 / 96).max(16);
        if size < dpi_size {
            size = dpi_size;
        }
        // Upscaling looks blocky; prefer a denser bitmap Windows can downscale cleanly.
        size = size.max(32);

        // 2x supersample, then box-filter down for sharper edges/glyphs.
        const SCALE: i32 = 2;
        let render = size * SCALE;
        let font_h = (render * 11 / 16).max(12);

        let mem_dc = CreateCompatibleDC(Some(screen_dc));
        if mem_dc.is_invalid() {
            ReleaseDC(None, screen_dc);
            return Err("CreateCompatibleDC failed".into());
        }

        let hi_bmi = BITMAPINFO {
            bmiHeader: BITMAPINFOHEADER {
                biSize: std::mem::size_of::<BITMAPINFOHEADER>() as u32,
                biWidth: render,
                biHeight: -render,
                biPlanes: 1,
                biBitCount: 32,
                biCompression: BI_RGB.0,
                ..Default::default()
            },
            ..Default::default()
        };

        let mut hi_bits: *mut core::ffi::c_void = std::ptr::null_mut();
        let hi_bitmap =
            CreateDIBSection(Some(mem_dc), &hi_bmi, DIB_RGB_COLORS, &mut hi_bits, None, 0)
                .map_err(|e| format!("CreateDIBSection(hi) failed: {e}"))?;

        if hi_bits.is_null() {
            cleanup_dc(mem_dc, screen_dc, hi_bitmap, HBITMAP::default());
            return Err("CreateDIBSection(hi) returned null bits".into());
        }

        let hi_count = (render * render) as usize;

        let old_bitmap = SelectObject(mem_dc, hi_bitmap.into());

        let face_name = windows_string("Segoe UI");
        let font = CreateFontW(
            font_h,
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
            ANTIALIASED_QUALITY,
            0,
            PCWSTR::from_raw(face_name.as_ptr()),
        );
        if font.is_invalid() {
            SelectObject(mem_dc, old_bitmap);
            cleanup_dc(mem_dc, screen_dc, hi_bitmap, HBITMAP::default());
            return Err("CreateFontW failed".into());
        }

        let old_font = SelectObject(mem_dc, font.into());
        SetBkMode(mem_dc, TRANSPARENT);
        SetTextColor(mem_dc, COLORREF(0x00FF_FFFF));

        // DrawTextW uses slice len as cch — do not include a trailing NUL.
        let mut text: Vec<u16> = label.encode_utf16().collect();

        // Probe-draw on a cleared buffer to find the real ink bounding box.
        std::ptr::write_bytes(hi_bits as *mut u8, 0, hi_count * 4);
        let mut calc = RECT {
            left: 0,
            top: 0,
            right: 0,
            bottom: 0,
        };
        DrawTextW(mem_dc, &mut text, &mut calc, DT_CALCRECT | DT_SINGLELINE);
        let text_w = (calc.right - calc.left).max(1);
        let text_h = (calc.bottom - calc.top).max(1);
        let mut probe = RECT {
            left: 0,
            top: 0,
            right: text_w,
            bottom: text_h,
        };
        DrawTextW(mem_dc, &mut text, &mut probe, DT_SINGLELINE | DT_NOCLIP);

        let (ink_min_x, ink_min_y, ink_max_x, ink_max_y) =
            ink_bounds(hi_bits, render).unwrap_or((0, 0, text_w - 1, text_h - 1));
        let ink_cx = (ink_min_x + ink_max_x) as f32 / 2.0;
        let ink_cy = (ink_min_y + ink_max_y) as f32 / 2.0;
        let canvas_c = (render - 1) as f32 / 2.0;
        let dx = (canvas_c - ink_cx).round() as i32;
        let dy = (canvas_c - ink_cy).round() as i32;

        // Final compose: circle, then text centered by ink.
        std::ptr::write_bytes(hi_bits as *mut u8, 0, hi_count * 4);
        draw_soft_circle(hi_bits, render);
        let alpha_snapshot: Vec<u8> = {
            let pixels = std::slice::from_raw_parts(hi_bits as *const u8, hi_count * 4);
            (0..hi_count).map(|i| pixels[i * 4 + 3]).collect()
        };

        let mut rect = RECT {
            left: dx,
            top: dy,
            right: dx + text_w,
            bottom: dy + text_h,
        };
        DrawTextW(mem_dc, &mut text, &mut rect, DT_SINGLELINE | DT_NOCLIP);

        composite_text_over_circle(hi_bits, render, &alpha_snapshot);

        SelectObject(mem_dc, old_font);
        let _ = DeleteObject(font.into());
        SelectObject(mem_dc, old_bitmap);

        let out_bmi = BITMAPINFO {
            bmiHeader: BITMAPINFOHEADER {
                biSize: std::mem::size_of::<BITMAPINFOHEADER>() as u32,
                biWidth: size,
                biHeight: -size,
                biPlanes: 1,
                biBitCount: 32,
                biCompression: BI_RGB.0,
                ..Default::default()
            },
            ..Default::default()
        };

        let mut out_bits: *mut core::ffi::c_void = std::ptr::null_mut();
        let color_bitmap =
            CreateDIBSection(Some(mem_dc), &out_bmi, DIB_RGB_COLORS, &mut out_bits, None, 0)
                .map_err(|e| format!("CreateDIBSection(out) failed: {e}"))?;

        if out_bits.is_null() {
            let _ = DeleteObject(hi_bitmap.into());
            cleanup_dc(mem_dc, screen_dc, color_bitmap, HBITMAP::default());
            return Err("CreateDIBSection(out) returned null bits".into());
        }

        downsample_2x(hi_bits, render, out_bits, size);
        let _ = DeleteObject(hi_bitmap.into());

        let mask_bitmap = create_mask_from_alpha(out_bits, size, size)?;
        let icon_info = ICONINFO {
            fIcon: true.into(),
            xHotspot: 0,
            yHotspot: 0,
            hbmMask: mask_bitmap,
            hbmColor: color_bitmap,
        };

        let icon = CreateIconIndirect(&icon_info)
            .map_err(|e| format!("CreateIconIndirect failed: {e}"))?;

        let _ = DeleteObject(mask_bitmap.into());
        let _ = DeleteObject(color_bitmap.into());
        let _ = DeleteDC(mem_dc);
        ReleaseDC(None, screen_dc);

        Ok(icon)
    }
}

unsafe fn ink_bounds(bits: *mut core::ffi::c_void, size: i32) -> Option<(i32, i32, i32, i32)> {
    let pixels =
        unsafe { std::slice::from_raw_parts(bits as *const u8, (size * size * 4) as usize) };
    let mut min_x = size;
    let mut min_y = size;
    let mut max_x = -1;
    let mut max_y = -1;

    for y in 0..size {
        for x in 0..size {
            let i = ((y * size + x) * 4) as usize;
            // White glyph ink on black: green/blue channels light up.
            if pixels[i + 1] > 20 || pixels[i] > 20 {
                min_x = min_x.min(x);
                min_y = min_y.min(y);
                max_x = max_x.max(x);
                max_y = max_y.max(y);
            }
        }
    }

    if max_x < 0 {
        None
    } else {
        Some((min_x, min_y, max_x, max_y))
    }
}

unsafe fn draw_soft_circle(bits: *mut core::ffi::c_void, size: i32) {
    let pixels =
        unsafe { std::slice::from_raw_parts_mut(bits as *mut u8, (size * size * 4) as usize) };
    let cx = (size as f32 - 1.0) / 2.0;
    let cy = cx;
    // Slightly tighter AA band keeps the badge crisp after downscale.
    let radius = size as f32 / 2.0 - 0.75;

    for y in 0..size {
        for x in 0..size {
            let dx = x as f32 - cx;
            let dy = y as f32 - cy;
            let dist = (dx * dx + dy * dy).sqrt();
            let coverage = (radius + 0.5 - dist).clamp(0.0, 1.0);
            if coverage <= 0.0 {
                continue;
            }
            let i = ((y * size + x) * 4) as usize;
            let alpha = (coverage * 255.0).round() as u8;
            pixels[i] = 0;
            pixels[i + 1] = 0;
            pixels[i + 2] = 255;
            pixels[i + 3] = alpha;
        }
    }
}

unsafe fn composite_text_over_circle(
    bits: *mut core::ffi::c_void,
    size: i32,
    alpha_snapshot: &[u8],
) {
    let pixels =
        unsafe { std::slice::from_raw_parts_mut(bits as *mut u8, (size * size * 4) as usize) };
    let pixel_count = (size * size) as usize;

    for i in 0..pixel_count {
        let base = i * 4;
        let b = pixels[base] as f32;
        let g = pixels[base + 1] as f32;
        let circle_a = alpha_snapshot[i] as f32 / 255.0;

        // DrawTextW writes near-white ink (with AA) over pure red (g≈0,b≈0).
        let text_cov = (g.max(b) / 255.0).clamp(0.0, 1.0);
        if text_cov <= 0.0 {
            pixels[base] = 0;
            pixels[base + 1] = 0;
            pixels[base + 2] = 255;
            pixels[base + 3] = alpha_snapshot[i];
            continue;
        }

        // Premultiplied-style composite: opaque white glyph over red disc.
        let inv = 1.0 - text_cov;
        let out_a = (text_cov + circle_a * inv).clamp(0.0, 1.0);
        let white = 255.0 * text_cov;
        let red = 255.0 * circle_a * inv;
        pixels[base] = white.round() as u8;
        pixels[base + 1] = white.round() as u8;
        pixels[base + 2] = (white + red).round().min(255.0) as u8;
        pixels[base + 3] = (out_a * 255.0).round() as u8;
    }
}

unsafe fn downsample_2x(
    src: *mut core::ffi::c_void,
    src_size: i32,
    dst: *mut core::ffi::c_void,
    dst_size: i32,
) {
    let src_px =
        unsafe { std::slice::from_raw_parts(src as *const u8, (src_size * src_size * 4) as usize) };
    let dst_px =
        unsafe { std::slice::from_raw_parts_mut(dst as *mut u8, (dst_size * dst_size * 4) as usize) };

    for y in 0..dst_size {
        for x in 0..dst_size {
            let mut b = 0u32;
            let mut g = 0u32;
            let mut r = 0u32;
            let mut a = 0u32;
            for oy in 0..2 {
                for ox in 0..2 {
                    let sx = x * 2 + ox;
                    let sy = y * 2 + oy;
                    let i = ((sy * src_size + sx) * 4) as usize;
                    b += src_px[i] as u32;
                    g += src_px[i + 1] as u32;
                    r += src_px[i + 2] as u32;
                    a += src_px[i + 3] as u32;
                }
            }
            let di = ((y * dst_size + x) * 4) as usize;
            dst_px[di] = (b / 4) as u8;
            dst_px[di + 1] = (g / 4) as u8;
            dst_px[di + 2] = (r / 4) as u8;
            dst_px[di + 3] = (a / 4) as u8;
        }
    }
}

unsafe fn create_mask_from_alpha(
    bits: *mut core::ffi::c_void,
    width: i32,
    height: i32,
) -> Result<HBITMAP, String> {
    let pixels = unsafe {
        std::slice::from_raw_parts(bits as *const u8, (width * height * 4) as usize)
    };
    let row_stride = ((width + 15) / 16) * 2;
    let mut mask = vec![0u8; (row_stride * height) as usize];

    // Only fully transparent pixels go into the 1-bit mask; soft edges stay in color alpha.
    for y in 0..height {
        for x in 0..width {
            let alpha = pixels[((y * width + x) * 4 + 3) as usize];
            if alpha == 0 {
                let byte_index = (y * row_stride + (x / 8)) as usize;
                mask[byte_index] |= 0x80 >> (x % 8);
            }
        }
    }

    let mask_bitmap =
        unsafe { CreateBitmap(width, height, 1, 1, Some(mask.as_ptr() as *const _)) };
    if mask_bitmap.is_invalid() {
        return Err("CreateBitmap(mask) failed".into());
    }

    Ok(mask_bitmap)
}

unsafe fn cleanup_dc(mem_dc: HDC, screen_dc: HDC, color_bitmap: HBITMAP, mask_bitmap: HBITMAP) {
    if !color_bitmap.is_invalid() {
        let _ = unsafe { DeleteObject(color_bitmap.into()) };
    }
    if !mask_bitmap.is_invalid() {
        let _ = unsafe { DeleteObject(mask_bitmap.into()) };
    }
    if !mem_dc.is_invalid() {
        let _ = unsafe { DeleteDC(mem_dc) };
    }
    if !screen_dc.is_invalid() {
        unsafe {
            ReleaseDC(None, screen_dc);
        }
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
