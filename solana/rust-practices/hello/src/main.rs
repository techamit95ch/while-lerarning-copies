// create Enum For Color
pub enum Color {
    Red,
    Green,
    Blue,
}

impl std::fmt::Debug for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Red => write!(f, "Red"),
            Self::Green => write!(f, "Green"),
            Self::Blue => write!(f, "Blue"),
        }
    }
}

pub enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
// structure for point
#[derive(Debug)]
pub struct Point {
    color: Color,
    __x: i32,
    __y: i32,
}

impl std::ops::DerefMut for Point {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.color
    }
}

impl std::ops::Deref for Point {
    type Target = Color;

    fn deref(&self) -> &Self::Target {
        &self.color
    }
}

impl Point {
    /// Creates a new [`Point`].
    pub fn new(__x: i32, __y: i32, color: Color) -> Self {
        Self { __x, __y, color }
    }
}

fn main() {
    // Implement Color
    //
    // Implement the `Color` trait for the `Point` struct below. `Color` trait
    // requires `fmt::Display` be implemented for `Point`.
    //
    // The output should look like:
    // ```text
    // Point(x: 2, y: 3, color: Color(Red, Green))
    // ```
    //
    // You can refer to the `colors` module for the `Color` enum.
    //
    // Your `main` function should call `point` and pass in the correct parameters.
    //
    // Execute `rustlings hint colors` to get more information about the Color enum.
    //
    // You can search for the `colors` module to find the documentation for the Color enum.
    //
    // If you get stuck, you can find a solution in the Rust Book:
    // https://doc.rust-lang.org/book/second-edition/ch03-02-defining-structs.html
    //
    // You can also ask for help on this exercise on the discussion forums.

    let point: Point = Point {
        __x: 2,
        __y: 3,
        color: Color::Red,
    };

    match point.color {
        Color::Red => println!("Red"),
        Color::Green => println!("Green"),
        Color::Blue => println!("Blue"),
    }

    println!(r#"Point({:?})"#, point);
    let _point_red: Point = Point::new(2, 3, Color::Red);
    let point_green: Point = Point::new(2, 3, Color::Green);

    match point_green {
        Point {
            color: Color::Red, ..
        } => println!("Red Point"),
        Point {
            color: Color::Green,
            ..
        } => println!("Green Point"),
        Point {
            color: Color::Blue, ..
        } => println!("Blue Point"),
    }

    let mut _a = MyEnum::A;
    let mut _b = MyEnum::B(100);
    let mut _c = MyEnum::C(C { __x: 1, __y: 2 });

    println!("{:?}", (_a, _b, _c));
}

// Enums

#[derive(Debug)]
struct C {
    __x: i32,
    __y: i32,
}

enum MyEnum {
    A,
    B(i32),
    C(C),
}

impl std::fmt::Debug for MyEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::A => write!(f, "A"),
            Self::B(arg0) => f.debug_tuple("B").field(arg0).finish(),
            Self::C(C { __x, __y }) => f.debug_struct("C").field("x", __x).field("y", __y).finish(),
        }
    }
}
pub struct ColorStruct {
    r: u8,
    g: u8,
    b: u8,
}

impl std::ops::DerefMut for ColorStruct {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.g
    }
}

impl std::ops::Deref for ColorStruct {
    type Target = u8;

    fn deref(&self) -> &Self::Target {
        &self.r
    }
}

impl ColorStruct {
    pub fn new(r: u8, g: u8, b: u8) -> ColorStruct {
        ColorStruct { r, g, b }
    }

    pub fn b_mut(&mut self) -> &mut u8 {
        &mut self.b
    }
}

// Language: rust
// Path: src/main.rs
