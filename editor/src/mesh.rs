use raylib::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FaceData<T> {
    Point([T; 1]),
    Line([T; 2]),
    Triangle([T; 3]),
    Quad([T; 4]),
}

impl<T, Idx> std::ops::Index<Idx> for FaceData<T>
where
    [T]: std::ops::Index<Idx>,
{
    type Output = <[T] as std::ops::Index<Idx>>::Output;

    #[inline]
    fn index(&self, index: Idx) -> &Self::Output {
        &self.as_slice()[index]
    }
}

impl<T, Idx> std::ops::IndexMut<Idx> for FaceData<T>
where
    [T]: std::ops::IndexMut<Idx>,
{
    #[inline]
    fn index_mut(&mut self, index: Idx) -> &mut Self::Output {
        &mut self.as_mut_slice()[index]
    }
}

impl<T> FaceData<T> {
    #[inline]
    pub const fn len(&self) -> usize {
        match self {
            FaceData::Point(_) => 1,
            FaceData::Line(_) => 2,
            FaceData::Triangle(_) => 3,
            FaceData::Quad(_) => 4,
        }
    }

    #[inline]
    pub const fn as_slice(&self) -> &[T] {
        match self {
            Self::Point(idx) => idx.as_slice(),
            Self::Line(idx) => idx.as_slice(),
            Self::Triangle(idx) => idx.as_slice(),
            Self::Quad(idx) => idx.as_slice(),
        }
    }

    #[inline]
    pub const fn as_mut_slice(&mut self) -> &mut [T] {
        match self {
            Self::Point(idx) => idx.as_mut_slice(),
            Self::Line(idx) => idx.as_mut_slice(),
            Self::Triangle(idx) => idx.as_mut_slice(),
            Self::Quad(idx) => idx.as_mut_slice(),
        }
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, T> {
        self.as_slice().iter()
    }

    #[inline]
    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, T> {
        self.as_mut_slice().iter_mut()
    }

    #[inline]
    pub fn map<F, U>(self, f: F) -> FaceData<U>
    where
        F: FnMut(T) -> U,
    {
        match self {
            Self::Point(arr) => FaceData::Point(arr.map(f)),
            Self::Line(arr) => FaceData::Line(arr.map(f)),
            Self::Triangle(arr) => FaceData::Triangle(arr.map(f)),
            Self::Quad(arr) => FaceData::Quad(arr.map(f)),
        }
    }

    #[cfg(debug_assertions)]
    #[inline]
    fn check_invariant(&self) -> bool
    where
        T: Eq,
    {
        match self {
            Self::Point(_) => true,
            Self::Line([a, b]) => a != b,
            Self::Triangle([a, b, c]) => (a != b) && (a != c) && (b != c),
            Self::Quad([a, b, c, d]) => {
                (a != b) && (a != c) && (a != d) && (b != c) && (b != d) && (c != d)
            }
        }
    }
}

impl<'a, T: 'a> IntoIterator for &'a FaceData<T> {
    type Item = &'a T;
    type IntoIter = std::slice::Iter<'a, T>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, T: 'a> IntoIterator for &'a mut FaceData<T> {
    type Item = &'a mut T;
    type IntoIter = std::slice::IterMut<'a, T>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

/// Library-level invariant: Elements must never be equal
impl FaceData<u16> {
    /// Returns [`None`] if any index is out of bounds
    #[inline]
    pub fn index<'a, T: 'a>(&self, slice: &'a [T]) -> Option<FaceData<&'a T>> {
        self.iter()
            .all(|&i| usize::from(i) < slice.len())
            // SAFETY: Just checked and all indices are in-bounds.
            .then(|| unsafe { self.index_unchecked(slice) })
    }

    /// Returns [`None`] if any index is out of bounds
    #[inline]
    pub fn index_mut<'a, T: 'a>(&self, slice: &'a mut [T]) -> Option<FaceData<&'a mut T>> {
        self.iter()
            .all(|&i| usize::from(i) < slice.len())
            // SAFETY: Just checked and all indices are in-bounds.
            .then(|| unsafe { self.index_unchecked_mut(slice) })
    }

    /// # Safety
    /// Indices must not be out of bounds.
    #[inline]
    pub unsafe fn index_unchecked<'a, T: 'a>(&self, slice: &'a [T]) -> FaceData<&'a T> {
        debug_assert!(
            self.check_invariant(),
            "face cannot have repeated vertex indices"
        );
        // SAFETY: Caller must uphold safety contract. Library invariant guarantees non-overlapping indices.
        self.map(|idx| unsafe { slice.get_unchecked(usize::from(idx)) })
    }

    /// # Safety
    /// Indices must not be out of bounds.
    #[inline]
    pub unsafe fn index_unchecked_mut<'a, T: 'a>(&self, slice: &'a mut [T]) -> FaceData<&'a mut T> {
        debug_assert!(
            self.check_invariant(),
            "face cannot have repeated vertex indices"
        );
        // SAFETY: Caller must uphold safety contract. Library invariant guarantees non-overlapping indices.
        unsafe {
            match self {
                Self::Point(idx) => {
                    FaceData::Point(slice.get_disjoint_unchecked_mut(idx.map(usize::from)))
                }
                Self::Line(idx) => {
                    FaceData::Line(slice.get_disjoint_unchecked_mut(idx.map(usize::from)))
                }
                Self::Triangle(idx) => {
                    FaceData::Triangle(slice.get_disjoint_unchecked_mut(idx.map(usize::from)))
                }
                Self::Quad(idx) => {
                    FaceData::Quad(slice.get_disjoint_unchecked_mut(idx.map(usize::from)))
                }
            }
        }
    }
}

/// Library-level invariant: Repeated indices must continue to the back of the array
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Face([u16; 4]);

impl Face {
    #[inline]
    pub fn new_point(i1: u16) -> Self {
        Self([i1, i1, i1, i1])
    }

    #[inline]
    pub fn new_line(i1: u16, i2: u16) -> Self {
        debug_assert!(i1 != i2, "line cannot have repeated vertex indices");
        Self([i1, i2, i2, i2])
    }

    #[inline]
    pub fn new_triangle(i1: u16, i2: u16, i3: u16) -> Self {
        debug_assert!(
            (i1 != i2) && (i1 != i3) && (i2 != i3),
            "triangle cannot have repeated vertex indices"
        );
        Self([i1, i2, i3, i3])
    }

    #[inline]
    pub fn new_quad(i1: u16, i2: u16, i3: u16, i4: u16) -> Self {
        debug_assert!(
            (i1 != i2) && (i1 != i3) && (i1 != i4) && (i2 != i3) && (i2 != i4) && (i3 != i4),
            "quad cannot have repeated vertex indices"
        );
        Self([i1, i2, i3, i4])
    }

    #[inline]
    pub fn indices(&self) -> FaceData<u16> {
        let [a, b, c, d] = self.0;
        debug_assert!(
            ((a != b) || (b == c)) && ((b != c) || (c == d)),
            "repeated indices must continue to the end of the array"
        );
        if a == b {
            FaceData::Point([a])
        } else if b == c {
            FaceData::Line([a, b])
        } else if c == d {
            FaceData::Triangle([a, b, c])
        } else {
            FaceData::Quad([a, b, c, d])
        }
    }
}

pub struct AmyMesh {
    vertices: Vec<Vector3>,
    texcoords: Vec<Vector2>,
    faces: Vec<Face>,
}

impl AmyMesh {
    fn check(vertices: &[Vector3], texcoords: &[Vector2], faces: &[Face]) {
        assert_eq!(
            texcoords.len(),
            vertices.len(),
            "vertices and texcoords should share a quanity"
        );

        assert!(
            faces.iter().all(|face| face
                .0
                .iter()
                .copied()
                .map(usize::from)
                .all(|index| index < vertices.len())),
            "every index should be a valid index into the vertex array",
        );
    }

    #[inline]
    pub fn new(vertices: Vec<Vector3>, texcoords: Vec<Vector2>, faces: Vec<Face>) -> Self {
        Self::check(&vertices, &texcoords, &faces);
        Self {
            vertices,
            texcoords,
            faces,
        }
    }

    pub fn gen_plane(width: f32, length: f32) -> Self {
        let mut vertices = Vec::with_capacity(4);
        vertices.extend((0..2).flat_map(|x| {
            let x = (2 * x - 1) as f32 * width;
            (0..2).map(move |z| {
                let z = (2 * z - 1) as f32 * length;
                Vector3::new(x, 0.0, z)
            })
        }));
        // 0: -x, -z
        // 1: -x, +z
        // 2: +x, -z
        // 3: +x, +z
        Self::new(
            vertices,
            vec![Vector2::default(); 4],
            vec![Face::new_quad(0, 1, 3, 2)],
        )
    }

    pub fn gen_cube(width: f32, height: f32, length: f32) -> Self {
        let mut vertices = Vec::with_capacity(8);
        vertices.extend((0..2).flat_map(|x| {
            let x = (2 * x - 1) as f32 * width;
            (0..2).flat_map(move |y| {
                let y = (2 * y - 1) as f32 * height;
                (0..2).map(move |z| {
                    let z = (2 * z - 1) as f32 * length;
                    Vector3::new(x, y, z)
                })
            })
        }));

        // 0: -x, -y, -z
        // 1: -x, -y, +z
        // 2: -x, +y, -z
        // 3: -x, +y, +z
        // 4: +x, -y, -z
        // 5: +x, -y, +z
        // 6: +x, +y, -z
        // 7: +x, +y, +z

        // 2--------6
        // |\       |\    y
        // | 3--------7   |
        // | |      | |   *--- x
        // 0-|------4 |    \
        //  \|       \|     z
        //   1--------5

        let faces = vec![
            Face::new_quad(0, 1, 3, 2),
            Face::new_quad(0, 1, 5, 4),
            Face::new_quad(4, 5, 7, 6),
            Face::new_quad(2, 3, 7, 6),
            Face::new_quad(0, 4, 6, 2),
            Face::new_quad(1, 5, 7, 3),
        ];

        Self::new(vertices, vec![Vector2::default(); 8], faces)
    }

    #[inline]
    pub const fn vertices(&self) -> &[Vector3] {
        self.vertices.as_slice()
    }

    #[inline]
    pub const fn texcoords(&self) -> &[Vector2] {
        self.texcoords.as_slice()
    }

    #[inline]
    pub const fn faces(&self) -> &[Face] {
        self.faces.as_slice()
    }

    #[inline]
    pub fn face_vertices(&self) -> impl ExactSizeIterator<Item = FaceData<&Vector3>> {
        self.faces.iter().map(|face| {
            face.indices()
                .index(self.vertices())
                .expect("all mesh indices should be in bounds")
        })
    }

    #[inline]
    pub fn face_texcoords(&self) -> impl ExactSizeIterator<Item = FaceData<&Vector2>> {
        self.faces.iter().map(|face| {
            face.indices()
                .index(self.texcoords())
                .expect("all mesh indices should be in bounds")
        })
    }
}
