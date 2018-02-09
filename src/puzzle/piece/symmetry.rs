//! Symmetries of pieces.

/// Contract how various entities transform under the symmetries of the cube.
pub trait Transformable {
    /// Apply a symmetry and transform the entity.
    fn transform(&mut self, symmetry: &CubeSymmetry);
}

/// Symmetries of the Cube.
///
/// The group of symmetries of the cube is isomorphic to S<sub>4</sub>. Here we
/// use that fact as a naming convention for our elements.
#[derive(Clone)]
pub enum CubeSymmetry {
    #[allow(missing_docs)]
    E0123,
    #[allow(missing_docs)]
    E0132,
    #[allow(missing_docs)]
    E0213,
    #[allow(missing_docs)]
    E0231,
    #[allow(missing_docs)]
    E0312,
    #[allow(missing_docs)]
    E0321,

    #[allow(missing_docs)]
    E1023,
    #[allow(missing_docs)]
    E1032,
    #[allow(missing_docs)]
    E1203,
    #[allow(missing_docs)]
    E1230,
    #[allow(missing_docs)]
    E1302,
    #[allow(missing_docs)]
    E1320,

    #[allow(missing_docs)]
    E2013,
    #[allow(missing_docs)]
    E2031,
    #[allow(missing_docs)]
    E2103,
    #[allow(missing_docs)]
    E2130,
    #[allow(missing_docs)]
    E2301,
    #[allow(missing_docs)]
    E2310,

    #[allow(missing_docs)]
    E3012,
    #[allow(missing_docs)]
    E3021,
    #[allow(missing_docs)]
    E3102,
    #[allow(missing_docs)]
    E3120,
    #[allow(missing_docs)]
    E3201,
    #[allow(missing_docs)]
    E3210,
}

/// Iterator over the symmetries of the cube.
pub struct CubeSymmetryIterator {
    item: Option<CubeSymmetry>,
}

impl CubeSymmetryIterator {
    /// Create a `CubeSymmetryIterator`.
    pub fn new() -> CubeSymmetryIterator {
        CubeSymmetryIterator { item: Some(CubeSymmetry::E0123) }
    }
}

impl Iterator for CubeSymmetryIterator {
    type Item = CubeSymmetry;

    fn next(&mut self) -> Option<Self::Item> {
        match self.item.clone() {
            s @ Some(CubeSymmetry::E0123) => { self.item = Some(CubeSymmetry::E0132); s },
            s @ Some(CubeSymmetry::E0132) => { self.item = Some(CubeSymmetry::E0213); s },
            s @ Some(CubeSymmetry::E0213) => { self.item = Some(CubeSymmetry::E0231); s },
            s @ Some(CubeSymmetry::E0231) => { self.item = Some(CubeSymmetry::E0312); s },
            s @ Some(CubeSymmetry::E0312) => { self.item = Some(CubeSymmetry::E0321); s },
            s @ Some(CubeSymmetry::E0321) => { self.item = Some(CubeSymmetry::E1023); s },

            s @ Some(CubeSymmetry::E1023) => { self.item = Some(CubeSymmetry::E1032); s },
            s @ Some(CubeSymmetry::E1032) => { self.item = Some(CubeSymmetry::E1203); s },
            s @ Some(CubeSymmetry::E1203) => { self.item = Some(CubeSymmetry::E1230); s },
            s @ Some(CubeSymmetry::E1230) => { self.item = Some(CubeSymmetry::E1302); s },
            s @ Some(CubeSymmetry::E1302) => { self.item = Some(CubeSymmetry::E1320); s },
            s @ Some(CubeSymmetry::E1320) => { self.item = Some(CubeSymmetry::E2013); s },

            s @ Some(CubeSymmetry::E2013) => { self.item = Some(CubeSymmetry::E2031); s },
            s @ Some(CubeSymmetry::E2031) => { self.item = Some(CubeSymmetry::E2103); s },
            s @ Some(CubeSymmetry::E2103) => { self.item = Some(CubeSymmetry::E2130); s },
            s @ Some(CubeSymmetry::E2130) => { self.item = Some(CubeSymmetry::E2301); s },
            s @ Some(CubeSymmetry::E2301) => { self.item = Some(CubeSymmetry::E2310); s },
            s @ Some(CubeSymmetry::E2310) => { self.item = Some(CubeSymmetry::E3012); s },

            s @ Some(CubeSymmetry::E3012) => { self.item = Some(CubeSymmetry::E3021); s },
            s @ Some(CubeSymmetry::E3021) => { self.item = Some(CubeSymmetry::E3102); s },
            s @ Some(CubeSymmetry::E3102) => { self.item = Some(CubeSymmetry::E3120); s },
            s @ Some(CubeSymmetry::E3120) => { self.item = Some(CubeSymmetry::E3201); s },
            s @ Some(CubeSymmetry::E3201) => { self.item = Some(CubeSymmetry::E3210); s },
            s @ Some(CubeSymmetry::E3210) | s @ None => { self.item = None; s },
        }
    }
}

impl Transformable for (i8, i8, i8) {
    fn transform(&mut self, symmetry: &CubeSymmetry) {
        let x = self.0;
        let y = self.1;
        let z = self.2;
        let sx: i8;
        let sy: i8;
        let sz: i8;
        match *symmetry {
            CubeSymmetry::E0123 => { sx =  x; sy =  y; sz =  z; },
            CubeSymmetry::E0132 => { sx = -z; sy = -y; sz = -x; },
            CubeSymmetry::E0213 => { sx = -x; sy = -z; sz = -y; },
            CubeSymmetry::E0231 => { sx =  y; sy =  z; sz =  x; },
            CubeSymmetry::E0312 => { sx =  z; sy =  x; sz =  y; },
            CubeSymmetry::E0321 => { sx = -y; sy = -x; sz = -z; },
            CubeSymmetry::E1023 => { sx =  z; sy = -y; sz =  x; },
            CubeSymmetry::E1032 => { sx = -x; sy =  y; sz = -z; },
            CubeSymmetry::E1203 => { sx = -z; sy = -x; sz =  y; },
            CubeSymmetry::E1230 => { sx = -y; sy =  x; sz =  z; },
            CubeSymmetry::E1302 => { sx =  x; sy =  z; sz = -y; },
            CubeSymmetry::E1320 => { sx =  y; sy = -z; sz = -x; },
            CubeSymmetry::E2013 => { sx = -y; sy =  z; sz = -x; },
            CubeSymmetry::E2031 => { sx =  x; sy = -z; sz =  y; },
            CubeSymmetry::E2103 => { sx =  y; sy =  x; sz = -z; },
            CubeSymmetry::E2130 => { sx =  z; sy = -x; sz = -y; },
            CubeSymmetry::E2301 => { sx = -x; sy = -y; sz =  z; },
            CubeSymmetry::E2310 => { sx = -z; sy =  y; sz =  x; },
            CubeSymmetry::E3012 => { sx =  y; sy = -x; sz =  z; },
            CubeSymmetry::E3021 => { sx = -z; sy =  x; sz = -y; },
            CubeSymmetry::E3102 => { sx = -y; sy = -z; sz =  x; },
            CubeSymmetry::E3120 => { sx = -x; sy =  z; sz =  y; },
            CubeSymmetry::E3201 => { sx =  z; sy =  y; sz = -x; },
            CubeSymmetry::E3210 => { sx =  x; sy = -y; sz = -z; },
         }
        self.0 = sx;
        self.1 = sy;
        self.2 = sz;
    }
}

impl Transformable for (i8, i8) {
    fn transform(&mut self, symmetry: &CubeSymmetry) {
        let mut v: (i8, i8, i8) = (self.0, self.1, 0);
        v.transform(symmetry);
        self.0 = v.0;
        self.1 = v.1;
    }
}
