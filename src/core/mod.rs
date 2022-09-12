pub mod clipboard;
pub mod conf;
pub mod fs;
pub mod graphics;
mod event;

pub use event::*;
pub use graphics::*;
pub use crate::wasm::{self, gl};

use std::ffi::CString;

#[deprecated(
    note = "libc rand is slow and inconsistent across platforms. Please use quad-rnd crate instead."
)]
pub unsafe fn rand() -> i32 {
    wasm::rand()
}

#[deprecated(
    note = "libc rand is slow and inconsistent across platforms. Please use quad-rnd crate instead."
)]
pub const RAND_MAX: u32 = wasm::RAND_MAX;

pub mod date {
    pub fn now() -> f64 {
        unsafe { crate::wasm::now() }
    }
}

impl Context {
    /// Capture mouse cursor to the current window
    /// On WASM this will automatically hide cursor
    /// On desktop this will bound cursor to windows border
    /// NOTICE: on desktop cursor will not be automatically released after window lost focus
    ///         so set_cursor_grab(false) on window's focus lost is recommended.
    /// TODO: implement window focus events
    pub fn set_cursor_grab(&self, grab: bool) {
        #[cfg(not(target_os = "ios"))]
        unsafe {
            wasm::oxid_set_cursor_grab(grab);
        }
    }

    /// Show or hide the mouse cursor
    pub fn show_mouse(&self, shown: bool) {
        unsafe {
            wasm::oxid_show_mouse(shown);
        }
    }
}

pub enum UserData {
    Owning((Box<dyn EventHandler>, Context)),
    Free(Box<dyn EventHandlerFree>),
}

impl UserData {
    pub fn owning(event_handler: impl EventHandler + 'static, ctx: Context) -> UserData {
        UserData::Owning((Box::new(event_handler), ctx))
    }

    pub fn free(event_handler: impl EventHandlerFree + 'static) -> UserData {
        UserData::Free(Box::new(event_handler))
    }
}

/// call appropriate event handler function - with or without Context reference
macro_rules! event_call {
    ( $event_handler:expr, $fn:ident $(, $args:expr)*) => {{
        match $event_handler {
            UserData::Owning((ref mut event_handler, ref mut context)) => {
                event_handler.$fn(context, $($args,)*);
            }
            UserData::Free(ref mut event_handler) => {
                event_handler.$fn($($args,)*);
            }
        }
    }};
}

enum UserDataState {
    Uninitialized(Box<dyn 'static + FnOnce(Context) -> UserData>),
    Intialized(UserData),
    Empty,
}

extern "C" fn init(user_data: *mut ::std::os::raw::c_void) {
    let data: &mut UserDataState = unsafe { &mut *(user_data as *mut UserDataState) };
    let empty = UserDataState::Empty;

    let f = std::mem::replace(data, empty);
    let f = if let UserDataState::Uninitialized(f) = f {
        f
    } else {
        panic!();
    };
    let context = graphics::Context::new();

    let user_data = f(context);
    *data = UserDataState::Intialized(user_data);
}

extern "C" fn frame(user_data: *mut ::std::os::raw::c_void) {
    let data: &mut UserDataState = unsafe { &mut *(user_data as *mut UserDataState) };

    let data = if let UserDataState::Intialized(ref mut data) = data {
        data
    } else {
        panic!()
    };

    event_call!(data, update);
    event_call!(data, draw);
}

extern "C" fn event(event: *const wasm::oxid_event, user_data: *mut ::std::os::raw::c_void) {
    let data: &mut UserDataState = unsafe { &mut *(user_data as *mut UserDataState) };
    let event = unsafe { &*event };

    let data = if let UserDataState::Intialized(ref mut data) = data {
        data
    } else {
        panic!()
    };

    match event.type_ {
        wasm::OXID_EVENT_TYPE_MOUSE_MOVE => {
            event_call!(data, mouse_motion_event, event.mouse_x, event.mouse_y);
        }
        wasm::OXID_EVENT_TYPE_MOUSE_SCROLL => {
            event_call!(data, mouse_wheel_event, event.scroll_x, event.scroll_y);
        }
        wasm::OXID_EVENT_TYPE_MOUSE_DOWN => {
            event_call!(
                data,
                mouse_button_down_event,
                MouseButton::from(event.mouse_button),
                event.mouse_x,
                event.mouse_y
            );
        }
        wasm::OXID_EVENT_TYPE_MOUSE_UP => {
            event_call!(
                data,
                mouse_button_up_event,
                MouseButton::from(event.mouse_button),
                event.mouse_x,
                event.mouse_y
            );
        }
        wasm::OXID_EVENT_TYPE_CHAR => {
            if let Some(character) = std::char::from_u32(event.char_code) {
                let key_mods = KeyMods::from(event.modifiers);

                event_call!(data, char_event, character, key_mods, event.key_repeat)
            }
        }
        wasm::OXID_EVENT_TYPE_KEY_DOWN => {
            let keycode = KeyCode::from(event.key_code);
            let key_mods = KeyMods::from(event.modifiers);

            event_call!(data, key_down_event, keycode, key_mods, event.key_repeat)
        }
        wasm::OXID_EVENT_TYPE_KEY_UP => {
            let keycode = KeyCode::from(event.key_code);
            let key_mods = KeyMods::from(event.modifiers);

            event_call!(data, key_up_event, keycode, key_mods);
        }
        wasm::OXID_EVENT_TYPE_RESIZED => {
            event_call!(
                data,
                resize_event,
                event.window_width as f32,
                event.window_height as f32
            );
        }
        wasm::OXID_EVENT_TYPE_TOUCHES_BEGAN
        | wasm::OXID_EVENT_TYPE_TOUCHES_ENDED
        | wasm::OXID_EVENT_TYPE_TOUCHES_CANCELLED
        | wasm::OXID_EVENT_TYPE_TOUCHES_MOVED => {
            for i in 0..(event.num_touches as usize) {
                if event.touches[i].changed {
                    event_call!(
                        data,
                        touch_event,
                        event.type_.into(),
                        event.touches[i].identifier as u64,
                        event.touches[i].pos_x,
                        event.touches[i].pos_y
                    );
                }
            }
        }
        wasm::OXID_EVENT_TYPE_QUIT_REQUESTED => {
            event_call!(data, quit_requested_event);
        }
        #[cfg(not(target_os = "ios"))]
        wasm::OXID_EVENT_TYPE_RAW_DEVICE => {
            event_call!(data, raw_mouse_motion, event.mouse_dx, event.mouse_dy);
        }
        _ => {}
    }
}

/// Start oxid::core.
/// Initialization callback will be called when oxid's Context is ready.
/// User can take ownership on Context and store it in user Code. Or return it back
/// to oxid and give oxid ownership on Context.
///
/// Variant wth EventHandler:
/// ```no_run
/// # use oxid::core::*;
/// struct Stage;
///
/// impl EventHandler for Stage {
///     fn update(&mut self, _ctx: &mut Context) {}
///     fn draw(&mut self, _ctx: &mut Context) {}
/// }
/// fn main() {
///     oxid::core::start(conf::Conf::default(), |ctx| UserData::owning(Stage, ctx));
/// }
/// ```
///
/// Variant wth EventHandlerFree:
/// ```no_run
/// # use oxid::core::*;
/// struct Stage {
///     ctx: Context,
/// }
/// impl EventHandlerFree for Stage {
///     fn update(&mut self) {}
///     fn draw(&mut self) {}
/// }
/// fn main() {
///     oxid::core::start(conf::Conf::default(), |ctx| UserData::free(Stage { ctx }));
/// }
/// ```
pub fn start<F>(conf: conf::Conf, f: F)
where
    F: 'static + FnOnce(Context) -> UserData,
{
    let mut desc: wasm::oxid_desc = unsafe { std::mem::zeroed() };

    let title = CString::new(conf.window_title.as_bytes()).unwrap_or_else(|e| panic!(e));

    let mut user_data = Box::new(UserDataState::Uninitialized(Box::new(f)));

    desc.sample_count = conf.sample_count;
    desc.width = conf.window_width;
    desc.height = conf.window_height;
    desc.fullscreen = conf.fullscreen as _;
    desc.high_dpi = conf.high_dpi as _;
    desc.window_title = title.as_ptr();
    desc.user_data = &mut *user_data as *mut _ as *mut _;
    desc.init_userdata_cb = Some(init);
    desc.frame_userdata_cb = Some(frame);
    desc.event_userdata_cb = Some(event);

    std::mem::forget(user_data);

    unsafe { crate::wasm::oxid_run(&desc as *const _) };
}
