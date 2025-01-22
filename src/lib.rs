pub mod custom {
    use rand::distributions::{Distribution, Standard};
    use rand::Rng;

    #[derive(Debug)]
    pub struct Point {
        x: i32,
        y: i32,
    }

    impl Distribution<Point> for Standard {
        fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Point {
            let (rand_x, rand_y) = rng.gen();
            Point {
                x: rand_x,
                y: rand_y,
            }
        }
    }
}
