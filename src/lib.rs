pub mod vector2;
pub mod vector3;
pub mod vector4;

mod test {
    use crate::vector2::Vector2;

    #[test]
    fn test_fn() {
        use crate::vector2::Vector2;

        let mut vec: Vector2<f32> = Vector2::empty();
        vec += Vector2::new(52.0, 2.6);
        vec *= Vector2::new(0.5, 2.0);
        println!("{}", vec);
    }
}
