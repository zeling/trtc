pub mod tuples {
    #[derive(Default)]
    pub struct Tuple {
        pub x: f32,
        pub y: f32,
        pub z: f32,
        pub w: f32,
    }

    impl Tuple {
        pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
            Self { x, y, z, w }
        }

        pub fn is_point(&self) -> bool {
            return self.w == 1.0;
        }

        pub fn is_vector(&self) -> bool {
            return self.w == 0.0;
        }
    }
}