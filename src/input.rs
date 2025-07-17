use raylib::prelude::*;
use std::{cell::Cell, str::FromStr};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum KeyState {
    Down,
    Released,
    Up,
    Pressed,
    PressedRepeat,
}

pub trait KeyStateExt {
    fn down(self) -> EventSource;
    fn released(self) -> EventSource;
    fn up(self) -> EventSource;
    fn pressed(self) -> EventSource;
    fn pressed_repeat(self) -> EventSource;
}

impl KeyStateExt for KeyboardKey {
    #[inline]
    fn down(self) -> EventSource {
        EventSource::KeyboardKey(KeyState::Down, self)
    }
    #[inline]
    fn released(self) -> EventSource {
        EventSource::KeyboardKey(KeyState::Released, self)
    }
    #[inline]
    fn up(self) -> EventSource {
        EventSource::KeyboardKey(KeyState::Up, self)
    }
    #[inline]
    fn pressed(self) -> EventSource {
        EventSource::KeyboardKey(KeyState::Pressed, self)
    }
    #[inline]
    fn pressed_repeat(self) -> EventSource {
        EventSource::KeyboardKey(KeyState::PressedRepeat, self)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ButtonState {
    Down,
    Released,
    Up,
    Pressed,
}

pub trait ButtonStateExt {
    fn down(self) -> EventSource;
    fn released(self) -> EventSource;
    fn up(self) -> EventSource;
    fn pressed(self) -> EventSource;
}

impl ButtonStateExt for MouseButton {
    #[inline]
    fn down(self) -> EventSource {
        EventSource::MouseButton(ButtonState::Down, self)
    }
    #[inline]
    fn released(self) -> EventSource {
        EventSource::MouseButton(ButtonState::Released, self)
    }
    #[inline]
    fn up(self) -> EventSource {
        EventSource::MouseButton(ButtonState::Up, self)
    }
    #[inline]
    fn pressed(self) -> EventSource {
        EventSource::MouseButton(ButtonState::Pressed, self)
    }
}

impl ButtonStateExt for (i32, GamepadButton) {
    #[inline]
    fn down(self) -> EventSource {
        EventSource::GamepadButton(ButtonState::Down, self.0, self.1)
    }
    #[inline]
    fn released(self) -> EventSource {
        EventSource::GamepadButton(ButtonState::Released, self.0, self.1)
    }
    #[inline]
    fn up(self) -> EventSource {
        EventSource::GamepadButton(ButtonState::Up, self.0, self.1)
    }
    #[inline]
    fn pressed(self) -> EventSource {
        EventSource::GamepadButton(ButtonState::Pressed, self.0, self.1)
    }
}

pub type Gamepad = i32;

#[derive(Debug)]
pub enum EventSource {
    Constant(bool),
    Not(Box<EventSource>),
    And(Vec<EventSource>),
    Nand(Vec<EventSource>),
    Or(Vec<EventSource>),
    Nor(Vec<EventSource>),
    Xor(Box<(EventSource, EventSource)>),
    Xnor(Box<(EventSource, EventSource)>),
    Toggle(Box<EventSource>, Cell<bool>),
    Eq(Box<(AxisSource, AxisSource, AxisSource)>),
    Ne(Box<(AxisSource, AxisSource, AxisSource)>),
    Gt(Box<(AxisSource, AxisSource)>),
    Ge(Box<(AxisSource, AxisSource)>),
    Lt(Box<(AxisSource, AxisSource)>),
    Le(Box<(AxisSource, AxisSource)>),
    KeyboardKey(KeyState, KeyboardKey),
    MouseButton(ButtonState, MouseButton),
    GamepadButton(ButtonState, Gamepad, GamepadButton),
}

impl From<bool> for EventSource {
    fn from(value: bool) -> Self {
        Self::Constant(value)
    }
}

impl EventSource {
    fn get(&self, rl: &RaylibHandle) -> bool {
        match self {
            &Self::Constant(val) => val,
            Self::Not(src) => !src.get(rl),
            Self::And(src) => src.iter().all(|src| src.get(rl)),
            Self::Nand(src) => !src.iter().all(|src| src.get(rl)),
            Self::Or(src) => src.iter().any(|src| src.get(rl)),
            Self::Nor(src) => !src.iter().any(|src| src.get(rl)),
            Self::Xor(src) => src.0.get(rl) != src.1.get(rl),
            Self::Xnor(src) => src.0.get(rl) == src.1.get(rl),
            Self::Toggle(src, mem) => {
                if src.get(rl) {
                    mem.set(!mem.get());
                }
                mem.get()
            }
            Self::Eq(src) => (src.0.get(rl) - src.1.get(rl)).abs() <= src.2.get(rl),
            Self::Ne(src) => (src.0.get(rl) - src.1.get(rl)).abs() > src.2.get(rl),
            Self::Gt(src) => src.0.get(rl) > src.1.get(rl),
            Self::Ge(src) => src.0.get(rl) >= src.1.get(rl),
            Self::Lt(src) => src.0.get(rl) < src.1.get(rl),
            Self::Le(src) => src.0.get(rl) <= src.1.get(rl),
            &Self::KeyboardKey(state, key) => match state {
                KeyState::Down => rl.is_key_down(key),
                KeyState::Released => rl.is_key_released(key),
                KeyState::Up => rl.is_key_up(key),
                KeyState::Pressed => rl.is_key_pressed(key),
                KeyState::PressedRepeat => rl.is_key_pressed_repeat(key),
            },
            &Self::MouseButton(state, button) => match state {
                ButtonState::Down => rl.is_mouse_button_down(button),
                ButtonState::Released => rl.is_mouse_button_released(button),
                ButtonState::Up => rl.is_mouse_button_up(button),
                ButtonState::Pressed => rl.is_mouse_button_pressed(button),
            },
            &Self::GamepadButton(state, gamepad, button) => match state {
                ButtonState::Down => rl.is_gamepad_button_down(gamepad, button),
                ButtonState::Released => rl.is_gamepad_button_released(gamepad, button),
                ButtonState::Up => rl.is_gamepad_button_up(gamepad, button),
                ButtonState::Pressed => rl.is_gamepad_button_pressed(gamepad, button),
            },
        }
    }
}

impl std::ops::Not for EventSource {
    type Output = EventSource;

    #[inline]
    fn not(self) -> Self::Output {
        EventSource::Not(Box::new(self))
    }
}

impl<T: Into<Self>> std::ops::BitAnd<T> for EventSource {
    type Output = EventSource;

    #[inline]
    fn bitand(self, rhs: T) -> Self::Output {
        EventSource::And(vec![self, rhs.into()])
    }
}

impl<T: Into<Self>> std::ops::BitOr<T> for EventSource {
    type Output = EventSource;

    #[inline]
    fn bitor(self, rhs: T) -> Self::Output {
        EventSource::Or(vec![self, rhs.into()])
    }
}

impl<T: Into<Self>> std::ops::BitXor<T> for EventSource {
    type Output = EventSource;

    #[inline]
    fn bitxor(self, rhs: T) -> Self::Output {
        EventSource::Xor(Box::new((self, rhs.into())))
    }
}

impl<T: Into<Self>> std::ops::Sub<T> for EventSource {
    type Output = AxisSource;

    #[inline]
    fn sub(self, rhs: T) -> Self::Output {
        AxisSource::Subtract(Box::new((self, rhs.into())))
    }
}

#[derive(Debug)]
pub enum AxisSource {
    Constant(f32),
    Map(Box<(EventSource, AxisSource, AxisSource)>),
    Subtract(Box<(EventSource, EventSource)>),
    Neg(Box<AxisSource>),
    Abs(Box<AxisSource>),
    Recip(Box<AxisSource>),
    Product(Vec<AxisSource>),
    Sum(Vec<AxisSource>),
    X(Box<VectorSource>),
    Y(Box<VectorSource>),
    MaxMagnitude(Box<VectorSource>),
    Magnitude(Box<VectorSource>),
    Dot(Box<(VectorSource, VectorSource)>),
    GamepadAxis(Gamepad, GamepadAxis),
}

impl From<f32> for AxisSource {
    fn from(value: f32) -> Self {
        Self::Constant(value)
    }
}

impl AxisSource {
    fn get(&self, rl: &RaylibHandle) -> f32 {
        match self {
            &Self::Constant(val) => val,
            Self::Map(src) => {
                if src.0.get(rl) {
                    src.1.get(rl)
                } else {
                    src.2.get(rl)
                }
            }
            Self::Subtract(src) => f32::from(i8::from(src.0.get(rl)) - i8::from(src.1.get(rl))),
            Self::Neg(src) => -src.get(rl),
            Self::Abs(src) => src.get(rl).abs(),
            Self::Recip(src) => src.get(rl).recip(),
            Self::Product(src) => src.iter().map(|src| src.get(rl)).product(),
            Self::Sum(src) => src.iter().map(|src| src.get(rl)).sum(),
            Self::X(src) => src.get(rl).x,
            Self::Y(src) => src.get(rl).y,
            Self::MaxMagnitude(src) => {
                let val = src.get(rl);
                val[val.abs().max_position()]
            }
            Self::Magnitude(src) => src.get(rl).length(),
            Self::Dot(src) => src.0.get(rl).dot(src.1.get(rl)),
            &Self::GamepadAxis(gamepad, axis) => rl.get_gamepad_axis_movement(gamepad, axis),
        }
    }
}

impl std::ops::Neg for AxisSource {
    type Output = AxisSource;

    #[inline]
    fn neg(self) -> Self::Output {
        AxisSource::Neg(Box::new(self))
    }
}

impl<T: Into<Self>> std::ops::Add<T> for AxisSource {
    type Output = AxisSource;

    #[inline]
    fn add(self, rhs: T) -> Self::Output {
        AxisSource::Sum(vec![self, rhs.into()])
    }
}

impl<T: Into<Self>> std::ops::Mul<T> for AxisSource {
    type Output = AxisSource;

    #[inline]
    fn mul(self, rhs: T) -> Self::Output {
        AxisSource::Product(vec![self, rhs.into()])
    }
}

impl AxisSource {
    #[inline]
    pub fn cartesian(self, rhs: impl Into<Self>) -> VectorSource {
        VectorSource::Cartesian(Box::new((self, rhs.into())))
    }

    #[inline]
    pub fn polar(self, rhs: impl Into<Self>) -> VectorSource {
        VectorSource::Polar(Box::new((self, rhs.into())))
    }

    #[inline]
    pub fn eq(self, rhs: impl Into<Self>, epsilon: impl Into<Self>) -> EventSource {
        EventSource::Eq(Box::new((self, rhs.into(), epsilon.into())))
    }
    #[inline]
    pub fn ne(self, rhs: impl Into<Self>, epsilon: impl Into<Self>) -> EventSource {
        EventSource::Ne(Box::new((self, rhs.into(), epsilon.into())))
    }
    #[inline]
    pub fn gt(self, rhs: impl Into<Self>) -> EventSource {
        EventSource::Gt(Box::new((self, rhs.into())))
    }
    #[inline]
    pub fn ge(self, rhs: impl Into<Self>) -> EventSource {
        EventSource::Ge(Box::new((self, rhs.into())))
    }
    #[inline]
    pub fn lt(self, rhs: impl Into<Self>) -> EventSource {
        EventSource::Lt(Box::new((self, rhs.into())))
    }
    #[inline]
    pub fn le(self, rhs: impl Into<Self>) -> EventSource {
        EventSource::Le(Box::new((self, rhs.into())))
    }
}

#[derive(Debug)]
pub enum VectorSource {
    Constant(Vector2),
    Cartesian(Box<(AxisSource, AxisSource)>),
    Polar(Box<(AxisSource, AxisSource)>),
    Normalize(Box<VectorSource>),
    Rotate(Box<(VectorSource, AxisSource)>),
    Reflect(Box<(VectorSource, VectorSource)>),
    MouseWheel,
    Mouse,
}

impl From<Vector2> for VectorSource {
    fn from(value: Vector2) -> Self {
        Self::Constant(value)
    }
}

impl VectorSource {
    fn get(&self, rl: &RaylibHandle) -> Vector2 {
        match self {
            &Self::Constant(val) => val,
            Self::Cartesian(src) => Vector2::new(src.0.get(rl), src.1.get(rl)),
            Self::Polar(src) => Vector2::from_angle(src.0.get(rl)) * src.1.get(rl),
            Self::Normalize(src) => src.get(rl).normalize(),
            Self::Rotate(src) => Vector2::from_angle(src.1.get(rl)).rotate(src.0.get(rl)),
            Self::Reflect(src) => src.0.get(rl).reflect(src.1.get(rl)),
            Self::MouseWheel => rl.get_mouse_wheel_move_v(),
            Self::Mouse => rl.get_mouse_delta(),
        }
    }
}

impl VectorSource {
    #[inline]
    pub fn normalize(self) -> VectorSource {
        VectorSource::Normalize(Box::new(self))
    }
    #[inline]
    pub fn rotate(self, angle: impl Into<AxisSource>) -> VectorSource {
        VectorSource::Rotate(Box::new((self, angle.into())))
    }
    #[inline]
    pub fn reflect(self, across: impl Into<Self>) -> VectorSource {
        VectorSource::Reflect(Box::new((self, across.into())))
    }

    #[inline]
    pub fn x(self) -> AxisSource {
        AxisSource::X(Box::new(self))
    }
    #[inline]
    pub fn y(self) -> AxisSource {
        AxisSource::Y(Box::new(self))
    }
    #[inline]
    pub fn max_magnitude(self) -> AxisSource {
        AxisSource::MaxMagnitude(Box::new(self))
    }
    #[inline]
    pub fn magnitude(self) -> AxisSource {
        AxisSource::Magnitude(Box::new(self))
    }
    #[inline]
    pub fn dot(self, rhs: impl Into<Self>) -> AxisSource {
        AxisSource::Dot(Box::new((self, rhs.into())))
    }
}

#[derive(Debug)]
pub struct Bindings {
    pub walk: VectorSource,
    pub sprint: EventSource,
    pub jump: EventSource,
    pub look: VectorSource,
    pub next_item: EventSource,
    pub prev_item: EventSource,
}

impl FromStr for Bindings {
    type Err = (); // todo

    fn from_str(_s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}

impl Default for Bindings {
    fn default() -> Self {
        #[allow(unused_imports, clippy::enum_glob_use, reason = "already prefixed")]
        use raylib::prelude::{GamepadAxis::*, GamepadButton::*, KeyboardKey::*, MouseButton::*};
        Self {
            walk: (KEY_D.down() - KEY_A.down())
                .cartesian(KEY_W.down() - KEY_S.down())
                .normalize(),
            sprint: KEY_LEFT_SHIFT.down() | KEY_RIGHT_SHIFT.down(),
            jump: KEY_SPACE.pressed(),
            look: VectorSource::Mouse,
            next_item: VectorSource::MouseWheel.max_magnitude().gt(0.0),
            prev_item: VectorSource::MouseWheel.max_magnitude().lt(0.0),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test0() {
        dbg!(Bindings::default());
    }
}
