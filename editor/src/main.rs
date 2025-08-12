use raylib::{
    math::glam::{EulerRot, Quat},
    prelude::*,
};

pub trait DropdownEnum:
    'static + Sized + Copy + Eq + std::hash::Hash + Into<i32> + TryFrom<i32, Error: std::fmt::Debug>
{
    const LIST: &[Self];
    const DROPDOWN_LIST: &str;

    #[must_use]
    fn as_str(self) -> &'static str;
}

macro_rules! concat_skip_first {
    (";", $($token:tt)*) => {
        concat!($($token)*)
    };
}

macro_rules! variant_name {
    ($Variant:ident = $vname:literal) => {
        $vname
    };

    ($Variant:ident) => {
        stringify!($Variant)
    };
}

macro_rules! dropdown_enum {
    (
        $(#[$m:meta])*
        $vis:vis enum $Enum:ident {
            $(
            $(#[$vm:meta])*
            $Variant:ident$( = $vname:literal)?
            ),* $(,)?
        }
    ) => {
        $(#[$m])*
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        $vis enum $Enum {
            $(
            $(#[$vm])*
            $Variant
            ),*
        }

        impl std::fmt::Display for $Enum {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                self.as_str().fmt(f)
            }
        }

        impl $Enum {
            pub const LIST: [Self; [$(Self::$Variant),*].len()] = [$(Self::$Variant),*];
        }

        impl DropdownEnum for $Enum {
            const LIST: &[Self] = &Self::LIST;
            const DROPDOWN_LIST: &str = concat_skip_first!($(";", variant_name!($Variant$( = $vname)?)),*);

            fn as_str(self) -> &'static str {
                match self { $(Self::$Variant => variant_name!($Variant$( = $vname)?)),* }
            }
        }

        impl TryFrom<i32> for $Enum {
            type Error = ();

            fn try_from(value: i32) -> Result<Self, Self::Error> {
                #![allow(non_upper_case_globals)]
                $(const $Variant: i32 = $Enum::$Variant as i32;)*
                match value {
                    $($Variant => std::result::Result::Ok(Self::$Variant),)*
                    _ => std::result::Result::Err(()),
                }
            }
        }

        impl From<$Enum> for i32 {
            #[inline]
            fn from(value: $Enum) -> Self {
                value as i32
            }
        }
    };
}

#[derive(Debug)]
pub struct Dropdown<T> {
    pub bounds: Rectangle,
    pub value: T,
    pub is_editing: bool,
}

impl<T: DropdownEnum> Dropdown<T> {
    #[must_use]
    pub const fn new(bounds: Rectangle, value: T) -> Self {
        Self {
            bounds,
            value,
            is_editing: false,
        }
    }

    #[must_use]
    fn fit_size(font: &Font, font_size: f32, spacing: f32) -> Option<Vector2> {
        T::LIST
            .iter()
            .map(|var| font.measure_text(var.as_str(), font_size, spacing))
            .reduce(|acc, v| Vector2::new(acc.x.max(v.x), acc.y.max(v.y)))
    }

    #[must_use]
    pub fn new_fitted(
        font: &Font,
        font_size: f32,
        spacing: f32,
        position: Vector2,
        value: T,
    ) -> Option<Self> {
        Self::fit_size(font, font_size, spacing).map(|size| Self {
            bounds: Rectangle {
                x: position.x,
                y: position.y,
                width: size.x,
                height: size.y,
            },
            value,
            is_editing: false,
        })
    }

    #[must_use]
    pub fn fit(&mut self, font: &Font, font_size: f32, spacing: f32) -> Option<()> {
        let size = Self::fit_size(font, font_size, spacing)?;
        self.bounds.width = size.x;
        self.bounds.height = size.y;
        Some(())
    }

    /// Returns `true` on change
    pub fn update(&mut self, d: &mut impl RaylibDraw) -> bool {
        let mut temp: i32 = self.value.into();
        if d.gui_dropdown_box(self.bounds, T::DROPDOWN_LIST, &mut temp, self.is_editing) {
            self.is_editing = !self.is_editing;
            if !self.is_editing && (self.value.into() != temp) {
                self.value = temp.try_into().unwrap();
                return true;
            }
        }
        false
    }
}

dropdown_enum! {
    pub enum EditorMode {
        Apple,
        Orange,
        Banana,
        Mango,
        FooBar = "Foo Bar",
    }
}

fn main() {
    use {KeyboardKey::*, MouseButton::*};

    let (mut rl, thread) = init().title("editor").resizable().build();

    let mut mode = Dropdown::new(Rectangle::new(10.0, 10.0, 80.0, 20.0), EditorMode::Apple);

    let mut asset = Mesh::gen_mesh_cube(&thread, 1.0, 1.0, 1.0);
    let mut camera =
        Camera::perspective(Vector3::new(2.0, 2.0, 2.0), Vector3::ZERO, Vector3::Y, 45.0);
    let material = rl.load_material_default(&thread);

    while !rl.window_should_close() {
        let dt = rl.get_frame_time();

        if rl.is_mouse_button_pressed(MOUSE_BUTTON_MIDDLE) {
            rl.disable_cursor();
        } else if rl.is_mouse_button_released(MOUSE_BUTTON_MIDDLE) {
            rl.enable_cursor();
        }

        let scroll = rl.get_mouse_wheel_move();
        let forward = camera.forward() * scroll * 100.0 * dt;
        if scroll < 0.0
            || camera.position.distance_squared(camera.target)
                > camera.position.distance_squared(camera.position + forward)
        {
            camera.position += forward;
        }
        if rl.is_mouse_button_down(MOUSE_BUTTON_MIDDLE) {
            let orbit_speed = 1.0;
            let pan_speed = 1.0;
            let mouse_move = rl.get_mouse_delta();
            if rl.is_key_down(KEY_LEFT_ALT) {
                // orbit
                let rotation = Quat::from_axis_angle(camera.up(), -mouse_move.x * orbit_speed * dt)
                    * Quat::from_axis_angle(
                        camera.up().cross(camera.forward()),
                        mouse_move.y * orbit_speed * dt,
                    );
                camera.position = camera.target + rotation * (camera.position - camera.target);
            } else {
                // pan
                camera.move_right(-mouse_move.x * pan_speed * std::f32::consts::PI * dt, true);
                camera.move_up(mouse_move.y * pan_speed * std::f32::consts::PI * dt);
            }
        }

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::new(24, 24, 24, 255));

        // 3D scene
        {
            let mut d = d.begin_mode3D(camera);
            d.draw_grid(10, 1.0);
            d.draw_mesh(&mut asset, material.clone(), Matrix::identity());
        }

        d.draw_ring(
            d.get_world_to_screen(camera.target, camera),
            6.0,
            9.0,
            0.0,
            360.0,
            10,
            Color::GRAY,
        );

        // UI
        mode.update(&mut d);
    }

    unsafe {
        rl.unload_material(&thread, material);
    }
}
