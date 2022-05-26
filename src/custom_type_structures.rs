// A struct with two fields
#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

// Structs can be reused as fields of another struct
#[derive(Debug)]
struct Rectangle {
    // A rectangle can be specified by where the top left and bottom right
    // corners are in space.
    top_left: Point,
    bottom_right: Point,
}

fn rect_area(rect: Rectangle) -> f32 {
    let Rectangle {
        top_left: Point { x: top, y: left },
        bottom_right: Point {
            x: bottom,
            y: right,
        },
    } = rect;

    (bottom - top) * (right - left)
}

fn square(pointer: Point, length: f32) -> Rectangle {
    let Point { x, y } = pointer;
    Rectangle {
        top_left: pointer,
        bottom_right: Point {
            x: x + length,
            y: y + length,
        },
    }
}

#[test]
fn test_for_structures() {
    println!(
        "area: {}",
        rect_area(Rectangle {
            top_left: Point { x: 0.0, y: 0.0 },
            bottom_right: Point { x: 10.0, y: 20.0 }
        })
    );

    let x = 0.0;
    let y = 0.0;

    println!("{:?} ", square(Point { x, y }, 100.0))
}
