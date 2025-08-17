use raylib::prelude::*;

pub trait DynRaylibDraw3D {
    /// Draw a point in 3D space, actually a small line
    #[allow(non_snake_case, reason = "consistency")]
    fn draw_point3D(&mut self, position: Vector3, color: Color);

    /// Draw a color-filled triangle (vertex in counter-clockwise order!)
    #[allow(non_snake_case, reason = "consistency")]
    fn draw_triangle3D(&mut self, v1: Vector3, v2: Vector3, v3: Vector3, color: Color);

    /// Draw a triangle strip defined by points
    #[allow(non_snake_case, reason = "consistency")]
    fn draw_triangle_strip3D(&mut self, points: &[Vector3], color: Color);

    /// Draws a line in 3D world space.
    #[allow(non_snake_case, reason = "consistency")]
    fn draw_line3D(&mut self, start_pos: Vector3, end_pos: Vector3, color: Color);

    /// Draws a circle in 3D world space.
    #[allow(non_snake_case, reason = "consistency")]
    fn draw_circle3D(
        &mut self,
        center: Vector3,
        radius: f32,
        rotation_axis: Vector3,
        rotation_angle: f32,
        color: Color,
    );

    /// Draws a cube.
    fn draw_cube(&mut self, position: Vector3, width: f32, height: f32, length: f32, color: Color);

    /// Draws a cube (Vector version).
    fn draw_cube_v(&mut self, position: Vector3, size: Vector3, color: Color);

    /// Draws a cube in wireframe.
    fn draw_cube_wires(
        &mut self,
        position: Vector3,
        width: f32,
        height: f32,
        length: f32,
        color: Color,
    );

    /// Draws a cube in wireframe. (Vector Version)
    fn draw_cube_wires_v(&mut self, position: Vector3, size: Vector3, color: Color);

    /// Draw a 3d mesh with material and transform
    fn draw_mesh(&mut self, mesh: ffi::Mesh, material: ffi::Material, transform: Matrix);

    /// Draw multiple mesh instances with material and different transforms
    fn draw_mesh_instanced(
        &mut self,
        mesh: ffi::Mesh,
        material: ffi::Material,
        transforms: &[Matrix],
    );

    /// Draws a sphere.
    fn draw_sphere(&mut self, center_pos: Vector3, radius: f32, color: Color);

    /// Draws a sphere with extended parameters.
    fn draw_sphere_ex(
        &mut self,
        center_pos: Vector3,
        radius: f32,
        rings: i32,
        slices: i32,
        color: Color,
    );

    /// Draws a sphere in wireframe.
    fn draw_sphere_wires(
        &mut self,
        center_pos: Vector3,
        radius: f32,
        rings: i32,
        slices: i32,
        color: Color,
    );

    /// Draws a cylinder.
    fn draw_cylinder(
        &mut self,
        position: Vector3,
        radius_top: f32,
        radius_bottom: f32,
        height: f32,
        slices: i32,
        color: Color,
    );

    /// Draws a cylinder with extended parameters.
    fn draw_cylinder_ex(
        &mut self,
        start_position: Vector3,
        end_position: Vector3,
        radius_start: f32,
        radius_end: f32,
        slices: i32,
        color: Color,
    );

    /// Draws a cylinder in wireframe.
    fn draw_cylinder_wires(
        &mut self,
        position: Vector3,
        radius_top: f32,
        radius_bottom: f32,
        height: f32,
        slices: i32,
        color: Color,
    );

    /// Draws a cylinder in wireframe with extended parameters.
    fn draw_cylinder_wires_ex(
        &mut self,
        start_position: Vector3,
        end_position: Vector3,
        radius_start: f32,
        radius_end: f32,
        slices: i32,
        color: Color,
    );

    /// Draw capsule with the center of its sphere caps at startPos and endPos
    fn draw_capsule(
        &mut self,
        start_pos: Vector3,
        end_pos: Vector3,
        radius: f32,
        slices: i32,
        rings: i32,
        color: Color,
    );

    ///Draw capsule wireframe with the center of its sphere caps at startPos and endPos
    fn draw_capsule_wires(
        &mut self,
        start_pos: Vector3,
        end_pos: Vector3,
        radius: f32,
        slices: i32,
        rings: i32,
        color: Color,
    );

    /// Draws an X/Z plane.
    fn draw_plane(&mut self, center_pos: Vector3, size: Vector2, color: Color);

    /// Draws a ray line.
    fn draw_ray(&mut self, ray: Ray, color: Color);

    /// Draws a grid (centered at (0, 0, 0)).
    fn draw_grid(&mut self, slices: i32, spacing: f32);

    /// Draws a model (with texture if set).
    fn draw_model(&mut self, model: ffi::Model, position: Vector3, scale: f32, tint: Color);

    /// Draws a model with extended parameters.
    fn draw_model_ex(
        &mut self,
        model: ffi::Model,
        position: Vector3,
        rotation_axis: Vector3,
        rotation_angle: f32,
        scale: Vector3,
        tint: Color,
    );

    /// Draws a model with wires (with texture if set).
    fn draw_model_wires(&mut self, model: ffi::Model, position: Vector3, scale: f32, tint: Color);

    /// Draws a model with wires.
    fn draw_model_wires_ex(
        &mut self,
        model: ffi::Model,
        position: Vector3,
        rotation_axis: Vector3,
        rotation_angle: f32,
        scale: Vector3,
        tint: Color,
    );

    /// Draws a bounding box (wires).
    fn draw_bounding_box(&mut self, bbox: BoundingBox, color: Color);

    /// Draws a billboard texture.
    fn draw_billboard(
        &mut self,
        camera: Camera3D,
        texture: &Texture2D,
        center: Vector3,
        size: f32,
        tint: Color,
    );

    /// Draws a billboard texture defined by `source_rec`.
    fn draw_billboard_rec(
        &mut self,
        camera: Camera3D,
        texture: &Texture2D,
        source_rec: Rectangle,
        center: Vector3,
        size: Vector2,
        tint: Color,
    );

    /// Draw a billboard texture defined by source and rotation
    #[allow(clippy::too_many_arguments, reason = "all are needed")]
    fn draw_billboard_pro(
        &mut self,
        camera: Camera,
        texture: ffi::Texture2D,
        source: Rectangle,
        position: Vector3,
        up: Vector3,
        size: Vector2,
        origin: Vector2,
        rotation: f32,
        tint: Color,
    );

    /// Draw a model as points
    fn draw_model_points(&mut self, model: ffi::Model, position: Vector3, scale: f32, tint: Color);

    /// Draw a model as points with extended parameters
    fn draw_model_points_ex(
        &mut self,
        model: ffi::Model,
        position: Vector3,
        rotation_axis: Vector3,
        angle: f32,
        scale: Vector3,
        tint: Color,
    );
}

impl<D: RaylibDraw3D> DynRaylibDraw3D for D {
    #[inline]
    fn draw_point3D(&mut self, position: Vector3, color: Color) {
        self.draw_point3D(position, color);
    }

    #[inline]
    fn draw_triangle3D(&mut self, v1: Vector3, v2: Vector3, v3: Vector3, color: Color) {
        self.draw_triangle3D(v1, v2, v3, color);
    }

    #[inline]
    fn draw_triangle_strip3D(&mut self, points: &[Vector3], color: Color) {
        self.draw_triangle_strip3D(points, color);
    }

    #[inline]
    fn draw_line3D(&mut self, start_pos: Vector3, end_pos: Vector3, color: Color) {
        self.draw_line3D(start_pos, end_pos, color);
    }

    #[inline]
    fn draw_circle3D(
        &mut self,
        center: Vector3,
        radius: f32,
        rotation_axis: Vector3,
        rotation_angle: f32,
        color: Color,
    ) {
        self.draw_circle3D(center, radius, rotation_axis, rotation_angle, color);
    }

    #[inline]
    fn draw_cube(&mut self, position: Vector3, width: f32, height: f32, length: f32, color: Color) {
        self.draw_cube(position, width, height, length, color);
    }

    #[inline]
    fn draw_cube_v(&mut self, position: Vector3, size: Vector3, color: Color) {
        self.draw_cube_v(position, size, color);
    }

    #[inline]
    fn draw_cube_wires(
        &mut self,
        position: Vector3,
        width: f32,
        height: f32,
        length: f32,
        color: Color,
    ) {
        self.draw_cube_wires(position, width, height, length, color);
    }

    #[inline]
    fn draw_cube_wires_v(&mut self, position: Vector3, size: Vector3, color: Color) {
        self.draw_cube_wires_v(position, size, color);
    }

    #[inline]
    fn draw_mesh(&mut self, mesh: ffi::Mesh, material: ffi::Material, transform: Matrix) {
        // SAFETY: TBD
        self.draw_mesh(
            unsafe { WeakMesh::from_raw(mesh) },
            unsafe { WeakMaterial::from_raw(material) },
            transform,
        );
    }

    #[inline]
    fn draw_mesh_instanced(
        &mut self,
        mesh: ffi::Mesh,
        material: ffi::Material,
        transforms: &[Matrix],
    ) {
        // SAFETY: TBD
        self.draw_mesh_instanced(
            unsafe { WeakMesh::from_raw(mesh) },
            unsafe { WeakMaterial::from_raw(material) },
            transforms,
        );
    }

    #[inline]
    fn draw_sphere(&mut self, center_pos: Vector3, radius: f32, color: Color) {
        self.draw_sphere(center_pos, radius, color);
    }

    #[inline]
    fn draw_sphere_ex(
        &mut self,
        center_pos: Vector3,
        radius: f32,
        rings: i32,
        slices: i32,
        color: Color,
    ) {
        self.draw_sphere_ex(center_pos, radius, rings, slices, color);
    }

    #[inline]
    fn draw_sphere_wires(
        &mut self,
        center_pos: Vector3,
        radius: f32,
        rings: i32,
        slices: i32,
        color: Color,
    ) {
        self.draw_sphere_wires(center_pos, radius, rings, slices, color);
    }

    #[inline]
    fn draw_cylinder(
        &mut self,
        position: Vector3,
        radius_top: f32,
        radius_bottom: f32,
        height: f32,
        slices: i32,
        color: Color,
    ) {
        self.draw_cylinder(position, radius_top, radius_bottom, height, slices, color);
    }

    #[inline]
    fn draw_cylinder_ex(
        &mut self,
        start_position: Vector3,
        end_position: Vector3,
        radius_start: f32,
        radius_end: f32,
        slices: i32,
        color: Color,
    ) {
        self.draw_cylinder_ex(
            start_position,
            end_position,
            radius_start,
            radius_end,
            slices,
            color,
        );
    }

    #[inline]
    fn draw_cylinder_wires(
        &mut self,
        position: Vector3,
        radius_top: f32,
        radius_bottom: f32,
        height: f32,
        slices: i32,
        color: Color,
    ) {
        self.draw_cylinder_wires(position, radius_top, radius_bottom, height, slices, color);
    }

    #[inline]
    fn draw_cylinder_wires_ex(
        &mut self,
        start_position: Vector3,
        end_position: Vector3,
        radius_start: f32,
        radius_end: f32,
        slices: i32,
        color: Color,
    ) {
        self.draw_cylinder_wires_ex(
            start_position,
            end_position,
            radius_start,
            radius_end,
            slices,
            color,
        );
    }

    #[inline]
    fn draw_capsule(
        &mut self,
        start_pos: Vector3,
        end_pos: Vector3,
        radius: f32,
        slices: i32,
        rings: i32,
        color: Color,
    ) {
        self.draw_capsule(start_pos, end_pos, radius, slices, rings, color);
    }

    #[inline]
    fn draw_capsule_wires(
        &mut self,
        start_pos: Vector3,
        end_pos: Vector3,
        radius: f32,
        slices: i32,
        rings: i32,
        color: Color,
    ) {
        self.draw_capsule_wires(start_pos, end_pos, radius, slices, rings, color);
    }

    #[inline]
    fn draw_plane(&mut self, center_pos: Vector3, size: Vector2, color: Color) {
        self.draw_plane(center_pos, size, color);
    }

    #[inline]
    fn draw_ray(&mut self, ray: Ray, color: Color) {
        self.draw_ray(ray, color);
    }

    #[inline]
    fn draw_grid(&mut self, slices: i32, spacing: f32) {
        self.draw_grid(slices, spacing);
    }

    #[inline]
    fn draw_model(&mut self, model: ffi::Model, position: Vector3, scale: f32, tint: Color) {
        // SAFETY: TBD
        self.draw_model(unsafe { WeakModel::from_raw(model) }, position, scale, tint);
    }

    #[inline]
    fn draw_model_ex(
        &mut self,
        model: ffi::Model,
        position: Vector3,
        rotation_axis: Vector3,
        rotation_angle: f32,
        scale: Vector3,
        tint: Color,
    ) {
        // SAFETY: TBD
        self.draw_model_ex(
            unsafe { WeakModel::from_raw(model) },
            position,
            rotation_axis,
            rotation_angle,
            scale,
            tint,
        );
    }

    #[inline]
    fn draw_model_wires(&mut self, model: ffi::Model, position: Vector3, scale: f32, tint: Color) {
        // SAFETY: TBD
        self.draw_model_wires(unsafe { WeakModel::from_raw(model) }, position, scale, tint);
    }

    #[inline]
    fn draw_model_wires_ex(
        &mut self,
        model: ffi::Model,
        position: Vector3,
        rotation_axis: Vector3,
        rotation_angle: f32,
        scale: Vector3,
        tint: Color,
    ) {
        // SAFETY: TBD
        self.draw_model_wires_ex(
            unsafe { WeakModel::from_raw(model) },
            position,
            rotation_axis,
            rotation_angle,
            scale,
            tint,
        );
    }

    #[inline]
    fn draw_bounding_box(&mut self, bbox: BoundingBox, color: Color) {
        self.draw_bounding_box(bbox, color);
    }

    #[inline]
    fn draw_billboard(
        &mut self,
        camera: Camera3D,
        texture: &Texture2D,
        center: Vector3,
        size: f32,
        tint: Color,
    ) {
        self.draw_billboard(camera, texture, center, size, tint);
    }

    #[inline]
    fn draw_billboard_rec(
        &mut self,
        camera: Camera3D,
        texture: &Texture2D,
        source_rec: Rectangle,
        center: Vector3,
        size: Vector2,
        tint: Color,
    ) {
        self.draw_billboard_rec(camera, texture, source_rec, center, size, tint);
    }

    #[inline]
    fn draw_billboard_pro(
        &mut self,
        camera: Camera,
        texture: ffi::Texture2D,
        source: Rectangle,
        position: Vector3,
        up: Vector3,
        size: Vector2,
        origin: Vector2,
        rotation: f32,
        tint: Color,
    ) {
        self.draw_billboard_pro(
            camera, texture, source, position, up, size, origin, rotation, tint,
        );
    }

    #[inline]
    fn draw_model_points(&mut self, model: ffi::Model, position: Vector3, scale: f32, tint: Color) {
        self.draw_model_points(model, position, scale, tint);
    }

    #[inline]
    fn draw_model_points_ex(
        &mut self,
        model: ffi::Model,
        position: Vector3,
        rotation_axis: Vector3,
        angle: f32,
        scale: Vector3,
        tint: Color,
    ) {
        self.draw_model_points_ex(model, position, rotation_axis, angle, scale, tint);
    }
}
