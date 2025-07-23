use crate::rlights::{Light, LightType};
use raylib::prelude::*;

#[derive(Debug)]
pub struct Resources {
    pub skybox: Texture2D,
    pub reactor: Model,
    pub orbital_s: Model,
    pub orbital_p: Model,
    pub orbital_d: Model,
    pub orbital_f: Model,
}

impl Resources {
    pub fn new(rl: &mut RaylibHandle, thread: &RaylibThread) -> Self {
        Self {
            skybox: {
                let image = Image::gen_image_gradient_radial(
                    256,
                    256,
                    0.1,
                    Color::DODGERBLUE,
                    Color::CORAL,
                );
                rl.load_texture_from_image(thread, &image).unwrap()
            },
            reactor: {
                // Mesh
                let mesh = Mesh::gen_mesh_cube(thread, 2.0, 2.0, 3.0);

                let mut mat = rl.load_material_default(thread);

                // Shader
                let mut shader = rl.load_shader_from_memory(
                    thread,
                    Some(include_str!("../assets/lighting.vs")),
                    Some(include_str!("../assets/lighting.fs")),
                );
                assert!(shader.is_shader_valid());
                shader.set_shader_value(
                    shader.get_shader_location("ambient"),
                    Vector4::new(0.2, 0.2, 0.2, 1.0),
                );
                Light::new(
                    LightType::Directional,
                    Vector3::new(0.0, 50.0, 0.0),
                    Vector3::ZERO,
                    Color::WHITE,
                    &mut shader,
                )
                .unwrap();
                // SAFETY: Material unloads non-default shader on its own
                *mat.shader_mut() = unsafe { shader.make_weak() };

                // Color
                *mat.maps_mut()[MaterialMapIndex::MATERIAL_MAP_ALBEDO as usize].color_mut() =
                    Color::GRAY;

                // Texture
                let image =
                    Image::gen_image_gradient_linear(64, 64, 0, Color::GRAY, Color::LIGHTGRAY);
                let texture = rl.load_texture_from_image(thread, &image).unwrap();
                // SAFETY: Material unloads non-default textures on its own
                mat.set_material_texture(MaterialMapIndex::MATERIAL_MAP_ALBEDO, unsafe {
                    texture.make_weak()
                });
                assert!(mat.is_material_valid());

                // SAFETY: Model unloads meshes on its own
                let mut model = rl
                    .load_model_from_mesh(thread, unsafe { mesh.make_weak() })
                    .unwrap();
                model.materials_mut()[0] = mat;
                model.transform = Matrix::translate(1.0, 1.0, 1.5).into();

                assert!(model.is_model_valid());
                model
            },
            orbital_s: {
                let mesh = Mesh::gen_mesh_sphere(thread, 1.0, 10, 10);
                let mut material = rl.load_material_default(thread);
                *material.maps_mut()[MaterialMapIndex::MATERIAL_MAP_ALBEDO as usize].color_mut() =
                    Color::BLUE;
                // SAFETY: Model unloads meshes on its own
                let mut model = rl
                    .load_model_from_mesh(thread, unsafe { mesh.make_weak() })
                    .unwrap();
                model.materials_mut()[0] = material;
                model.transform = Matrix::identity().into();
                model
            },
            orbital_p: {
                let mesh = Mesh::gen_mesh_cube(thread, 1.0, 1.0, 1.0); // TODO
                let mut material = rl.load_material_default(thread);
                *material.maps_mut()[MaterialMapIndex::MATERIAL_MAP_ALBEDO as usize].color_mut() =
                    Color::MAGENTA;
                // SAFETY: Model unloads meshes on its own
                let mut model = rl
                    .load_model_from_mesh(thread, unsafe { mesh.make_weak() })
                    .unwrap();
                model.materials_mut()[0] = material;
                model.transform = Matrix::identity().into();
                model
            },
            orbital_d: {
                let mesh = Mesh::gen_mesh_cube(thread, 1.0, 1.0, 1.0); // TODO
                let mut material = rl.load_material_default(thread);
                *material.maps_mut()[MaterialMapIndex::MATERIAL_MAP_ALBEDO as usize].color_mut() =
                    Color::MAGENTA;
                // SAFETY: Model unloads meshes on its own
                let mut model = rl
                    .load_model_from_mesh(thread, unsafe { mesh.make_weak() })
                    .unwrap();
                model.materials_mut()[0] = material;
                model.transform = Matrix::identity().into();
                model
            },
            orbital_f: {
                let mesh = Mesh::gen_mesh_cube(thread, 1.0, 1.0, 1.0); // TODO
                let mut material = rl.load_material_default(thread);
                *material.maps_mut()[MaterialMapIndex::MATERIAL_MAP_ALBEDO as usize].color_mut() =
                    Color::MAGENTA;
                // SAFETY: Model unloads meshes on its own
                let mut model = rl
                    .load_model_from_mesh(thread, unsafe { mesh.make_weak() })
                    .unwrap();
                model.materials_mut()[0] = material;
                model.transform = Matrix::identity().into();
                model
            },
        }
    }
}
