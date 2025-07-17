use std::{cell::Cell, str::FromStr};

use raylib::prelude::*;

pub trait Source: std::fmt::Debug {
    type Type;

    fn get(&self, rl: &RaylibHandle) -> Self::Type;
}

pub trait Adapter: std::fmt::Debug {
    type Output;

    fn get(&self, rl: &RaylibHandle) -> Self::Output;
}

impl<T: Adapter> Source for T {
    type Type = T::Output;

    #[inline]
    fn get(&self, rl: &RaylibHandle) -> Self::Type {
        <T as Adapter>::get(self, rl)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum KeyState {
    Down,
    Released,
    Up,
    Pressed,
    PressedRepeat,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ButtonState {
    Down,
    Released,
    Up,
    Pressed,
}

pub type Gamepad = i32;

#[derive(Debug)]
pub enum EventToEvent<'a> {
    Not(EventSource<'a>),
    And(Vec<EventSource<'a>>),
    Nand(Vec<EventSource<'a>>),
    Or(Vec<EventSource<'a>>),
    Nor(Vec<EventSource<'a>>),
    Xor(EventSource<'a>, EventSource<'a>),
    Xnor(EventSource<'a>, EventSource<'a>),
    Toggle(EventSource<'a>, Cell<bool>),
}

impl Adapter for EventToEvent<'_> {
    type Output = bool;

    fn get(&self, rl: &RaylibHandle) -> bool {
        match self {
            Self::Not(src) => !src.get(rl),
            Self::And(src) => src.iter().all(|src| src.get(rl)),
            Self::Nand(src) => !src.iter().all(|src| src.get(rl)),
            Self::Or(src) => src.iter().any(|src| src.get(rl)),
            Self::Nor(src) => !src.iter().any(|src| src.get(rl)),
            Self::Xor(a, b) => a.get(rl) != b.get(rl),
            Self::Xnor(a, b) => a.get(rl) == b.get(rl),
            Self::Toggle(src, mem) => {
                if src.get(rl) {
                    mem.set(!mem.get());
                }
                mem.get()
            }
        }
    }
}

#[derive(Debug)]
pub enum AxisToEvent<'a> {
    Eq(AxisSource<'a>, AxisSource<'a>, AxisSource<'a>),
    Ne(AxisSource<'a>, AxisSource<'a>, AxisSource<'a>),
    Gt(AxisSource<'a>, AxisSource<'a>),
    Ge(AxisSource<'a>, AxisSource<'a>),
    Lt(AxisSource<'a>, AxisSource<'a>),
    Le(AxisSource<'a>, AxisSource<'a>),
}

impl Adapter for AxisToEvent<'_> {
    type Output = bool;

    fn get(&self, rl: &RaylibHandle) -> bool {
        match self {
            Self::Eq(a, b, epsilon) => (a.get(rl) - b.get(rl)).abs() <= epsilon.get(rl),
            Self::Ne(a, b, epsilon) => (a.get(rl) - b.get(rl)).abs() > epsilon.get(rl),
            Self::Gt(a, b) => a.get(rl) > b.get(rl),
            Self::Ge(a, b) => a.get(rl) >= b.get(rl),
            Self::Lt(a, b) => a.get(rl) < b.get(rl),
            Self::Le(a, b) => a.get(rl) <= b.get(rl),
        }
    }
}

#[derive(Debug)]
pub enum EventSource<'a> {
    Constant(bool),
    Adapter(Box<dyn Adapter<Output = bool> + 'a>),
    KeyboardKey(KeyState, KeyboardKey),
    MouseButton(ButtonState, MouseButton),
    GamepadButton(ButtonState, Gamepad, GamepadButton),
}

impl From<bool> for EventSource<'_> {
    fn from(value: bool) -> Self {
        Self::Constant(value)
    }
}

impl<'a, A: Adapter<Output = bool> + 'a> From<A> for EventSource<'a> {
    fn from(value: A) -> Self {
        Self::Adapter(Box::new(value))
    }
}

impl Source for EventSource<'_> {
    type Type = bool;

    fn get(&self, rl: &RaylibHandle) -> bool {
        match *self {
            Self::Constant(val) => val,
            Self::Adapter(ref src) => src.get(rl),
            Self::KeyboardKey(state, key) => match state {
                KeyState::Down => rl.is_key_down(key),
                KeyState::Released => rl.is_key_released(key),
                KeyState::Up => rl.is_key_up(key),
                KeyState::Pressed => rl.is_key_pressed(key),
                KeyState::PressedRepeat => rl.is_key_pressed_repeat(key),
            },
            Self::MouseButton(state, button) => match state {
                ButtonState::Down => rl.is_mouse_button_down(button),
                ButtonState::Released => rl.is_mouse_button_released(button),
                ButtonState::Up => rl.is_mouse_button_up(button),
                ButtonState::Pressed => rl.is_mouse_button_pressed(button),
            },
            Self::GamepadButton(state, gamepad, button) => match state {
                ButtonState::Down => rl.is_gamepad_button_down(gamepad, button),
                ButtonState::Released => rl.is_gamepad_button_released(gamepad, button),
                ButtonState::Up => rl.is_gamepad_button_up(gamepad, button),
                ButtonState::Pressed => rl.is_gamepad_button_pressed(gamepad, button),
            },
        }
    }
}

#[derive(Debug)]
pub enum EventToAxis<'a> {
    Branch(EventSource<'a>, AxisSource<'a>, AxisSource<'a>),
    Scale(EventSource<'a>, EventSource<'a>, AxisSource<'a>),
}

impl Adapter for EventToAxis<'_> {
    type Output = f32;

    fn get(&self, rl: &RaylibHandle) -> f32 {
        match self {
            Self::Branch(cond, if_true, if_false) => {
                if cond.get(rl) {
                    if_true.get(rl)
                } else {
                    if_false.get(rl)
                }
            }
            Self::Scale(pos, neg, amount) => {
                f32::from(i8::from(pos.get(rl)) - i8::from(neg.get(rl))) * amount.get(rl)
            }
        }
    }
}

#[derive(Debug)]
pub enum AxisToAxis<'a> {
    Neg(AxisSource<'a>),
    Abs(AxisSource<'a>),
    Recip(AxisSource<'a>),
    Product(Vec<AxisSource<'a>>),
    Sum(Vec<AxisSource<'a>>),
}

impl Adapter for AxisToAxis<'_> {
    type Output = f32;

    fn get(&self, rl: &RaylibHandle) -> f32 {
        match self {
            Self::Neg(src) => -src.get(rl),
            Self::Abs(src) => src.get(rl).abs(),
            Self::Recip(src) => src.get(rl).recip(),
            Self::Product(src) => src.iter().map(|src| src.get(rl)).product(),
            Self::Sum(src) => src.iter().map(|src| src.get(rl)).sum(),
        }
    }
}

#[derive(Debug)]
pub enum VectorToAxis<'a> {
    X(VectorSource<'a>),
    Y(VectorSource<'a>),
    MaxMagnitude(VectorSource<'a>),
    Magnitude(VectorSource<'a>),
    Dot(VectorSource<'a>, VectorSource<'a>),
}

impl Adapter for VectorToAxis<'_> {
    type Output = f32;

    fn get(&self, rl: &RaylibHandle) -> f32 {
        match self {
            Self::X(src) => src.get(rl).x,
            Self::Y(src) => src.get(rl).y,
            Self::MaxMagnitude(src) => {
                let val = src.get(rl);
                val[val.abs().max_position()]
            }
            Self::Magnitude(src) => src.get(rl).length(),
            Self::Dot(a, b) => a.get(rl).dot(b.get(rl)),
        }
    }
}

#[derive(Debug)]
pub enum AxisSource<'a> {
    Constant(f32),
    Adapter(Box<dyn Adapter<Output = f32> + 'a>),
    GamepadAxis(Gamepad, GamepadAxis),
}

impl From<f32> for AxisSource<'_> {
    fn from(value: f32) -> Self {
        Self::Constant(value)
    }
}

impl<'a, A: Adapter<Output = f32> + 'a> From<A> for AxisSource<'a> {
    fn from(value: A) -> Self {
        Self::Adapter(Box::new(value))
    }
}

impl Source for AxisSource<'_> {
    type Type = f32;

    fn get(&self, rl: &RaylibHandle) -> f32 {
        match *self {
            Self::Constant(val) => val,
            Self::Adapter(ref src) => src.get(rl),
            Self::GamepadAxis(gamepad, axis) => rl.get_gamepad_axis_movement(gamepad, axis),
        }
    }
}

#[derive(Debug)]
pub enum AxisToVector<'a> {
    Cartesian(AxisSource<'a>, AxisSource<'a>),
    Polar(AxisSource<'a>, AxisSource<'a>),
}

impl Adapter for AxisToVector<'_> {
    type Output = Vector2;

    fn get(&self, rl: &RaylibHandle) -> Vector2 {
        match self {
            Self::Cartesian(x, y) => Vector2::new(x.get(rl), y.get(rl)),
            Self::Polar(angle, radius) => Vector2::from_angle(angle.get(rl)) * radius.get(rl),
        }
    }
}

#[derive(Debug)]
pub enum VectorToVector<'a> {
    Normalize(VectorSource<'a>),
    Rotate(VectorSource<'a>, AxisSource<'a>),
    Reflect(VectorSource<'a>, VectorSource<'a>),
}

impl Adapter for VectorToVector<'_> {
    type Output = Vector2;

    fn get(&self, rl: &RaylibHandle) -> Vector2 {
        match self {
            Self::Normalize(src) => src.get(rl).normalize(),
            Self::Rotate(src, angle) => Vector2::from_angle(angle.get(rl)).rotate(src.get(rl)),
            Self::Reflect(src, across) => src.get(rl).reflect(across.get(rl)),
        }
    }
}

#[derive(Debug)]
pub enum VectorSource<'a> {
    Constant(Vector2),
    Adapter(Box<dyn Adapter<Output = Vector2> + 'a>),
    MouseWheel,
    Mouse,
}

impl From<Vector2> for VectorSource<'_> {
    fn from(value: Vector2) -> Self {
        Self::Constant(value)
    }
}

impl<'a, A: Adapter<Output = Vector2> + 'a> From<A> for VectorSource<'a> {
    fn from(value: A) -> Self {
        Self::Adapter(Box::new(value))
    }
}

impl Source for VectorSource<'_> {
    type Type = Vector2;

    fn get(&self, rl: &RaylibHandle) -> Vector2 {
        match *self {
            Self::Constant(val) => val,
            Self::Adapter(ref src) => src.get(rl),
            Self::MouseWheel => rl.get_mouse_wheel_move_v(),
            Self::Mouse => rl.get_mouse_delta(),
        }
    }
}

#[derive(Debug)]
pub struct Bindings<'a, 'b, 'c, 'd, 'e> {
    pub walk: VectorSource<'a>,
    pub sprint: EventSource<'a>,
    pub jump: EventSource<'b>,
    pub look: VectorSource<'c>,
    pub next_item: EventSource<'d>,
    pub prev_item: EventSource<'e>,
}

impl FromStr for Bindings<'_, '_, '_, '_, '_> {
    type Err = (); // todo

    fn from_str(_s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}

impl Default for Bindings<'_, '_, '_, '_, '_> {
    fn default() -> Self {
        #[allow(unused_imports, clippy::enum_glob_use)]
        use {
            AxisSource::*,
            EventSource::*,
            VectorSource::*,
            raylib::prelude::{GamepadAxis::*, GamepadButton::*, KeyboardKey::*, MouseButton::*},
        };
        Self {
            walk: AxisToVector::Cartesian(
                EventToAxis::Scale(
                    KeyboardKey(KeyState::Down, KEY_D),
                    KeyboardKey(KeyState::Down, KEY_A),
                    1.0.into(),
                )
                .into(),
                EventToAxis::Scale(
                    KeyboardKey(KeyState::Down, KEY_W),
                    KeyboardKey(KeyState::Down, KEY_S),
                    1.0.into(),
                )
                .into(),
            )
            .into(),
            sprint: KeyboardKey(KeyState::Pressed, KEY_SPACE),
            jump: KeyboardKey(KeyState::Pressed, KEY_SPACE),
            look: Mouse,
            next_item: AxisToEvent::Gt(VectorToAxis::MaxMagnitude(MouseWheel).into(), 0.0.into())
                .into(),
            prev_item: AxisToEvent::Lt(VectorToAxis::MaxMagnitude(MouseWheel).into(), 0.0.into())
                .into(),
        }
    }
}
