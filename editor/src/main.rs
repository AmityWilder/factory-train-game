#![feature(
    likely_unlikely,
    cold_path,
    array_windows,
    iter_array_chunks,
    sized_hierarchy,
    unsize,
    coerce_unsized
)]

use arrayvec::ArrayVec;
use raylib::{math::glam::Quat, prelude::*};
use std::sync::OnceLock;
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter, IntoStaticStr};

use crate::mesh::AmyMesh;

mod mesh;

pub trait DropdownEnum:
    'static + Sized + Copy + Eq + std::hash::Hash + IntoEnumIterator + Into<&'static str>
{
    #[must_use]
    fn dropdown_list() -> &'static str {
        static DROPDOWN_LIST: OnceLock<String> = OnceLock::new();
        DROPDOWN_LIST
            .get_or_init(|| {
                Self::iter()
                    .map(<Self as Into<&'static str>>::into)
                    .collect::<Vec<&str>>()
                    .join(";")
            })
            .as_str()
    }
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
        T::iter()
            .map(|var| font.measure_text(var.into(), font_size, spacing))
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

    /// Returns `true` on value change
    pub fn update(&mut self, d: &mut impl RaylibDraw) -> bool {
        // SAFETY: T::iter() contains every variant, and self.value is T which must be a variant.
        let value_index = unsafe { T::iter().position(|v| v == self.value).unwrap_unchecked() }
            .try_into()
            .expect("dropdown enum should not exceed i32::MAX variants");
        let mut new_index = value_index;
        let is_editing = self.is_editing;
        let toggle_editing =
            d.gui_dropdown_box(self.bounds, T::dropdown_list(), &mut new_index, is_editing);
        if
        // dropdowns only toggle for one tick and then spend dozens or hundreds of ticks retaining the new state
        std::hint::unlikely(toggle_editing)
            && std::mem::replace(&mut self.is_editing, !is_editing)
            // people dont usually open the dropdown just to leave it unchanged
            && std::hint::likely(value_index != new_index)
        {
            self.value = T::iter()
                .nth(
                    new_index
                        .try_into()
                        .expect("gui_dropdown_box should not assign a negative to `active`"),
                )
                .expect("gui_dropdown_box should assign a valid variant to `active`");
            true
        } else {
            false
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, IntoStaticStr, Display, EnumIter)]
pub enum EditorMode {
    #[default]
    Vertex = 1,
    Edge,
    Border,
    Face,
    Mesh,
    Object,
}

impl DropdownEnum for EditorMode {}

pub enum MaterialMapIndex {
    Albedo,
    Metalness,
    Normal,
    Roughness,
    Occlusion,
    Emission,
    Height,
    Cubemap,
    Irradiance,
    Prefilter,
    Brdf,
}

pub trait MaterialExt: RaylibMaterial {
    #[inline]
    fn material_map(&self, index: MaterialMapIndex) -> &MaterialMap {
        &self.maps()[index as usize]
    }

    #[inline]
    fn material_map_mut(&mut self, index: MaterialMapIndex) -> &mut MaterialMap {
        &mut self.maps_mut()[index as usize]
    }
}

impl<T: RaylibMaterial> MaterialExt for T {}

fn main() {
    use {KeyboardKey::*, MouseButton::*};

    let (mut rl, thread) = init().title("editor").resizable().build();

    rl.set_target_fps(120);

    let mut mode_dropdown = Dropdown::new(
        Rectangle::new(10.0, 10.0, 80.0, 20.0),
        EditorMode::default(),
    );

    let mut asset = AmyMesh::gen_cube(1.0, 1.0, 1.0);
    let mut camera =
        Camera::perspective(Vector3::new(2.0, 2.0, 2.0), Vector3::ZERO, Vector3::Y, 45.0);
    let mut material = rl.load_material_default(&thread);
    *material
        .material_map_mut(MaterialMapIndex::Albedo)
        .color_mut() = Color::LIGHTGRAY;

    let mut keys_pressed = Vec::new();

    while !rl.window_should_close() {
        let dt = rl.get_frame_time();

        keys_pressed.clear();
        keys_pressed.extend(std::iter::from_fn(|| rl.get_key_pressed()));

        if let Some(pos) = keys_pressed.iter().rposition(|&key| {
            const KEYCODE_ONE: i32 = KEY_ONE as i32;
            const KEYCODE_SIX: i32 = KEY_SIX as i32;
            matches!(key as i32, KEYCODE_ONE..=KEYCODE_SIX)
        }) {
            // the majority of ticks will have no buttons pressed, and even fewer will be for these hotkeys specifically.
            std::hint::cold_path();
            mode_dropdown.value = match keys_pressed.remove(pos) {
                KEY_ONE => EditorMode::Vertex,
                KEY_TWO => EditorMode::Edge,
                KEY_THREE => EditorMode::Border,
                KEY_FOUR => EditorMode::Face,
                KEY_FIVE => EditorMode::Mesh,
                KEY_SIX => EditorMode::Object,
                _ => unreachable!(),
            };
        }

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

            let mut face = ArrayVec::<&Vector3, 5>::new();
            for verts in asset.face_vertices() {
                face.clear();
                face.try_extend_from_slice(verts.as_slice())
                    .expect("faces do not exceed 4 vertices");
                face.push(verts[0]);
                for &world_positions in face.array_windows::<2>() {
                    let [start_screen, end_screen] =
                        world_positions.map(|v| d.get_world_to_screen(*v, camera));
                    d.draw_line_v(start_screen, end_screen, Color::YELLOW);
                }
            }
        }

        // 3D 2D overlay

        {
            let vert_extent: f32 = 3.0;

            let mut square = Rectangle {
                width: vert_extent * 2.0,
                height: vert_extent * 2.0,
                ..Default::default()
            };

            match mode_dropdown.value {
                EditorMode::Vertex => {
                    for &vert_world in asset.vertices() {
                        let vert_screen = d.get_world_to_screen(vert_world, camera);
                        square.x = vert_screen.x - vert_extent;
                        square.y = vert_screen.y - vert_extent;
                        d.draw_rectangle_rec(square, Color::YELLOW);
                    }
                }
                EditorMode::Edge => {
                    let mut face = ArrayVec::<&Vector3, 5>::new();
                    for verts in asset.face_vertices() {
                        face.clear();
                        face.try_extend_from_slice(verts.as_slice())
                            .expect("faces do not exceed 4 vertices");
                        face.push(verts[0]);
                        for &world_positions in face.array_windows::<2>() {
                            let [start_screen, end_screen] =
                                world_positions.map(|v| d.get_world_to_screen(*v, camera));
                            d.draw_line_v(start_screen, end_screen, Color::YELLOW);
                        }
                    }
                }
                EditorMode::Border => {
                    // todo
                }
                EditorMode::Face => {
                    // todo
                }
                EditorMode::Mesh => {
                    // todo
                }
                EditorMode::Object => {
                    // todo
                }
            }

            // origin
            d.draw_ring(
                d.get_world_to_screen(camera.target, camera),
                6.0,
                9.0,
                0.0,
                360.0,
                10,
                Color::GRAY,
            );
        }

        // UI
        mode_dropdown.update(&mut d);

        d.draw_fps(0, 400);
    }

    unsafe {
        rl.unload_material(&thread, material);
    }
}
