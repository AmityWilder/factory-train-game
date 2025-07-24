//! raylib.lights - Some useful functions to deal with lights data
//!
#![allow(clippy::doc_markdown, reason = "that's not a type, dumbass")]
//! Rewritten in Rust by Amy Wilder (@AmityWilder)
//!
//! LICENSE: zlib/libpng
//!
//! Copyright (c) 2017-2024 Victor Fisac (@victorfisac) and Ramon Santamaria (@raysan5)
//!
//! This software is provided "as-is", without any express or implied warranty. In no event
//! will the authors be held liable for any damages arising from the use of this software.
//!
//! Permission is granted to anyone to use this software for any purpose, including commercial
//! applications, and to alter it and redistribute it freely, subject to the following restrictions:
//!
//! 1. The origin of this software must not be misrepresented; you must not claim that you
//!    wrote the original software. If you use this software in a product, an acknowledgment
//!    in the product documentation would be appreciated but is not required.
//!
//! 2. Altered source versions must be plainly marked as such, and must not be misrepresented
//!    as being the original software.
//!
//! 3. This notice may not be removed or altered from any source distribution.

//----------------------------------------------------------------------------------
// Defines and Macros
//----------------------------------------------------------------------------------
/// Max dynamic lights supported by shader
pub const MAX_LIGHTS: usize = 32;

//----------------------------------------------------------------------------------
// Types and Structures Definition
//----------------------------------------------------------------------------------

/// Light data
#[derive(Debug, Clone)]
pub struct Light {
    pub ty: LightType,
    pub enabled: bool,
    pub position: Vector3,
    pub target: Vector3,
    pub color: Color,
    // pub attenuation: f32,

    // Shader locations
    enabled_loc: i32,
    type_loc: i32,
    position_loc: i32,
    target_loc: i32,
    color_loc: i32,
    // attenuation_loc: i32,
}

/// Light type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LightType {
    Directional = 0,
    Point,
}

/***********************************************************************************
*
*   RLIGHTS IMPLEMENTATION
*
************************************************************************************/

use std::sync::atomic::{AtomicUsize, Ordering};

use raylib::prelude::*;

//----------------------------------------------------------------------------------
// Defines and Macros
//----------------------------------------------------------------------------------
// ...

//----------------------------------------------------------------------------------
// Types and Structures Definition
//----------------------------------------------------------------------------------
// ...

//----------------------------------------------------------------------------------
// Global Variables Definition
//----------------------------------------------------------------------------------
/// Current amount of created lights
static LIGHTS_COUNT: AtomicUsize = AtomicUsize::new(0);

//----------------------------------------------------------------------------------
// Module specific Functions Declaration
//----------------------------------------------------------------------------------
// ...

//----------------------------------------------------------------------------------
// Module Functions Definition
//----------------------------------------------------------------------------------

impl Light {
    /// Create a light and get shader locations
    #[must_use]
    pub fn new(
        ty: LightType,
        position: Vector3,
        target: Vector3,
        color: Color,
        shader: &mut Shader,
    ) -> Option<Light> {
        if let Ok(light_index) =
            LIGHTS_COUNT.fetch_update(Ordering::SeqCst, Ordering::SeqCst, |light_count| {
                (light_count < MAX_LIGHTS).then(|| light_count + 1)
            })
        {
            let mut light = Light {
                ty,
                enabled: true,
                position,
                target,
                color,
                // attenuation: 0.0,

                // NOTE: Lighting shader naming must be the provided ones
                enabled_loc: shader.get_shader_location(&format!("lights[{light_index}].enabled")),
                type_loc: shader.get_shader_location(&format!("lights[{light_index}].type")),
                position_loc: shader
                    .get_shader_location(&format!("lights[{light_index}].position")),
                target_loc: shader.get_shader_location(&format!("lights[{light_index}].target")),
                color_loc: shader.get_shader_location(&format!("lights[{light_index}].color")),
                // attenuation_loc: 0,
            };

            light.update_light_values(shader);

            Some(light)
        } else {
            None
        }
    }

    /// Send light properties to shader
    /// NOTE: Light shader locations should be available
    pub fn update_light_values(&mut self, shader: &mut Shader) {
        // Send to shader light enabled state and type
        shader.set_shader_value(self.enabled_loc, i32::from(self.enabled));
        shader.set_shader_value(self.type_loc, self.ty as i32);

        // Send to shader self position values
        shader.set_shader_value(self.position_loc, self.position);

        // Send to shader self target position values
        shader.set_shader_value(self.target_loc, self.target);

        // Send to shader self color values
        shader.set_shader_value(
            self.color_loc,
            Vector4::new(
                f32::from(self.color.r) / 255.0,
                f32::from(self.color.g) / 255.0,
                f32::from(self.color.b) / 255.0,
                f32::from(self.color.a) / 255.0,
            ),
        );
    }
}
