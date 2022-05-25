use std::fmt::{self, Error}; // Import the `fmt` module.

// Define a structure named `List` containing a `Vec`.
struct List(Vec<i32>);

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let vec = &self.0;
        let size = vec.len();
        write!(f, "[").and_then(|_| {
            vec.iter()
                .enumerate()
                .fold(Ok(()), |acc, (idx, value)| match idx + 1 == size {
                    true => {
                        acc.and_then(|_| -> Result<(), Error> { write!(f, "{}:{}", idx, value) })
                    }
                    false => {
                        acc.and_then(|_| -> Result<(), Error> { write!(f, "{}:{},", idx, value) })
                    }
                })
                .and_then(|_| write!(f, "]"))
        })
    }
}

#[test]
fn display_list() {
    let v = List(vec![1, 2, 3]);
    println!("{}", v);
}
