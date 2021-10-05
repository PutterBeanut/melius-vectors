# melius-vectors
 Simple 2-4 dimensional vectors that can be easily created and manipulated.

# Usage
```rs
use melius_vectors::vector2::Vector2;

let mut my_vector: Vector2<f32> = Vector2::empty();
my_vector += Vector2::new(0.0, 24.3);

println!("{}", my_vector);
```