use std::fmt::{self, Display};

struct Complex(f64, f64);

impl Display for Complex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}+{}i", self.0, self.1)
    }
}

impl fmt::Debug for Complex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Complex {{ real:{}, imag:{} }}", self.0, self.1)
    }
}

#[test]
fn format_display() {
  let complex = Complex(3.3, 7.2);
  println!("{}\n{:?}", complex, complex);
}
