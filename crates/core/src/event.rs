use crate::Context;

use crate::oxid::{self, oxid_keycode, oxid_mousebutton};

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
            oxid::oxid_mousebutton_OXID_MOUSEBUTTON_LEFT => MouseButton::Left,
            oxid::oxid_mousebutton_OXID_MOUSEBUTTON_RIGHT => MouseButton::Right,
            oxid::oxid_mousebutton_OXID_MOUSEBUTTON_MIDDLE => MouseButton::Middle,
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
            oxid::OXID_KEYCODE_SPACE => KeyCode::Space,
            oxid::OXID_KEYCODE_APOSTROPHE => KeyCode::Apostrophe,
            oxid::OXID_KEYCODE_COMMA => KeyCode::Comma,
            oxid::OXID_KEYCODE_MINUS => KeyCode::Minus,
            oxid::OXID_KEYCODE_PERIOD => KeyCode::Period,
            oxid::OXID_KEYCODE_SLASH => KeyCode::Slash,
            oxid::OXID_KEYCODE_0 => KeyCode::Key0,
            oxid::OXID_KEYCODE_1 => KeyCode::Key1,
            oxid::OXID_KEYCODE_2 => KeyCode::Key2,
            oxid::OXID_KEYCODE_3 => KeyCode::Key3,
            oxid::OXID_KEYCODE_4 => KeyCode::Key4,
            oxid::OXID_KEYCODE_5 => KeyCode::Key5,
            oxid::OXID_KEYCODE_6 => KeyCode::Key6,
            oxid::OXID_KEYCODE_7 => KeyCode::Key7,
            oxid::OXID_KEYCODE_8 => KeyCode::Key8,
            oxid::OXID_KEYCODE_9 => KeyCode::Key9,
            oxid::OXID_KEYCODE_SEMICOLON => KeyCode::Semicolon,
            oxid::OXID_KEYCODE_EQUAL => KeyCode::Equal,
            oxid::OXID_KEYCODE_A => KeyCode::A,
            oxid::OXID_KEYCODE_B => KeyCode::B,
            oxid::OXID_KEYCODE_C => KeyCode::C,
            oxid::OXID_KEYCODE_D => KeyCode::D,
            oxid::OXID_KEYCODE_E => KeyCode::E,
            oxid::OXID_KEYCODE_F => KeyCode::F,
            oxid::OXID_KEYCODE_G => KeyCode::G,
            oxid::OXID_KEYCODE_H => KeyCode::H,
            oxid::OXID_KEYCODE_I => KeyCode::I,
            oxid::OXID_KEYCODE_J => KeyCode::J,
            oxid::OXID_KEYCODE_K => KeyCode::K,
            oxid::OXID_KEYCODE_L => KeyCode::L,
            oxid::OXID_KEYCODE_M => KeyCode::M,
            oxid::OXID_KEYCODE_N => KeyCode::N,
            oxid::OXID_KEYCODE_O => KeyCode::O,
            oxid::OXID_KEYCODE_P => KeyCode::P,
            oxid::OXID_KEYCODE_Q => KeyCode::Q,
            oxid::OXID_KEYCODE_R => KeyCode::R,
            oxid::OXID_KEYCODE_S => KeyCode::S,
            oxid::OXID_KEYCODE_T => KeyCode::T,
            oxid::OXID_KEYCODE_U => KeyCode::U,
            oxid::OXID_KEYCODE_V => KeyCode::V,
            oxid::OXID_KEYCODE_W => KeyCode::W,
            oxid::OXID_KEYCODE_X => KeyCode::X,
            oxid::OXID_KEYCODE_Y => KeyCode::Y,
            oxid::OXID_KEYCODE_Z => KeyCode::Z,
            oxid::OXID_KEYCODE_LEFT_BRACKET => KeyCode::LeftBracket,
            oxid::OXID_KEYCODE_BACKSLASH => KeyCode::Backslash,
            oxid::OXID_KEYCODE_RIGHT_BRACKET => KeyCode::RightBracket,
            oxid::OXID_KEYCODE_GRAVE_ACCENT => KeyCode::GraveAccent,
            oxid::OXID_KEYCODE_WORLD_1 => KeyCode::World1,
            oxid::OXID_KEYCODE_WORLD_2 => KeyCode::World2,
            oxid::OXID_KEYCODE_ESCAPE => KeyCode::Escape,
            oxid::OXID_KEYCODE_ENTER => KeyCode::Enter,
            oxid::OXID_KEYCODE_TAB => KeyCode::Tab,
            oxid::OXID_KEYCODE_BACKSPACE => KeyCode::Backspace,
            oxid::OXID_KEYCODE_INSERT => KeyCode::Insert,
            oxid::OXID_KEYCODE_DELETE => KeyCode::Delete,
            oxid::OXID_KEYCODE_RIGHT => KeyCode::Right,
            oxid::OXID_KEYCODE_LEFT => KeyCode::Left,
            oxid::OXID_KEYCODE_DOWN => KeyCode::Down,
            oxid::OXID_KEYCODE_UP => KeyCode::Up,
            oxid::OXID_KEYCODE_PAGE_UP => KeyCode::PageUp,
            oxid::OXID_KEYCODE_PAGE_DOWN => KeyCode::PageDown,
            oxid::OXID_KEYCODE_HOME => KeyCode::Home,
            oxid::OXID_KEYCODE_END => KeyCode::End,
            oxid::OXID_KEYCODE_CAPS_LOCK => KeyCode::CapsLock,
            oxid::OXID_KEYCODE_SCROLL_LOCK => KeyCode::ScrollLock,
            oxid::OXID_KEYCODE_NUM_LOCK => KeyCode::NumLock,
            oxid::OXID_KEYCODE_PRINT_SCREEN => KeyCode::PrintScreen,
            oxid::OXID_KEYCODE_PAUSE => KeyCode::Pause,
            oxid::OXID_KEYCODE_F1 => KeyCode::F1,
            oxid::OXID_KEYCODE_F2 => KeyCode::F2,
            oxid::OXID_KEYCODE_F3 => KeyCode::F3,
            oxid::OXID_KEYCODE_F4 => KeyCode::F4,
            oxid::OXID_KEYCODE_F5 => KeyCode::F5,
            oxid::OXID_KEYCODE_F6 => KeyCode::F6,
            oxid::OXID_KEYCODE_F7 => KeyCode::F7,
            oxid::OXID_KEYCODE_F8 => KeyCode::F8,
            oxid::OXID_KEYCODE_F9 => KeyCode::F9,
            oxid::OXID_KEYCODE_F10 => KeyCode::F10,
            oxid::OXID_KEYCODE_F11 => KeyCode::F11,
            oxid::OXID_KEYCODE_F12 => KeyCode::F12,
            oxid::OXID_KEYCODE_F13 => KeyCode::F13,
            oxid::OXID_KEYCODE_F14 => KeyCode::F14,
            oxid::OXID_KEYCODE_F15 => KeyCode::F15,
            oxid::OXID_KEYCODE_F16 => KeyCode::F16,
            oxid::OXID_KEYCODE_F17 => KeyCode::F17,
            oxid::OXID_KEYCODE_F18 => KeyCode::F18,
            oxid::OXID_KEYCODE_F19 => KeyCode::F19,
            oxid::OXID_KEYCODE_F20 => KeyCode::F20,
            oxid::OXID_KEYCODE_F21 => KeyCode::F21,
            oxid::OXID_KEYCODE_F22 => KeyCode::F22,
            oxid::OXID_KEYCODE_F23 => KeyCode::F23,
            oxid::OXID_KEYCODE_F24 => KeyCode::F24,
            oxid::OXID_KEYCODE_F25 => KeyCode::F25,
            oxid::OXID_KEYCODE_KP_0 => KeyCode::Kp0,
            oxid::OXID_KEYCODE_KP_1 => KeyCode::Kp1,
            oxid::OXID_KEYCODE_KP_2 => KeyCode::Kp2,
            oxid::OXID_KEYCODE_KP_3 => KeyCode::Kp3,
            oxid::OXID_KEYCODE_KP_4 => KeyCode::Kp4,
            oxid::OXID_KEYCODE_KP_5 => KeyCode::Kp5,
            oxid::OXID_KEYCODE_KP_6 => KeyCode::Kp6,
            oxid::OXID_KEYCODE_KP_7 => KeyCode::Kp7,
            oxid::OXID_KEYCODE_KP_8 => KeyCode::Kp8,
            oxid::OXID_KEYCODE_KP_9 => KeyCode::Kp9,
            oxid::OXID_KEYCODE_KP_DECIMAL => KeyCode::KpDecimal,
            oxid::OXID_KEYCODE_KP_DIVIDE => KeyCode::KpDivide,
            oxid::OXID_KEYCODE_KP_MULTIPLY => KeyCode::KpMultiply,
            oxid::OXID_KEYCODE_KP_SUBTRACT => KeyCode::KpSubtract,
            oxid::OXID_KEYCODE_KP_ADD => KeyCode::KpAdd,
            oxid::OXID_KEYCODE_KP_ENTER => KeyCode::KpEnter,
            oxid::OXID_KEYCODE_KP_EQUAL => KeyCode::KpEqual,
            oxid::OXID_KEYCODE_LEFT_SHIFT => KeyCode::LeftShift,
            oxid::OXID_KEYCODE_LEFT_CONTROL => KeyCode::LeftControl,
            oxid::OXID_KEYCODE_LEFT_ALT => KeyCode::LeftAlt,
            oxid::OXID_KEYCODE_LEFT_SUPER => KeyCode::LeftSuper,
            oxid::OXID_KEYCODE_RIGHT_SHIFT => KeyCode::RightShift,
            oxid::OXID_KEYCODE_RIGHT_CONTROL => KeyCode::RightControl,
            oxid::OXID_KEYCODE_RIGHT_ALT => KeyCode::RightAlt,
            oxid::OXID_KEYCODE_RIGHT_SUPER => KeyCode::RightSuper,
            oxid::OXID_KEYCODE_MENU => KeyCode::Menu,
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

        if value & oxid::OXID_MODIFIER_SHIFT != 0 {
            key_mods.shift = true;
        }
        if value & oxid::OXID_MODIFIER_CTRL != 0 {
            key_mods.ctrl = true;
        }
        if value & oxid::OXID_MODIFIER_ALT != 0 {
            key_mods.alt = true;
        }
        if value & oxid::OXID_MODIFIER_SUPER != 0 {
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
            oxid::OXID_EVENT_TYPE_TOUCHES_BEGAN => TouchPhase::Started,
            oxid::OXID_EVENT_TYPE_TOUCHES_ENDED => TouchPhase::Ended,
            oxid::OXID_EVENT_TYPE_TOUCHES_CANCELLED => TouchPhase::Cancelled,
            oxid::OXID_EVENT_TYPE_TOUCHES_MOVED => TouchPhase::Moved,
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