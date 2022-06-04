fn main() {
    // unsigned integer (u)
    // u8, u16, u32, u64, u128
    let x: u8 = 123;

    // signed integer (i)
    // i8, i16, i32, i64, i128
    let y: i8 = -123;

    // float (f)
    // f32, f64
    let z: f64 = 0.1;

    println!("x = {}, y = {}, z = {}", x, y, z);

    // boolean (b)
    let is_ok: bool = true;

    // char (c)
    let c: char = 'a';

    // string (s)
    let s: String = "hello".to_string();

    println!("is_ok = {}, c = {}, s = {}", is_ok, c, s);

    // array (a)
    let a: [i32; 5] = [1, 2, 3, 4, 5];

    // tuple (t)
    let t: (i32, f64, u8) = (1, 0.1, 123);

    println!(
        "t = {:?},
                t.0 = {},
                t.1 = {},
                t.2 = {}",
        t, t.0, t.1, t.2
    );

    println!("a = {:?}", a);

    // enum (e)
    enum Color {
        Red,
        Green,
        Blue,
    }

    let red: Color = Color::Red;
    let green: Color = Color::Green;
    let blue: Color = Color::Blue;

    // struct (st)
    struct Point {
        x: i32,
        y: i32,
    }

    let p: Point = Point { x: 1, y: 2 };

    // function (f)
    fn add(x: i32, y: i32) -> i32 {
        x + y
    }

    // if
    if is_ok {
        println!("ok");
    }

    // while
    let mut i = 0;
    while i < 10 {
        println!("{}", i);
        i += 1;
    }

    // for
    for i in 0..10 {
        if i % 3 == 0 {
            match red {
                Color::Red => println!("red"),
                Color::Green => println!("green"),
                Color::Blue => println!("blue"),
            }
        } else if i % 3 == 1 {
            match green {
                Color::Red => println!("red"),
                Color::Green => println!("green"),
                Color::Blue => println!("blue"),
            }
        } else {
            match blue {
                Color::Red => println!("red"),
                Color::Green => println!("green"),
                Color::Blue => println!("blue"),
            }
        }
    }

    // function
    println!("{}", add(p.x, p.y));

    struct Emoji {
        __id: i32,

        name: String,
    }

    let emoji_dog = Emoji {
        __id: 1,
        name: "🐶".to_string(),
    };

    let emoji_cat = Emoji {
        __id: 2,
        name: "🐱".to_string(),
    };
    let emoji_panda = Emoji {
        __id: 3,
        name: "🐼".to_string(),
    };

    let emojis = vec![emoji_dog, emoji_cat, emoji_panda];

    for emoji in emojis {
        println!("Emoji => {}", emoji.name);
    }

    struct HappyFace {
        face_code: String,
    }

    let smiley_face = HappyFace {
        face_code: "😀".to_string(),
    };

    let frowny_face = HappyFace {
        face_code: "😞".to_string(),
    };

    let happy_faces = vec![smiley_face, frowny_face];

    for face in happy_faces {
        println!("Face => {}", face.face_code);
    }

    let once_array: [u64; 10] = [1; 10];

    let mut mutable_array: [u64; 10] = [1; 10];

    mutable_array[0] = 2;

    println!("once_array = {:?}, legth= {}", once_array, once_array.len());
    println!("mutable_array = {:?}", mutable_array);

    // destructuring
    let (x, y, z) = t;

    println!("x = {}, y = {}, z = {}", x, y, z);
    // even if we don't need all the values, we can still destructure
    let (x, _, z) = t;

    println!("x = {}, z = {}", x, z);

    // struct destructuring
    let Point { x, y } = p;

    println!("x = {}, y = {}", x, y);

    // is Even ?
    let even: bool = is_even(x);

    println!("x is even ? {}", even);

    // slices
    let arr = [1, 2, 3, 4, 5];
    let slice = &arr[1..3];
    borrow_slicer(&arr, slice);

    // string slice
    let s: &str = "hello world";
    let s_slice = &s[0..5];
    borrow_str_slicer(s, s_slice);

    let mut string: String = String::from("Hello World");
    let string_slice = &string[0..5];
    println!("string_slice = {}", string_slice);

    string.push('!');
    string.push_str("!");
    let mut string_slice = string.as_str();
    println!("string_slice = {}", string_slice);

    string = string.replace("Hello", "Hi");
    string_slice = string.as_str();
    println!("string_slice = {}", string_slice);

    // match
    let i = 3;
    match i {
        0 => println!("Zer0"),
        1 | 2 => println!("one | two"),
        3..=4 => println!("three or four"),
        _ => println!("something else"),
    }

    let __name: String = String::from("Parrot");
    let __color: String = String::from("Green");
    let __age: i32 = 1;

    let bird: Bird = Bird { __name, __color };
    bird.fly();
    bird.rest();
    bird.fly_again();

    println!("{}", bird.can_lay_eggs());
}

// Public Even Function Checking if a number is even or odd (returns a bool) (f)
pub fn is_even(x: i32) -> bool {
    x % 2 == 0
}

// Public Borrowing Slicer (s) (f)
pub fn borrow_slicer(arr: &[i32], slice: &[i32]) {
    println!("arr = {:?}", arr);
    println!("slice = {:?}", slice);
    println!("length: {}", slice.len());
    println!("first element: {}", slice[0]);
}
// Public Borrowing Slicer (s) (f)
pub fn borrow_str_slicer(s: &str, slice: &str) {
    println!("s = {}", s);
    println!("slice = {}", slice);
    println!("length: {}", slice.len());
}

// Traits

pub trait AnimalTrait {
    fn is_animal(&self) -> bool;
    fn can_fly(&self) -> bool;
    fn can_walk(&self) -> bool;
    fn can_swim(&self) -> bool;
    fn can_sing(&self) -> bool;
    fn has_name(&self) -> bool;
}
trait BirdTrait {
    fn fly(&self);
    fn fly_again(&self);
    fn rest(&self);
    fn can_lay_eggs(&self) -> bool;
}

// Structs with Traits

pub struct Animal {
    __name: String,
}
#[derive()]
pub struct Bird {
    __name: String,
    __color: String,
}

// Implementations
impl Animal {
    pub fn new(name: String) -> Animal {
        Animal { __name: name }
    }
}
impl BirdTrait for Bird {
    fn fly(&self) {
        println!("{} can fly", self.__name);
    }
    fn fly_again(&self) {
        println!("{} can fly again", self.__name);
    }
    fn rest(&self) {
        println!("{} rest", self.__name);
    }
    fn can_lay_eggs(&self) -> bool {
        true
    }
}

// Language: rust
// Path: src/main.rs
