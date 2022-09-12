use crate::core::Context;
use crate::wasm::{self, oxid_keycode, oxid_mousebutton};

#[derive(Debug, Copy, Clone, PartialEq, Hash, Eq)]
pub enum MouseButton {
    Right,
    Left,
    Middle,
    Unknown,
}

#[derive(Debug, Copy, Clone)]
pub struct Touch {
    pub id: u32,
    pub x: f32,
    pub y: f32,
}

impl From<oxid_mousebutton> for MouseButton {
    fn from(btn: oxid_mousebutton) -> MouseButton {
        match btn {
            wasm::oxid_mousebutton_OXID_MOUSEBUTTON_LEFT => MouseButton::Left,
            wasm::oxid_mousebutton_OXID_MOUSEBUTTON_RIGHT => MouseButton::Right,
            wasm::oxid_mousebutton_OXID_MOUSEBUTTON_MIDDLE => MouseButton::Middle,
            _ => MouseButton::Unknown,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Hash, Eq)]
#[repr(u32)]
pub enum KeyCode {
    Space,
    Apostrophe,
    Comma,
    Minus,
    Period,
    Slash,
    Key0,
    Key1,
    Key2,
    Key3,
    Key4,
    Key5,
    Key6,
    Key7,
    Key8,
    Key9,
    Semicolon,
    Equal,
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
    LeftBracket,
    Backslash,
    RightBracket,
    GraveAccent,
    World1,
    World2,
    Escape,
    Enter,
    Tab,
    Backspace,
    Insert,
    Delete,
    Right,
    Left,
    Down,
    Up,
    PageUp,
    PageDown,
    Home,
    End,
    CapsLock,
    ScrollLock,
    NumLock,
    PrintScreen,
    Pause,
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
    F13,
    F14,
    F15,
    F16,
    F17,
    F18,
    F19,
    F20,
    F21,
    F22,
    F23,
    F24,
    F25,
    Kp0,
    Kp1,
    Kp2,
    Kp3,
    Kp4,
    Kp5,
    Kp6,
    Kp7,
    Kp8,
    Kp9,
    KpDecimal,
    KpDivide,
    KpMultiply,
    KpSubtract,
    KpAdd,
    KpEnter,
    KpEqual,
    LeftShift,
    LeftControl,
    LeftAlt,
    LeftSuper,
    RightShift,
    RightControl,
    RightAlt,
    RightSuper,
    Menu,
    Unknown,
}

impl From<oxid_keycode> for KeyCode {
    fn from(key_code: oxid_keycode) -> KeyCode {
        match key_code {
            wasm::OXID_KEYCODE_SPACE => KeyCode::Space,
            wasm::OXID_KEYCODE_APOSTROPHE => KeyCode::Apostrophe,
            wasm::OXID_KEYCODE_COMMA => KeyCode::Comma,
            wasm::OXID_KEYCODE_MINUS => KeyCode::Minus,
            wasm::OXID_KEYCODE_PERIOD => KeyCode::Period,
            wasm::OXID_KEYCODE_SLASH => KeyCode::Slash,
            wasm::OXID_KEYCODE_0 => KeyCode::Key0,
            wasm::OXID_KEYCODE_1 => KeyCode::Key1,
            wasm::OXID_KEYCODE_2 => KeyCode::Key2,
            wasm::OXID_KEYCODE_3 => KeyCode::Key3,
            wasm::OXID_KEYCODE_4 => KeyCode::Key4,
            wasm::OXID_KEYCODE_5 => KeyCode::Key5,
            wasm::OXID_KEYCODE_6 => KeyCode::Key6,
            wasm::OXID_KEYCODE_7 => KeyCode::Key7,
            wasm::OXID_KEYCODE_8 => KeyCode::Key8,
            wasm::OXID_KEYCODE_9 => KeyCode::Key9,
            wasm::OXID_KEYCODE_SEMICOLON => KeyCode::Semicolon,
            wasm::OXID_KEYCODE_EQUAL => KeyCode::Equal,
            wasm::OXID_KEYCODE_A => KeyCode::A,
            wasm::OXID_KEYCODE_B => KeyCode::B,
            wasm::OXID_KEYCODE_C => KeyCode::C,
            wasm::OXID_KEYCODE_D => KeyCode::D,
            wasm::OXID_KEYCODE_E => KeyCode::E,
            wasm::OXID_KEYCODE_F => KeyCode::F,
            wasm::OXID_KEYCODE_G => KeyCode::G,
            wasm::OXID_KEYCODE_H => KeyCode::H,
            wasm::OXID_KEYCODE_I => KeyCode::I,
            wasm::OXID_KEYCODE_J => KeyCode::J,
            wasm::OXID_KEYCODE_K => KeyCode::K,
            wasm::OXID_KEYCODE_L => KeyCode::L,
            wasm::OXID_KEYCODE_M => KeyCode::M,
            wasm::OXID_KEYCODE_N => KeyCode::N,
            wasm::OXID_KEYCODE_O => KeyCode::O,
            wasm::OXID_KEYCODE_P => KeyCode::P,
            wasm::OXID_KEYCODE_Q => KeyCode::Q,
            wasm::OXID_KEYCODE_R => KeyCode::R,
            wasm::OXID_KEYCODE_S => KeyCode::S,
            wasm::OXID_KEYCODE_T => KeyCode::T,
            wasm::OXID_KEYCODE_U => KeyCode::U,
            wasm::OXID_KEYCODE_V => KeyCode::V,
            wasm::OXID_KEYCODE_W => KeyCode::W,
            wasm::OXID_KEYCODE_X => KeyCode::X,
            wasm::OXID_KEYCODE_Y => KeyCode::Y,
            wasm::OXID_KEYCODE_Z => KeyCode::Z,
            wasm::OXID_KEYCODE_LEFT_BRACKET => KeyCode::LeftBracket,
            wasm::OXID_KEYCODE_BACKSLASH => KeyCode::Backslash,
            wasm::OXID_KEYCODE_RIGHT_BRACKET => KeyCode::RightBracket,
            wasm::OXID_KEYCODE_GRAVE_ACCENT => KeyCode::GraveAccent,
            wasm::OXID_KEYCODE_WORLD_1 => KeyCode::World1,
            wasm::OXID_KEYCODE_WORLD_2 => KeyCode::World2,
            wasm::OXID_KEYCODE_ESCAPE => KeyCode::Escape,
            wasm::OXID_KEYCODE_ENTER => KeyCode::Enter,
            wasm::OXID_KEYCODE_TAB => KeyCode::Tab,
            wasm::OXID_KEYCODE_BACKSPACE => KeyCode::Backspace,
            wasm::OXID_KEYCODE_INSERT => KeyCode::Insert,
            wasm::OXID_KEYCODE_DELETE => KeyCode::Delete,
            wasm::OXID_KEYCODE_RIGHT => KeyCode::Right,
            wasm::OXID_KEYCODE_LEFT => KeyCode::Left,
            wasm::OXID_KEYCODE_DOWN => KeyCode::Down,
            wasm::OXID_KEYCODE_UP => KeyCode::Up,
            wasm::OXID_KEYCODE_PAGE_UP => KeyCode::PageUp,
            wasm::OXID_KEYCODE_PAGE_DOWN => KeyCode::PageDown,
            wasm::OXID_KEYCODE_HOME => KeyCode::Home,
            wasm::OXID_KEYCODE_END => KeyCode::End,
            wasm::OXID_KEYCODE_CAPS_LOCK => KeyCode::CapsLock,
            wasm::OXID_KEYCODE_SCROLL_LOCK => KeyCode::ScrollLock,
            wasm::OXID_KEYCODE_NUM_LOCK => KeyCode::NumLock,
            wasm::OXID_KEYCODE_PRINT_SCREEN => KeyCode::PrintScreen,
            wasm::OXID_KEYCODE_PAUSE => KeyCode::Pause,
            wasm::OXID_KEYCODE_F1 => KeyCode::F1,
            wasm::OXID_KEYCODE_F2 => KeyCode::F2,
            wasm::OXID_KEYCODE_F3 => KeyCode::F3,
            wasm::OXID_KEYCODE_F4 => KeyCode::F4,
            wasm::OXID_KEYCODE_F5 => KeyCode::F5,
            wasm::OXID_KEYCODE_F6 => KeyCode::F6,
            wasm::OXID_KEYCODE_F7 => KeyCode::F7,
            wasm::OXID_KEYCODE_F8 => KeyCode::F8,
            wasm::OXID_KEYCODE_F9 => KeyCode::F9,
            wasm::OXID_KEYCODE_F10 => KeyCode::F10,
            wasm::OXID_KEYCODE_F11 => KeyCode::F11,
            wasm::OXID_KEYCODE_F12 => KeyCode::F12,
            wasm::OXID_KEYCODE_F13 => KeyCode::F13,
            wasm::OXID_KEYCODE_F14 => KeyCode::F14,
            wasm::OXID_KEYCODE_F15 => KeyCode::F15,
            wasm::OXID_KEYCODE_F16 => KeyCode::F16,
            wasm::OXID_KEYCODE_F17 => KeyCode::F17,
            wasm::OXID_KEYCODE_F18 => KeyCode::F18,
            wasm::OXID_KEYCODE_F19 => KeyCode::F19,
            wasm::OXID_KEYCODE_F20 => KeyCode::F20,
            wasm::OXID_KEYCODE_F21 => KeyCode::F21,
            wasm::OXID_KEYCODE_F22 => KeyCode::F22,
            wasm::OXID_KEYCODE_F23 => KeyCode::F23,
            wasm::OXID_KEYCODE_F24 => KeyCode::F24,
            wasm::OXID_KEYCODE_F25 => KeyCode::F25,
            wasm::OXID_KEYCODE_KP_0 => KeyCode::Kp0,
            wasm::OXID_KEYCODE_KP_1 => KeyCode::Kp1,
            wasm::OXID_KEYCODE_KP_2 => KeyCode::Kp2,
            wasm::OXID_KEYCODE_KP_3 => KeyCode::Kp3,
            wasm::OXID_KEYCODE_KP_4 => KeyCode::Kp4,
            wasm::OXID_KEYCODE_KP_5 => KeyCode::Kp5,
            wasm::OXID_KEYCODE_KP_6 => KeyCode::Kp6,
            wasm::OXID_KEYCODE_KP_7 => KeyCode::Kp7,
            wasm::OXID_KEYCODE_KP_8 => KeyCode::Kp8,
            wasm::OXID_KEYCODE_KP_9 => KeyCode::Kp9,
            wasm::OXID_KEYCODE_KP_DECIMAL => KeyCode::KpDecimal,
            wasm::OXID_KEYCODE_KP_DIVIDE => KeyCode::KpDivide,
            wasm::OXID_KEYCODE_KP_MULTIPLY => KeyCode::KpMultiply,
            wasm::OXID_KEYCODE_KP_SUBTRACT => KeyCode::KpSubtract,
            wasm::OXID_KEYCODE_KP_ADD => KeyCode::KpAdd,
            wasm::OXID_KEYCODE_KP_ENTER => KeyCode::KpEnter,
            wasm::OXID_KEYCODE_KP_EQUAL => KeyCode::KpEqual,
            wasm::OXID_KEYCODE_LEFT_SHIFT => KeyCode::LeftShift,
            wasm::OXID_KEYCODE_LEFT_CONTROL => KeyCode::LeftControl,
            wasm::OXID_KEYCODE_LEFT_ALT => KeyCode::LeftAlt,
            wasm::OXID_KEYCODE_LEFT_SUPER => KeyCode::LeftSuper,
            wasm::OXID_KEYCODE_RIGHT_SHIFT => KeyCode::RightShift,
            wasm::OXID_KEYCODE_RIGHT_CONTROL => KeyCode::RightControl,
            wasm::OXID_KEYCODE_RIGHT_ALT => KeyCode::RightAlt,
            wasm::OXID_KEYCODE_RIGHT_SUPER => KeyCode::RightSuper,
            wasm::OXID_KEYCODE_MENU => KeyCode::Menu,
            _ => KeyCode::Unknown,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Default)]
pub struct KeyMods {
    pub shift: bool,
    pub ctrl: bool,
    pub alt: bool,
    pub logo: bool,
}

impl From<u32> for KeyMods {
    fn from(value: u32) -> KeyMods {
        let mut key_mods = KeyMods::default();

        if value & wasm::OXID_MODIFIER_SHIFT != 0 {
            key_mods.shift = true;
        }
        if value & wasm::OXID_MODIFIER_CTRL != 0 {
            key_mods.ctrl = true;
        }
        if value & wasm::OXID_MODIFIER_ALT != 0 {
            key_mods.alt = true;
        }
        if value & wasm::OXID_MODIFIER_SUPER != 0 {
            key_mods.logo = true;
        }

        key_mods
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub enum TouchPhase {
    Started,
    Moved,
    Ended,
    Cancelled,
}

impl From<u32> for TouchPhase {
    fn from(event: u32) -> TouchPhase {
        match event {
            wasm::OXID_EVENT_TYPE_TOUCHES_BEGAN => TouchPhase::Started,
            wasm::OXID_EVENT_TYPE_TOUCHES_ENDED => TouchPhase::Ended,
            wasm::OXID_EVENT_TYPE_TOUCHES_CANCELLED => TouchPhase::Cancelled,
            wasm::OXID_EVENT_TYPE_TOUCHES_MOVED => TouchPhase::Moved,
            _ => unreachable!(),
        }
    }
}

/// A trait defining event callbacks.
pub trait EventHandler {
    fn update(&mut self, _ctx: &mut Context);
    fn draw(&mut self, _ctx: &mut Context);
    fn resize_event(&mut self, _ctx: &mut Context, _width: f32, _height: f32) {}
    fn mouse_motion_event(&mut self, _ctx: &mut Context, _x: f32, _y: f32) {}
    fn mouse_wheel_event(&mut self, _ctx: &mut Context, _x: f32, _y: f32) {}
    fn mouse_button_down_event(
        &mut self,
        _ctx: &mut Context,
        _button: MouseButton,
        _x: f32,
        _y: f32,
    ) {
    }
    fn mouse_button_up_event(
        &mut self,
        _ctx: &mut Context,
        _button: MouseButton,
        _x: f32,
        _y: f32,
    ) {
    }

    fn char_event(
        &mut self,
        _ctx: &mut Context,
        _character: char,
        _keymods: KeyMods,
        _repeat: bool,
    ) {
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        _keycode: KeyCode,
        _keymods: KeyMods,
        _repeat: bool,
    ) {
    }

    fn key_up_event(&mut self, _ctx: &mut Context, _keycode: KeyCode, _keymods: KeyMods) {}

    /// Default implementation emulates mouse clicks
    fn touch_event(&mut self, ctx: &mut Context, phase: TouchPhase, _id: u64, x: f32, y: f32) {
        if phase == TouchPhase::Started {
            self.mouse_button_down_event(ctx, MouseButton::Left, x, y);
        }

        if phase == TouchPhase::Ended {
            self.mouse_button_up_event(ctx, MouseButton::Left, x, y);
        }

        if phase == TouchPhase::Moved {
            self.mouse_motion_event(ctx, x, y);
        }
    }

    /// Represents raw hardware mouse motion event
    /// Note that these events are delivered regardless of input focus and not in pixels, but in
    /// hardware units instead. And those units may be different from pixels depending on the target platform
    fn raw_mouse_motion(&mut self, _ctx: &mut Context, _dx: f32, _dy: f32) {}

    /// This event is sent when the userclicks the window's close button
    /// or application code calls the ctx.request_quit() function. The event
    /// handler callback code can handle this event by calling
    /// ctx.cancel_quit() to cancel the quit.
    /// If the event is ignored, the application will quit as usual.
    fn quit_requested_event(&mut self, _ctx: &mut Context) {}
}

/// A trait defining event callbacks.
/// Used for oxid's setup with user-owned Context.
/// The only difference from EventHandler - will not receive "&mut Context"
pub trait EventHandlerFree {
    fn update(&mut self);
    fn draw(&mut self);
    fn resize_event(&mut self, _width: f32, _height: f32) {}
    fn mouse_motion_event(&mut self, _x: f32, _y: f32) {}
    fn mouse_wheel_event(&mut self, _x: f32, _y: f32) {}
    fn mouse_button_down_event(&mut self, _button: MouseButton, _x: f32, _y: f32) {}
    fn mouse_button_up_event(&mut self, _button: MouseButton, _x: f32, _y: f32) {}
    fn char_event(&mut self, _character: char, _keymods: KeyMods, _repeat: bool) {}
    fn key_down_event(&mut self, _keycode: KeyCode, _keymods: KeyMods, _repeat: bool) {}
    fn key_up_event(&mut self, _keycode: KeyCode, _keymods: KeyMods) {}

    /// Default implementation emulates mouse clicks
    fn touch_event(&mut self, phase: TouchPhase, _id: u64, x: f32, y: f32) {
        if phase == TouchPhase::Started {
            self.mouse_button_down_event(MouseButton::Left, x, y);
        }

        if phase == TouchPhase::Ended {
            self.mouse_button_up_event(MouseButton::Left, x, y);
        }

        if phase == TouchPhase::Moved {
            self.mouse_motion_event(x, y);
        }
    }

    /// Represents raw hardware mouse motion event
    /// Note that these events are delivered regardless of input focus and not in pixels, but in
    /// hardware units instead. And those units may be different from pixels depending on the target platform
    fn raw_mouse_motion(&mut self, _dx: f32, _dy: f32) {}

    /// This event is sent when the userclicks the window's close button
    /// or application code calls the ctx.request_quit() function. The event
    /// handler callback code can handle this event by calling
    /// ctx.cancel_quit() to cancel the quit.
    /// If the event is ignored, the application will quit as usual.
    fn quit_requested_event(&mut self) {}
}
