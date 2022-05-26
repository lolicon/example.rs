use std::{fmt::Display, mem};

#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

impl Display for Matrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "( {} {} ) \n( {} {} )", self.0, self.1, self.2, self.3)
    }
}

fn transpose(from: &Matrix) -> Matrix {
    Matrix(from.0, from.2, from.1, from.3)
}

#[test]
fn test_for_primitive_tuples() {
    let from = Matrix(1.1, 1.2, 2.1, 2.2);
    let to = transpose(&from);
    println!("{} \n{}", from, to);

    let foo = &[1, 2, 3, 4];
    let chr = 'a';
    println!("usize: {} chr: {}", mem::size_of_val(&foo), mem::size_of_val(&chr));
}
