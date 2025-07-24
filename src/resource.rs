use crate::rlights::{Light, LightType};
use raylib::prelude::*;

// if you have a better idea, go ahead
#[rustfmt::skip]
pub static PERIODIC_OFFSETS: [(u8, u8); 118] = [
    (0,0),                                                                                                                                                                                                         (31,0),
    (0,1),(1,1),                                                                                                                                                                (26,1),(27,1),(28,1),(29,1),(30,1),(31,1),
    (0,2),(1,2),                                                                                                                                                                (26,2),(27,2),(28,2),(29,2),(30,2),(31,2),
    (0,3),(1,3),                                                                                          (16,3),(17,3),(18,3),(19,3),(20,3),(21,3),(22,3),(23,3),(24,3),(25,3),(26,3),(27,3),(28,3),(29,3),(30,3),(31,3),
    (0,4),(1,4),                                                                                          (16,4),(17,4),(18,4),(19,4),(20,4),(21,4),(22,4),(23,4),(24,4),(25,4),(26,4),(27,4),(28,4),(29,4),(30,4),(31,4),
    (0,5),(1,5),(2,5),(3,5),(4,5),(5,5),(6,5),(7,5),(8,5),(9,5),(10,5),(11,5),(12,5),(13,5),(14,5),(15,5),(16,5),(17,5),(18,5),(19,5),(20,5),(21,5),(22,5),(23,5),(24,5),(25,5),(26,5),(27,5),(28,5),(29,5),(30,5),(31,5),
    (0,6),(1,6),(2,6),(3,6),(4,6),(5,6),(6,6),(7,6),(8,6),(9,6),(10,6),(11,6),(12,6),(13,6),(14,6),(15,6),(16,6),(17,6),(18,6),(19,6),(20,6),(21,6),(22,6),(23,6),(24,6),(25,6),(26,6),(27,6),(28,6),(29,6),(30,6),(31,6),
];

#[derive(Debug)]
pub struct Resources {
    pub skybox: Texture2D,
    pub reactor: Model,
    pub orbital_s: Model,
    pub orbital_p: Model,
    pub orbital_d: Model,
    pub orbital_f: Model,
    pub periodic_table_mesh: Mesh,
    pub periodic_table_mats: [(Matrix, Material); 118],
}

impl Resources {
    #[allow(clippy::too_many_lines, reason = "shut the fuck up")]
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
            periodic_table_mesh: Mesh::gen_mesh_cube(thread, 0.25, 0.25, 0.25),
            periodic_table_mats: {
                PERIODIC_OFFSETS.map(|(col, row)| {
                    let [x, z] = [col, row].map(|x| f32::from(x) * 0.25);
                    #[rustfmt::skip]
                    let matrix = Matrix {
                        m0: 1.0, m4: 0.0, m8:  0.0, m12:   x,
                        m1: 0.0, m5: 1.0, m9:  0.0, m13: 0.0,
                        m2: 0.0, m6: 0.0, m10: 1.0, m14:   z,
                        m3: 0.0, m7: 0.0, m11: 0.0, m15: 1.0,
                    };

                    let image = Image::gen_image_white_noise(128, 128, 0.5);
                    let texture = rl.load_texture_from_image(thread, &image).unwrap();

                    // TODO: lights don't seem to work well if multiple shaders are being loaded.
                    // Need to find a way of reusing the lighting shader...

                    // SAFETY: TBD
                    let mut material =
                        unsafe { Material::from_raw(*rl.load_material_default(thread)) };

                    *material.maps_mut()[MaterialMapIndex::MATERIAL_MAP_ALBEDO as usize]
                        // SAFETY: Material unloads non-default textures
                        .texture_mut() = unsafe { texture.make_weak() };

                    *material.maps_mut()[MaterialMapIndex::MATERIAL_MAP_ALBEDO as usize]
                        .color_mut() = Color::LIGHTGRAY;

                    (matrix, material)
                })
            },
        }
    }
}
