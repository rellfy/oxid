#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]

pub mod fs;
mod gl;
mod rand;

pub use gl::*;
pub use rand::*;

struct OxidContext {
    desc: oxid_desc,
    clipboard: Option<String>,
}

impl OxidContext {
    unsafe fn init(desc: oxid_desc) {
        let user_data = desc.user_data;
        OXID_CONTEXT = Some(OxidContext {
            desc,
            clipboard: None,
        });
        OXID_CONTEXT
            .as_mut()
            .unwrap()
            .desc
            .init_userdata_cb
            .unwrap_or_else(|| panic!())(user_data);
    }

    unsafe fn frame(&mut self) {
        let user_data = self.desc.user_data;
        self.desc.frame_userdata_cb.unwrap_or_else(|| panic!())(user_data);
    }

    unsafe fn event(&mut self, event: oxid_event) {
        let user_data = self.desc.user_data;
        self.desc.event_userdata_cb.unwrap_or_else(|| panic!())(&event as *const _, user_data);
    }
}

static mut OXID_CONTEXT: Option<OxidContext> = None;

unsafe fn oxid_context() -> &'static mut OxidContext {
    OXID_CONTEXT.as_mut().unwrap()
}

pub type oxid_event_type = u32;
pub type oxid_mousebutton = i32;
pub type oxid_keycode = u32;

pub const OXID_EVENT_TYPE_INVALID: oxid_event_type = 0;
pub const OXID_EVENT_TYPE_KEY_DOWN: oxid_event_type = 1;
pub const OXID_EVENT_TYPE_KEY_UP: oxid_event_type = 2;
pub const OXID_EVENT_TYPE_CHAR: oxid_event_type = 3;
pub const OXID_EVENT_TYPE_MOUSE_DOWN: oxid_event_type = 4;
pub const OXID_EVENT_TYPE_MOUSE_UP: oxid_event_type = 5;
pub const OXID_EVENT_TYPE_MOUSE_SCROLL: oxid_event_type = 6;
pub const OXID_EVENT_TYPE_MOUSE_MOVE: oxid_event_type = 7;
pub const OXID_EVENT_TYPE_MOUSE_ENTER: oxid_event_type = 8;
pub const OXID_EVENT_TYPE_MOUSE_LEAVE: oxid_event_type = 9;
pub const OXID_EVENT_TYPE_TOUCHES_BEGAN: oxid_event_type = 10;
pub const OXID_EVENT_TYPE_TOUCHES_MOVED: oxid_event_type = 11;
pub const OXID_EVENT_TYPE_TOUCHES_ENDED: oxid_event_type = 12;
pub const OXID_EVENT_TYPE_TOUCHES_CANCELLED: oxid_event_type = 13;
pub const OXID_EVENT_TYPE_RESIZED: oxid_event_type = 14;
pub const OXID_EVENT_TYPE_ICONIFIED: oxid_event_type = 15;
pub const OXID_EVENT_TYPE_RESTORED: oxid_event_type = 16;
pub const OXID_EVENT_TYPE_SUSPENDED: oxid_event_type = 17;
pub const OXID_EVENT_TYPE_RESUMED: oxid_event_type = 18;
pub const OXID_EVENT_TYPE_UPDATE_CURSOR: oxid_event_type = 19;
pub const OXID_EVENT_TYPE_QUIT_REQUESTED: oxid_event_type = 20;
pub const OXID_EVENT_TYPE_RAW_DEVICE: oxid_event_type = 21;
pub const OXID_EVENT_TYPE_NUM: oxid_event_type = 22;
pub const OXID_EVENT_TYPE_FORCE_U32: oxid_event_type = 2147483647;

pub const OXID_KEYCODE_INVALID: oxid_keycode = 0;
pub const OXID_KEYCODE_SPACE: oxid_keycode = 32;
pub const OXID_KEYCODE_APOSTROPHE: oxid_keycode = 39;
pub const OXID_KEYCODE_COMMA: oxid_keycode = 44;
pub const OXID_KEYCODE_MINUS: oxid_keycode = 45;
pub const OXID_KEYCODE_PERIOD: oxid_keycode = 46;
pub const OXID_KEYCODE_SLASH: oxid_keycode = 47;
pub const OXID_KEYCODE_0: oxid_keycode = 48;
pub const OXID_KEYCODE_1: oxid_keycode = 49;
pub const OXID_KEYCODE_2: oxid_keycode = 50;
pub const OXID_KEYCODE_3: oxid_keycode = 51;
pub const OXID_KEYCODE_4: oxid_keycode = 52;
pub const OXID_KEYCODE_5: oxid_keycode = 53;
pub const OXID_KEYCODE_6: oxid_keycode = 54;
pub const OXID_KEYCODE_7: oxid_keycode = 55;
pub const OXID_KEYCODE_8: oxid_keycode = 56;
pub const OXID_KEYCODE_9: oxid_keycode = 57;
pub const OXID_KEYCODE_SEMICOLON: oxid_keycode = 59;
pub const OXID_KEYCODE_EQUAL: oxid_keycode = 61;
pub const OXID_KEYCODE_A: oxid_keycode = 65;
pub const OXID_KEYCODE_B: oxid_keycode = 66;
pub const OXID_KEYCODE_C: oxid_keycode = 67;
pub const OXID_KEYCODE_D: oxid_keycode = 68;
pub const OXID_KEYCODE_E: oxid_keycode = 69;
pub const OXID_KEYCODE_F: oxid_keycode = 70;
pub const OXID_KEYCODE_G: oxid_keycode = 71;
pub const OXID_KEYCODE_H: oxid_keycode = 72;
pub const OXID_KEYCODE_I: oxid_keycode = 73;
pub const OXID_KEYCODE_J: oxid_keycode = 74;
pub const OXID_KEYCODE_K: oxid_keycode = 75;
pub const OXID_KEYCODE_L: oxid_keycode = 76;
pub const OXID_KEYCODE_M: oxid_keycode = 77;
pub const OXID_KEYCODE_N: oxid_keycode = 78;
pub const OXID_KEYCODE_O: oxid_keycode = 79;
pub const OXID_KEYCODE_P: oxid_keycode = 80;
pub const OXID_KEYCODE_Q: oxid_keycode = 81;
pub const OXID_KEYCODE_R: oxid_keycode = 82;
pub const OXID_KEYCODE_S: oxid_keycode = 83;
pub const OXID_KEYCODE_T: oxid_keycode = 84;
pub const OXID_KEYCODE_U: oxid_keycode = 85;
pub const OXID_KEYCODE_V: oxid_keycode = 86;
pub const OXID_KEYCODE_W: oxid_keycode = 87;
pub const OXID_KEYCODE_X: oxid_keycode = 88;
pub const OXID_KEYCODE_Y: oxid_keycode = 89;
pub const OXID_KEYCODE_Z: oxid_keycode = 90;
pub const OXID_KEYCODE_LEFT_BRACKET: oxid_keycode = 91;
pub const OXID_KEYCODE_BACKSLASH: oxid_keycode = 92;
pub const OXID_KEYCODE_RIGHT_BRACKET: oxid_keycode = 93;
pub const OXID_KEYCODE_GRAVE_ACCENT: oxid_keycode = 96;
pub const OXID_KEYCODE_WORLD_1: oxid_keycode = 161;
pub const OXID_KEYCODE_WORLD_2: oxid_keycode = 162;
pub const OXID_KEYCODE_ESCAPE: oxid_keycode = 256;
pub const OXID_KEYCODE_ENTER: oxid_keycode = 257;
pub const OXID_KEYCODE_TAB: oxid_keycode = 258;
pub const OXID_KEYCODE_BACKSPACE: oxid_keycode = 259;
pub const OXID_KEYCODE_INSERT: oxid_keycode = 260;
pub const OXID_KEYCODE_DELETE: oxid_keycode = 261;
pub const OXID_KEYCODE_RIGHT: oxid_keycode = 262;
pub const OXID_KEYCODE_LEFT: oxid_keycode = 263;
pub const OXID_KEYCODE_DOWN: oxid_keycode = 264;
pub const OXID_KEYCODE_UP: oxid_keycode = 265;
pub const OXID_KEYCODE_PAGE_UP: oxid_keycode = 266;
pub const OXID_KEYCODE_PAGE_DOWN: oxid_keycode = 267;
pub const OXID_KEYCODE_HOME: oxid_keycode = 268;
pub const OXID_KEYCODE_END: oxid_keycode = 269;
pub const OXID_KEYCODE_CAPS_LOCK: oxid_keycode = 280;
pub const OXID_KEYCODE_SCROLL_LOCK: oxid_keycode = 281;
pub const OXID_KEYCODE_NUM_LOCK: oxid_keycode = 282;
pub const OXID_KEYCODE_PRINT_SCREEN: oxid_keycode = 283;
pub const OXID_KEYCODE_PAUSE: oxid_keycode = 284;
pub const OXID_KEYCODE_F1: oxid_keycode = 290;
pub const OXID_KEYCODE_F2: oxid_keycode = 291;
pub const OXID_KEYCODE_F3: oxid_keycode = 292;
pub const OXID_KEYCODE_F4: oxid_keycode = 293;
pub const OXID_KEYCODE_F5: oxid_keycode = 294;
pub const OXID_KEYCODE_F6: oxid_keycode = 295;
pub const OXID_KEYCODE_F7: oxid_keycode = 296;
pub const OXID_KEYCODE_F8: oxid_keycode = 297;
pub const OXID_KEYCODE_F9: oxid_keycode = 298;
pub const OXID_KEYCODE_F10: oxid_keycode = 299;
pub const OXID_KEYCODE_F11: oxid_keycode = 300;
pub const OXID_KEYCODE_F12: oxid_keycode = 301;
pub const OXID_KEYCODE_F13: oxid_keycode = 302;
pub const OXID_KEYCODE_F14: oxid_keycode = 303;
pub const OXID_KEYCODE_F15: oxid_keycode = 304;
pub const OXID_KEYCODE_F16: oxid_keycode = 305;
pub const OXID_KEYCODE_F17: oxid_keycode = 306;
pub const OXID_KEYCODE_F18: oxid_keycode = 307;
pub const OXID_KEYCODE_F19: oxid_keycode = 308;
pub const OXID_KEYCODE_F20: oxid_keycode = 309;
pub const OXID_KEYCODE_F21: oxid_keycode = 310;
pub const OXID_KEYCODE_F22: oxid_keycode = 311;
pub const OXID_KEYCODE_F23: oxid_keycode = 312;
pub const OXID_KEYCODE_F24: oxid_keycode = 313;
pub const OXID_KEYCODE_F25: oxid_keycode = 314;
pub const OXID_KEYCODE_KP_0: oxid_keycode = 320;
pub const OXID_KEYCODE_KP_1: oxid_keycode = 321;
pub const OXID_KEYCODE_KP_2: oxid_keycode = 322;
pub const OXID_KEYCODE_KP_3: oxid_keycode = 323;
pub const OXID_KEYCODE_KP_4: oxid_keycode = 324;
pub const OXID_KEYCODE_KP_5: oxid_keycode = 325;
pub const OXID_KEYCODE_KP_6: oxid_keycode = 326;
pub const OXID_KEYCODE_KP_7: oxid_keycode = 327;
pub const OXID_KEYCODE_KP_8: oxid_keycode = 328;
pub const OXID_KEYCODE_KP_9: oxid_keycode = 329;
pub const OXID_KEYCODE_KP_DECIMAL: oxid_keycode = 330;
pub const OXID_KEYCODE_KP_DIVIDE: oxid_keycode = 331;
pub const OXID_KEYCODE_KP_MULTIPLY: oxid_keycode = 332;
pub const OXID_KEYCODE_KP_SUBTRACT: oxid_keycode = 333;
pub const OXID_KEYCODE_KP_ADD: oxid_keycode = 334;
pub const OXID_KEYCODE_KP_ENTER: oxid_keycode = 335;
pub const OXID_KEYCODE_KP_EQUAL: oxid_keycode = 336;
pub const OXID_KEYCODE_LEFT_SHIFT: oxid_keycode = 340;
pub const OXID_KEYCODE_LEFT_CONTROL: oxid_keycode = 341;
pub const OXID_KEYCODE_LEFT_ALT: oxid_keycode = 342;
pub const OXID_KEYCODE_LEFT_SUPER: oxid_keycode = 343;
pub const OXID_KEYCODE_RIGHT_SHIFT: oxid_keycode = 344;
pub const OXID_KEYCODE_RIGHT_CONTROL: oxid_keycode = 345;
pub const OXID_KEYCODE_RIGHT_ALT: oxid_keycode = 346;
pub const OXID_KEYCODE_RIGHT_SUPER: oxid_keycode = 347;
pub const OXID_KEYCODE_MENU: oxid_keycode = 348;

pub const oxid_mousebutton_OXID_MOUSEBUTTON_INVALID: oxid_mousebutton = -1;
pub const oxid_mousebutton_OXID_MOUSEBUTTON_LEFT: oxid_mousebutton = 0;
pub const oxid_mousebutton_OXID_MOUSEBUTTON_RIGHT: oxid_mousebutton = 1;
pub const oxid_mousebutton_OXID_MOUSEBUTTON_MIDDLE: oxid_mousebutton = 2;

pub const OXID_MODIFIER_SHIFT: u32 = 1 << 0;
pub const OXID_MODIFIER_CTRL: u32 = 1 << 1;
pub const OXID_MODIFIER_ALT: u32 = 1 << 2;
pub const OXID_MODIFIER_SUPER: u32 = 1 << 3;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct oxid_event {
    pub frame_count: u64,
    pub type_: oxid_event_type,
    pub key_code: oxid_keycode,
    pub char_code: u32,
    pub key_repeat: bool,
    pub modifiers: u32,
    pub mouse_button: oxid_mousebutton,
    pub mouse_x: f32,
    pub mouse_y: f32,
    pub mouse_dx: f32,
    pub mouse_dy: f32,
    pub scroll_x: f32,
    pub scroll_y: f32,
    pub num_touches: ::std::os::raw::c_int,
    pub touches: [oxid_touchpoint; 8usize],
    pub window_width: ::std::os::raw::c_int,
    pub window_height: ::std::os::raw::c_int,
    pub framebuffer_width: ::std::os::raw::c_int,
    pub framebuffer_height: ::std::os::raw::c_int,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct oxid_touchpoint {
    pub identifier: usize,
    pub pos_x: f32,
    pub pos_y: f32,
    pub changed: bool,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct oxid_desc {
    pub init_cb: ::std::option::Option<unsafe extern "C" fn()>,
    pub frame_cb: ::std::option::Option<unsafe extern "C" fn()>,
    pub cleanup_cb: ::std::option::Option<unsafe extern "C" fn()>,
    pub event_cb: ::std::option::Option<unsafe extern "C" fn(arg1: *const oxid_event)>,
    pub fail_cb: ::std::option::Option<unsafe extern "C" fn(arg1: *const ::std::os::raw::c_char)>,
    pub user_data: *mut ::std::os::raw::c_void,
    pub init_userdata_cb:
        ::std::option::Option<unsafe extern "C" fn(arg1: *mut ::std::os::raw::c_void)>,
    pub frame_userdata_cb:
        ::std::option::Option<unsafe extern "C" fn(arg1: *mut ::std::os::raw::c_void)>,
    pub cleanup_userdata_cb:
        ::std::option::Option<unsafe extern "C" fn(arg1: *mut ::std::os::raw::c_void)>,
    pub event_userdata_cb: ::std::option::Option<
        unsafe extern "C" fn(arg1: *const oxid_event, arg2: *mut ::std::os::raw::c_void),
    >,
    pub fail_userdata_cb: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: *const ::std::os::raw::c_char,
            arg2: *mut ::std::os::raw::c_void,
        ),
    >,
    pub width: ::std::os::raw::c_int,
    pub height: ::std::os::raw::c_int,
    pub sample_count: ::std::os::raw::c_int,
    pub swap_interval: ::std::os::raw::c_int,
    pub high_dpi: bool,
    pub fullscreen: bool,
    pub alpha: bool,
    pub window_title: *const ::std::os::raw::c_char,
    pub user_cursor: bool,
    pub html5_canvas_name: *const ::std::os::raw::c_char,
    pub html5_canvas_resize: bool,
    pub html5_preserve_drawing_buffer: bool,
    pub html5_premultiplied_alpha: bool,
    pub html5_ask_leave_site: bool,
    pub ios_keyboard_resizes_canvas: bool,
    pub gl_force_gles2: bool,
}

pub unsafe fn oxid_run(desc: *const oxid_desc) -> ::std::os::raw::c_int {
    {
        use std::ffi::CString;
        use std::panic;

        panic::set_hook(Box::new(|info| {
            let msg = CString::new(format!("{:?}", info)).unwrap_or_else(|_| {
                CString::new(format!("MALFORMED ERROR MESSAGE {:?}", info.location())).unwrap()
            });
            console_log(msg.as_ptr());
        }));
    }

    init_opengl();

    OxidContext::init(*desc);

    0
}

pub unsafe fn oxid_width() -> ::std::os::raw::c_int {
    canvas_width()
}

pub unsafe fn oxid_height() -> ::std::os::raw::c_int {
    canvas_height()
}

#[no_mangle]
extern "C" {
    pub fn init_opengl();
    pub fn canvas_width() -> i32;
    pub fn canvas_height() -> i32;
    pub fn console_debug(msg: *const ::std::os::raw::c_char);
    pub fn console_log(msg: *const ::std::os::raw::c_char);
    pub fn console_info(msg: *const ::std::os::raw::c_char);
    pub fn console_warn(msg: *const ::std::os::raw::c_char);
    pub fn console_error(msg: *const ::std::os::raw::c_char);

    pub fn oxid_set_clipboard(clipboard: *const i8, len: usize);

    /// call "requestPointerLock" and "exitPointerLock" internally.
    /// Will hide cursor and will disable mouse_move events, but instead will
    /// will make inifinite mouse field for raw_device_input event.
    /// Notice that this function will works only from "engaging" event callbacks - from
    /// "mouse_down"/"key_down" event handler functions.
    pub fn oxid_set_cursor_grab(grab: bool);
}

/// Do nothing on wasm - cursor will be hidden by "oxid_set_cursor_grab" anyway.
pub unsafe fn oxid_show_mouse(_shown: bool) {}

pub unsafe fn oxid_high_dpi() -> bool {
    false
}

pub unsafe fn oxid_dpi_scale() -> f32 {
    1.
}

#[no_mangle]
pub extern "C" fn allocate_vec_u8(len: usize) -> *mut u8 {
    let mut string = vec![0u8; len];
    let ptr = string.as_mut_ptr();
    std::mem::forget(string);
    ptr
}

#[no_mangle]
pub extern "C" fn on_clipboard_paste(msg: *mut u8, len: usize) {
    let msg = unsafe { String::from_raw_parts(msg, len, len) };

    unsafe { oxid_context().clipboard = Some(msg) };
}

pub fn clipboard_get() -> Option<String> {
    unsafe { oxid_context().clipboard.clone() }
}

pub fn clipboard_set(data: &str) {
    let len = data.len();
    let data = std::ffi::CString::new(data).unwrap();
    unsafe { oxid_set_clipboard(data.as_ptr(), len) };
}

#[no_mangle]
pub extern "C" fn frame() {
    unsafe {
        oxid_context().frame();
    }
}

#[no_mangle]
pub extern "C" fn mouse_move(x: i32, y: i32) {
    let mut event: oxid_event = unsafe { std::mem::zeroed() };

    event.type_ = OXID_EVENT_TYPE_MOUSE_MOVE;
    event.mouse_x = x as f32;
    event.mouse_y = y as f32;
    unsafe {
        oxid_context().event(event);
    }
}

#[no_mangle]
pub extern "C" fn raw_mouse_move(dx: i32, dy: i32) {
    let mut event: oxid_event = unsafe { std::mem::zeroed() };

    event.type_ = OXID_EVENT_TYPE_RAW_DEVICE;
    event.mouse_dx = dx as f32;
    event.mouse_dy = dy as f32;
    unsafe {
        oxid_context().event(event);
    }
}

#[no_mangle]
pub extern "C" fn mouse_down(x: i32, y: i32, btn: i32) {
    let mut event: oxid_event = unsafe { std::mem::zeroed() };

    event.type_ = OXID_EVENT_TYPE_MOUSE_DOWN;
    event.mouse_button = btn;
    event.mouse_x = x as f32;
    event.mouse_y = y as f32;
    unsafe {
        oxid_context().event(event);
    }
}

#[no_mangle]
pub extern "C" fn mouse_up(x: i32, y: i32, btn: i32) {
    let mut event: oxid_event = unsafe { std::mem::zeroed() };

    event.type_ = OXID_EVENT_TYPE_MOUSE_UP;
    event.mouse_button = btn;
    event.mouse_x = x as f32;
    event.mouse_y = y as f32;
    unsafe {
        oxid_context().event(event);
    }
}

#[no_mangle]
pub extern "C" fn mouse_wheel(delta_x: i32, delta_y: i32) {
    let mut event: oxid_event = unsafe { std::mem::zeroed() };

    event.type_ = OXID_EVENT_TYPE_MOUSE_SCROLL;
    event.scroll_x = delta_x as f32;
    event.scroll_y = delta_y as f32;
    unsafe {
        oxid_context().event(event);
    }
}

#[no_mangle]
pub extern "C" fn key_down(key: u32, modifiers: u32, repeat: bool) {
    let mut event: oxid_event = unsafe { std::mem::zeroed() };

    event.type_ = OXID_EVENT_TYPE_KEY_DOWN;
    event.key_code = key;
    event.modifiers = modifiers;
    event.key_repeat = repeat;
    unsafe {
        oxid_context().event(event);
    }
}

#[no_mangle]
pub extern "C" fn key_press(key: u32) {
    let mut event: oxid_event = unsafe { std::mem::zeroed() };

    event.type_ = OXID_EVENT_TYPE_CHAR;
    event.char_code = key;
    unsafe {
        oxid_context().event(event);
    }
}

#[no_mangle]
pub extern "C" fn key_up(key: u32) {
    let mut event: oxid_event = unsafe { std::mem::zeroed() };

    event.type_ = OXID_EVENT_TYPE_KEY_UP;
    event.key_code = key;
    unsafe {
        oxid_context().event(event);
    }
}

#[no_mangle]
pub extern "C" fn resize(width: i32, height: i32) {
    let mut event: oxid_event = unsafe { std::mem::zeroed() };

    event.type_ = OXID_EVENT_TYPE_RESIZED;
    event.window_width = width;
    event.window_height = height;
    unsafe {
        oxid_context().event(event);
    }
}

#[no_mangle]
pub extern "C" fn touch(event_type: u32, id: u32, x: f32, y: f32) {
    let mut event: oxid_event = unsafe { std::mem::zeroed() };

    event.type_ = event_type as u32;
    event.num_touches = 1;
    event.touches[0].identifier = id as usize;
    event.touches[0].pos_x = x;
    event.touches[0].pos_y = y;
    event.touches[0].changed = true;
    unsafe {
        oxid_context().event(event);
    }
}