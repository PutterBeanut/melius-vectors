# melius-vectors
 Simple 2-4 dimensional vectors that can be easily created and manipulated.

# Installation
Under your Cargo.toml's `[Dependencies]`, add the following dependency:
`melius-vectors = { git = "https://github.com/PutterBeanut/melius-vectors/" }`

# Usage
```rs
use melius_vectors::vector2::Vector2;

let mut my_vector: Vector2<f32> = Vector2::empty();
my_vector += Vector2::new(20, 10);
my_vector /= Vector2::new(4, 2);

println!("{}", my_vector); // Prints "{5.0, 5.0}"
```
