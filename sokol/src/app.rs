//! sokol::app - cross-platform application wrapper
//!
//! A Rust API to the [sokol_app.h](https://github.com/floooh/sokol/blob/master/sokol_app.h)
//! header-only C library.

use std::os::raw::c_void;

pub mod ffi {
    use std::ffi::CStr;
    use std::ffi::CString;
    use std::os::raw::c_char;
    use std::os::raw::c_int;
    use std::os::raw::c_void;
    use std::ptr::null;
    use std::slice::from_raw_parts_mut;

    pub const SAPP_MAX_TOUCHPOINTS: usize = 8;
    pub const SAPP_MAX_MOUSEBUTTONS: usize = 3;
    const _SAPP_MAX_KEYCODES: usize = 512;
    pub const SAPP_MAX_ICONIMAGES: usize = 8;

    #[repr(C)]
    #[derive(Copy, Clone, Debug)]
    pub struct SAppEvent {
        frame_count: u64,
        event_type: super::SAppEventType,
        key_code: super::SAppKeycode,
        char_code: u32,
        key_repeat: bool,
        modifiers: super::SAppModifier,
        mouse_button: super::SAppMouseButton,
        mouse_x: f32,
        mouse_y: f32,
        scroll_x: f32,
        scroll_y: f32,
        num_touches: c_int,
        touches: [super::SAppTouchPoint; SAPP_MAX_TOUCHPOINTS],
        window_width: c_int,
        window_height: c_int,
        framebuffer_width: c_int,
        framebuffer_height: c_int,
    }

    impl SAppEvent {
        pub fn translate(event: &super::SAppEvent) -> Self {
            SAppEvent {
                frame_count: event.frame_count,
                event_type: event.event_type,
                key_code: event.key_code,
                char_code: event.char_code,
                key_repeat: event.key_repeat,
                modifiers: event.modifiers,
                mouse_button: event.mouse_button,
                mouse_x: event.mouse_x,
                mouse_y: event.mouse_y,
                scroll_x: event.scroll_x,
                scroll_y: event.scroll_y,
                num_touches: event.num_touches,
                touches: event.touches,
                window_width: event.window_width,
                window_height: event.window_height,
                framebuffer_width: event.framebuffer_width,
                framebuffer_height: event.framebuffer_height,
            }
        }
    }

    #[repr(C)]
    #[derive(Debug)]
    pub struct SAppLogger {
      func: unsafe extern fn(),
      user_data: *const c_void,
    }
    impl Default for SAppLogger {
      fn default() -> Self {
        SAppLogger {
          func: slog_func,
          user_data: null(),
        }
      }
    }
    
    #[repr(C)]
    #[derive(Debug)]
    pub struct SAppAllocator {
      alloc: *const c_void,
      free: *const c_void,
      user_data: *const c_void,
    }

    impl Default for SAppAllocator {
      fn default() -> Self {
        SAppAllocator {
          alloc: null(),
          free: null(),
          user_data: null(),
        }
      }
    }

    #[repr(C)]
    #[derive(Debug)]
    pub struct SAppDesc {
        init_cb: *const c_void,
        frame_cb: *const c_void,
        cleanup_cb: *const c_void,
        event_cb: *const c_void,

        user_data: *mut c_void,
        init_userdata_cb: extern fn(*mut c_void),
        frame_userdata_cb: extern fn(*mut c_void),
        cleanup_userdata_cb: extern fn(*mut c_void),
        event_userdata_cb: extern fn(*const SAppEvent, *mut c_void),

        width: c_int,
        height: c_int,
        sample_count: c_int,
        swap_interval: c_int,
        high_dpi: bool,
        fullscreen: bool,
        alpha: bool,
        window_title: *const c_char,
        enable_clipboard: bool,
        clipboard_size: c_int,
        enable_dragndrop: bool,
        max_dropped_files: c_int,
        max_dropped_file_path_length: c_int,
        icon: super::SAppIconDesc,
        allocator: SAppAllocator,
        logger: SAppLogger,

        gl_force_gles2: bool,
        gl_major_version: c_int,
        gl_minor_version: c_int,
        win32_console_utf8: bool,
        win32_console_create: bool,
        win32_console_attach: bool,
        html5_canvas_name: *const c_char,
        html5_canvas_resize: bool,
        html5_preserve_drawing_buffer: bool,
        html5_premultiplied_alpha: bool,
        html5_ask_leave_site: bool,
        ios_keyboard_resizes_canvas: bool,
    }

    extern {
        fn slog_func();

        /// sokol entry point (compiled with SOKOL_NO_ENTRY)
        pub fn sapp_run(desc: *const SAppDesc) -> c_int;

        pub fn sapp_isvalid() -> bool;
        pub fn sapp_width() -> c_int;
        pub fn sapp_height() -> c_int;
        pub fn sapp_high_dpi() -> bool;
        pub fn sapp_dpi_scale() -> f32;
        pub fn sapp_show_keyboard(visible: bool);
        pub fn sapp_keyboard_shown() -> bool;
        pub fn sapp_userdata() -> *mut c_void;
        pub fn sapp_request_quit();
        pub fn sapp_cancel_quit();
        pub fn sapp_quit();
        pub fn sapp_frame_count() -> u64;
        pub fn sapp_frame_duration() -> f64;

        pub fn sapp_gles2() -> bool;

        pub fn sapp_metal_get_device() -> *const c_void;
        pub fn sapp_metal_get_renderpass_descriptor() -> *const c_void;
        pub fn sapp_metal_get_drawable() -> *const c_void;
        pub fn sapp_macos_get_window() -> *const c_void;
        pub fn sapp_ios_get_window() -> *const c_void;

        pub fn sapp_d3d11_get_device() -> *const c_void;
        pub fn sapp_d3d11_get_device_context() -> *const c_void;
        pub fn sapp_d3d11_get_render_target_view() -> *const c_void;
        pub fn sapp_d3d11_get_depth_stencil_view() -> *const c_void;
        pub fn sapp_win32_get_hwnd() -> *const c_void;
    }

    pub fn sapp_make_desc(app: &super::SAppImpl) -> SAppDesc {
        let app_ptr = app as *const super::SAppImpl;
        let desc = &app.desc;

        let window_title = CString::new(&*desc.window_title).unwrap();
        let canvas_name = CString::new(&*desc.html5_canvas_name).unwrap();

        SAppDesc {
            init_cb: null(),
            frame_cb: null(),
            cleanup_cb: null(),
            event_cb: null(),

            user_data: app_ptr as *mut c_void,
            init_userdata_cb,
            frame_userdata_cb,
            cleanup_userdata_cb,
            event_userdata_cb,

            width: desc.width,
            height: desc.height,
            sample_count: desc.sample_count,
            swap_interval: desc.swap_interval,
            high_dpi: desc.high_dpi,
            fullscreen: desc.fullscreen,
            alpha: desc.alpha,
            window_title: window_title.into_raw(),
            enable_clipboard: desc.enable_clipboard,
            clipboard_size: desc.clipboard_size,
            enable_dragndrop: desc.enable_dragndrop,
            max_dropped_files: desc.max_dropped_files,
            max_dropped_file_path_length: desc.max_dropped_file_path_length,
            icon: desc.icon,
            allocator: Default::default(),
            logger: Default::default(),

            gl_force_gles2: desc.gl_force_gles2,
            gl_major_version: desc.gl_major_version,
            gl_minor_version: desc.gl_minor_version,
            win32_console_utf8: desc.win32_console_utf8,
            win32_console_create: desc.win32_console_create,
            win32_console_attach: desc.win32_console_attach,
            html5_canvas_name: canvas_name.into_raw(),
            html5_canvas_resize: desc.html5_canvas_resize,
            html5_preserve_drawing_buffer: desc.html5_preserve_drawing_buffer,
            html5_premultiplied_alpha: desc.html5_premultiplied_alpha,
            html5_ask_leave_site: desc.html5_ask_leave_site,
            ios_keyboard_resizes_canvas: desc.ios_keyboard_resizes_canvas,
        }
    }

    #[no_mangle]
    pub extern fn init_userdata_cb(user_data: *mut c_void) {
        super::SAppImpl::get(user_data).init_cb();
    }

    #[no_mangle]
    pub extern fn frame_userdata_cb(user_data: *mut c_void) {
        super::SAppImpl::get(user_data).frame_cb();
    }

    #[no_mangle]
    pub extern fn cleanup_userdata_cb(user_data: *mut c_void) {
        super::SAppImpl::get(user_data).cleanup_cb();
    }

    #[no_mangle]
    pub extern fn event_userdata_cb(event: *const SAppEvent, user_data: *mut c_void) {
        let e = *unsafe {
            &*event
        };

        super::SAppImpl::get(user_data).event_cb(super::SAppEvent {
            frame_count: e.frame_count,
            event_type: e.event_type,
            key_code: e.key_code,
            char_code: e.char_code,
            key_repeat: e.key_repeat,
            modifiers: e.modifiers,
            mouse_button: e.mouse_button,
            mouse_x: e.mouse_x,
            mouse_y: e.mouse_y,
            scroll_x: e.scroll_x,
            scroll_y: e.scroll_y,
            num_touches: e.num_touches,
            touches: e.touches,
            window_width: e.window_width,
            window_height: e.window_height,
            framebuffer_width: e.framebuffer_width,
            framebuffer_height: e.framebuffer_height,
        });
    }

    #[no_mangle]
    pub extern fn fail_userdata_cb(message: *const c_char, user_data: *mut c_void) {
        let msg = unsafe {
            CStr::from_ptr(message)
        };

        super::SAppImpl::get(user_data).fail_cb(msg.to_str().unwrap());
    }

    #[no_mangle]
    pub extern fn stream_userdata_cb(buffer: *mut f32, num_frames: c_int, num_channels: c_int, user_data: *mut c_void) {
        let arr = unsafe {
            let len = num_frames * num_channels;
            from_raw_parts_mut(buffer, len as usize)
        };

        super::SAppImpl::get(user_data).stream_cb(arr, num_frames, num_channels);
    }
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum SAppEventType {
    Invalid,
    KeyDown,
    KeyUp,
    Char,
    MouseDown,
    MouseUp,
    MouseScroll,
    MouseMove,
    MouseEnter,
    MouseLeave,
    TouchesBegan,
    TouchesMoved,
    TouchesEnded,
    TouchesCancelled,
    Resized,
    Iconified,
    Restored,
    Suspended,
    Resumed,
    UpdateCursor,
    QuitRequested,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum SAppKeycode {
    KeyInvalid = 0,
    KeySpace = 32,
    KeyApostrophe = 39,
    KeyComma = 44,
    KeyMinus = 45,
    KeyPeriod = 46,
    KeySlash = 47,
    Key0 = 48,
    Key1 = 49,
    Key2 = 50,
    Key3 = 51,
    Key4 = 52,
    Key5 = 53,
    Key6 = 54,
    Key7 = 55,
    Key8 = 56,
    Key9 = 57,
    KeySemicolon = 59,
    KeyEqual = 61,
    KeyA = 65,
    KeyB = 66,
    KeyC = 67,
    KeyD = 68,
    KeyE = 69,
    KeyF = 70,
    KeyG = 71,
    KeyH = 72,
    KeyI = 73,
    KeyJ = 74,
    KeyK = 75,
    KeyL = 76,
    KeyM = 77,
    KeyN = 78,
    KeyO = 79,
    KeyP = 80,
    KeyQ = 81,
    KeyR = 82,
    KeyS = 83,
    KeyT = 84,
    KeyU = 85,
    KeyV = 86,
    KeyW = 87,
    KeyX = 88,
    KeyY = 89,
    KeyZ = 90,
    KeyLeftBracket = 91,
    KeyBackslash = 92,
    KeyRightBracket = 93,
    KeyGraveAccent = 96,
    KeyWorld1 = 161,
    KeyWorld2 = 162,
    KeyEscape = 256,
    KeyEnter = 257,
    KeyTab = 258,
    KeyBackspace = 259,
    KeyInsert = 260,
    KeyDelete = 261,
    KeyRight = 262,
    KeyLeft = 263,
    KeyDown = 264,
    KeyUp = 265,
    KeyPageUp = 266,
    KeyPageDown = 267,
    KeyHome = 268,
    KeyEnd = 269,
    KeyCapsLock = 280,
    KeyScrollLock = 281,
    KeyNumLock = 282,
    KeyPrintScreen = 283,
    KeyPause = 284,
    KeyF1 = 290,
    KeyF2 = 291,
    KeyF3 = 292,
    KeyF4 = 293,
    KeyF5 = 294,
    KeyF6 = 295,
    KeyF7 = 296,
    KeyF8 = 297,
    KeyF9 = 298,
    KeyF10 = 299,
    KeyF11 = 300,
    KeyF12 = 301,
    KeyF13 = 302,
    KeyF14 = 303,
    KeyF15 = 304,
    KeyF16 = 305,
    KeyF17 = 306,
    KeyF18 = 307,
    KeyF19 = 308,
    KeyF20 = 309,
    KeyF21 = 310,
    KeyF22 = 311,
    KeyF23 = 312,
    KeyF24 = 313,
    KeyF25 = 314,
    KeyKP0 = 320,
    KeyKP1 = 321,
    KeyKP2 = 322,
    KeyKP3 = 323,
    KeyKP4 = 324,
    KeyKP5 = 325,
    KeyKP6 = 326,
    KeyKP7 = 327,
    KeyKP8 = 328,
    KeyKP9 = 329,
    KeyKPDecimal = 330,
    KeyKPDivide = 331,
    KeyKPMultiply = 332,
    KeyKPSubtract = 333,
    KeyKPAdd = 334,
    KeyKPEnter = 335,
    KeyKPEqual = 336,
    KeyLeftShift = 340,
    KeyLeftControl = 341,
    KeyLeftAlt = 342,
    KeyLeftSuper = 343,
    KeyRightShift = 344,
    KeyRightControl = 345,
    KeyRightAlt = 346,
    KeyRightSuper = 347,
    KeyMenu = 348,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum SAppMouseButton {
    Invalid = -1,
    Left = 0,
    Right = 1,
    Middle = 2,
}

bitflags! {
    #[repr(C)]
    pub struct SAppModifier: u32 {
        const SHIFT = 0x01;
        const CONTROL = 0x02;
        const ALT = 0x04;
        const SUPER = 0x08;
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct SAppTouchPoint {
    pub identifier: usize,
    pub pos_x: f32,
    pub pos_y: f32,
    pub changed: bool,
}

#[derive(Debug)]
pub struct SAppEvent {
    pub frame_count: u64,
    pub event_type: SAppEventType,
    pub key_code: SAppKeycode,
    pub char_code: u32,
    pub key_repeat: bool,
    pub modifiers: SAppModifier,
    pub mouse_button: SAppMouseButton,
    pub mouse_x: f32,
    pub mouse_y: f32,
    pub scroll_x: f32,
    pub scroll_y: f32,
    pub num_touches: i32,
    pub touches: [SAppTouchPoint; ffi::SAPP_MAX_TOUCHPOINTS],
    pub window_width: i32,
    pub window_height: i32,
    pub framebuffer_width: i32,
    pub framebuffer_height: i32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct SAppRange {
  pub ptr: *const c_void,
  pub size: usize,
}

impl Default for SAppRange {
  fn default() -> Self {
    SAppRange {
      ptr: std::ptr::null(),
      size: 0,
    }
  }
}

#[repr(C)]
#[derive(Default, Clone, Copy, Debug)]
pub struct SAppImageDesc {
  pub width: i32,
  pub height: i32,
  pub pixels: SAppRange,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SAppIconDesc {
  pub sokol_default: bool,
  pub images: [SAppImageDesc; ffi::SAPP_MAX_ICONIMAGES],
}

impl Default for SAppIconDesc {
  fn default() -> Self {
    return SAppIconDesc {
      sokol_default: true,
      images: [Default::default(); ffi::SAPP_MAX_ICONIMAGES],
    };
  }
}

#[derive(Default, Debug)]
pub struct SAppDesc {
    pub width: i32,
    pub height: i32,
    pub sample_count: i32,
    pub swap_interval: i32,
    pub high_dpi: bool,
    pub fullscreen: bool,
    pub alpha: bool,
    pub window_title: String,
    pub enable_clipboard: bool,
    pub clipboard_size: i32,
    pub enable_dragndrop: bool,
    pub max_dropped_files: i32,
    pub max_dropped_file_path_length: i32,
    pub icon: SAppIconDesc,

    pub gl_force_gles2: bool,
    pub gl_major_version: i32,
    pub gl_minor_version: i32,
    pub win32_console_utf8: bool,
    pub win32_console_create: bool,
    pub win32_console_attach: bool,
    pub html5_canvas_name: String,
    pub html5_canvas_resize: bool,
    pub html5_preserve_drawing_buffer: bool,
    pub html5_premultiplied_alpha: bool,
    pub html5_ask_leave_site: bool,
    pub ios_keyboard_resizes_canvas: bool,
}

pub trait SApp {
    /// Init callback function.
    fn sapp_init(&mut self);

    /// Frame callback function.
    fn sapp_frame(&mut self);

    /// Cleanup callback function.
    fn sapp_cleanup(&mut self);

    /// Event callback function.
    fn sapp_event(&mut self, event: SAppEvent);

    /// Optional `sokol_app` error reporting callback function.
    fn sapp_fail(&mut self, msg: &str) {
        print!("{}", msg);
    }

    /// Function called by `sokol_audio` in callback mode.
    ///
    /// The default implementation clears the buffer to zero. Applications
    /// using this mode are expected to mix audio data into the buffer.
    ///
    /// This is called from a separate thread on all desktop platforms.
    fn saudio_stream(&mut self, buffer: &mut [f32], num_frames: i32, num_channels: i32) {
        let len = (num_frames * num_channels) as usize;
        for i in 0..len {
            buffer[i] = 0.0;
        }
    }
}

pub struct SAppImpl {
    callbacks: Box<dyn SApp>,
    desc: SAppDesc,
}

impl SAppImpl {
    fn new<S: SApp + 'static>(callbacks: S, desc: SAppDesc) -> SAppImpl {
        SAppImpl {
            callbacks: Box::new(callbacks),
            desc,
        }
    }

    pub fn init_cb(&mut self) {
        self.callbacks.sapp_init();
    }

    pub fn frame_cb(&mut self) {
        self.callbacks.sapp_frame();
    }

    pub fn cleanup_cb(&mut self) {
        self.callbacks.sapp_cleanup();
    }

    pub fn event_cb(&mut self, event: SAppEvent) {
        self.callbacks.sapp_event(event);
    }

    pub fn fail_cb(&mut self, msg: &str) {
        self.callbacks.sapp_fail(msg);
    }

    pub fn stream_cb(&mut self, buffer: &mut [f32], num_frames: i32, num_channels: i32) {
        self.callbacks.saudio_stream(buffer, num_frames, num_channels);
    }

    pub fn get(user_data: *mut c_void) -> &'static mut SAppImpl {
        unsafe {
            let app_ptr = user_data as *mut SAppImpl;
            &mut *app_ptr
        }
    }
}

pub fn sapp_run<S: SApp + 'static>(callbacks: S,
                                   desc: SAppDesc) -> i32 {
    let app = SAppImpl::new(callbacks, desc);

    unsafe {
        ffi::sapp_run(&ffi::sapp_make_desc(&app))
    }
}

pub fn sapp_isvalid() -> bool {
    unsafe {
        ffi::sapp_isvalid()
    }
}

pub fn sapp_width() -> i32 {
    unsafe {
        ffi::sapp_width()
    }
}

pub fn sapp_height() -> i32 {
    unsafe {
        ffi::sapp_height()
    }
}

pub fn sapp_high_dpi() -> bool {
    unsafe {
        ffi::sapp_high_dpi()
    }
}

pub fn sapp_dpi_scale() -> f32 {
    unsafe {
        ffi::sapp_dpi_scale()
    }
}

pub fn sapp_show_keyboard(visible: bool) {
    unsafe {
        ffi::sapp_show_keyboard(visible);
    }
}

pub fn sapp_keyboard_shown() -> bool {
    unsafe {
        ffi::sapp_keyboard_shown()
    }
}

pub fn sapp_request_quit() {
    unsafe {
        ffi::sapp_request_quit();
    }
}

pub fn sapp_cancel_quit() {
    unsafe {
        ffi::sapp_cancel_quit();
    }
}

pub fn sapp_quit() {
    unsafe {
        ffi::sapp_quit();
    }
}

pub fn sapp_frame_count() -> u64 {
    unsafe {
        ffi::sapp_frame_count()
    }
}

pub fn sapp_frame_duration() -> f64 {
    unsafe {
        ffi::sapp_frame_duration()
    }
}

pub fn sapp_gles2() -> bool {
    unsafe {
        ffi::sapp_gles2()
    }
}
