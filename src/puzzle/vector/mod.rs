//! Mathematical vector are bases of translations and positions.
//!
//! Mathematical vectors are things that will implement (a selection) of the
//! triats in this module. Not necessarily a `Vec`.

/// Determine the difference of two mathematical vectors.
pub trait VectorDifference<T> {
    /// other - self
    fn difference(&self, other: &T) -> T;
}

impl VectorDifference<(i8, i8)> for (i8, i8) {
    fn difference(&self, other: &(i8, i8)) -> (i8, i8) {
        (other.0 - self.0, other.1 - self.1)
    }
}

impl VectorDifference<(i8, i8, i8)> for (i8, i8, i8) {
    fn difference(&self, other: &(i8, i8, i8)) -> (i8, i8, i8) {
        (other.0 - self.0, other.1 - self.1, other.2 - self.2)
    }
}

/// Add a mathematical vector to an other
pub trait VectorAdd<T> {
    /// self += other
    fn add(&mut self, other: &T);
}

impl VectorAdd<(i8, i8)> for (i8, i8) {
    fn add(&mut self, other: &(i8, i8)) {
        self.0 += other.0;
        self.1 += other.1;
    }
}

impl VectorAdd<(i8, i8, i8)> for (i8, i8, i8) {
    fn add(&mut self, other: &(i8, i8, i8)) {
        self.0 += other.0;
        self.1 += other.1;
        self.2 += other.2;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_determine_vector_difference_of_tuple() {
        let a: (i8, i8) = (1, 2);
        let b: (i8, i8) = (4, 4);

        let difference = a.difference(&b);

        assert_eq!(difference, (3, 2))
    }

    #[test]
    fn should_determine_vector_difference_of_triple() {
        let a: (i8, i8, i8) = (1, 2, 3);
        let b: (i8, i8, i8) = (4, 4, 4);

        let difference = a.difference(&b);

        assert_eq!(difference, (3, 2, 1))
    }

    #[test]
    fn should_determine_vector_addition_of_tuple() {
        let mut a: (i8, i8) = (1, 2);
        let b: (i8, i8) = (4, 4);

        a.add(&b);

        assert_eq!(a, (5, 6))
    }

    #[test]
    fn should_determine_vector_addition_of_triple() {
        let mut a: (i8, i8, i8) = (1, 2, 3);
        let b: (i8, i8, i8) = (4, 4, 4);

        a.add(&b);

        assert_eq!(a, (5, 6, 7))
    }
}
