use bevy::prelude::Vec3;

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Direction {
    x: f32,
    y: f32,
    z: f32,
}

impl Direction {
    /// A direction pointing along the positive X axis (Right).
    pub const X: Self = Self::new_unchecked(1.0, 0.0, 0.0);

    /// A direction pointing along the positive Y axis (Up).
    pub const Y: Self = Self::new_unchecked(0.0, 1.0, 0.0);

    /// A direction pointing along the positive Z axis (Back).
    pub const Z: Self = Self::new_unchecked(0.0, 0.0, 1.0);

    /// A direction pointing along the negative X axis (Left).
    pub const NEG_X: Self = Self::new_unchecked(-1.0, 0.0, 0.0);

    /// A direction pointing along the negative Y axis (Down).
    pub const NEG_Y: Self = Self::new_unchecked(0.0, -1.0, 0.0);

    /// A direction pointing along the negative Z axis (Forward).
    pub const NEG_Z: Self = Self::new_unchecked(0.0, 0.0, -1.0);

    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self::validate(Vec3::new(x, y, z));
        Self { x, y, z }
    }

    /// Creates a new Direction without validating its magnitude.
    const fn new_unchecked(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn to_vector(&self) -> Vec3 {
        Vec3::new(self.x, self.y, self.z)
    }

    pub fn from_vector(vector: Vec3) -> Self {
        Self::validate(vector);
        Self {
            x: vector.x,
            y: vector.y,
            z: vector.z,
        }
    }

    fn validate(value: Vec3) {
        if value.length() != 1.0 {
            panic!(
                "A Direction needs to have a magnitude of 1, but {} was given.",
                value.length()
            );
        }
    }
}

impl From<Vec3> for Direction {
    fn from(value: Vec3) -> Self {
        Self::from_vector(value)
    }
}

impl From<Direction> for Vec3 {
    fn from(value: Direction) -> Self {
        value.to_vector()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_create_direction_from_vector() {
        let direction = Direction::from_vector(Vec3::new(1.0, 0.0, 0.0));

        assert_eq!(direction.x, 1.0);
        assert_eq!(direction.y, 0.0);
    }

    #[test]
    #[should_panic]
    fn panics_on_invalid_input() {
        Direction::new(1.0, 0.2, 0.0);
    }

    #[test]
    fn can_get_vector_from_direction() {
        let direction = Direction::Y;

        let result = direction.to_vector();

        assert_eq!(result, Vec3::Y);
    }
}
