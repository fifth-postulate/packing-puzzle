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
