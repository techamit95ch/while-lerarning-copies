use std::collections::HashMap;

// Options
// None to indicate failure or lack of value, and
//  Some(Value) , a tuple struct that wraps a value with Type T
//  Option<T> , a type alias for Option<T>

fn devide(devident: i32, devisor: i32) -> Option<i32> {
    if devident % devisor != 0 {
        None
    } else {
        Some(devident / devisor)
    }
}

// Error Enum
//  ErrorKind, a type alias for enum
//  Error, a tuple struct that wraps a value with Type T
//  Result<T, E>, a type alias for Result<T, E>

#[derive(Debug)]
enum Error {
    DivisionByZero,
    NegativeLogarithm,
    NegativeSquareRoot,
    CustomError,
}

// Result
//  Ok(T) , a type alias for Result<T, E>
//  Err(E) , a type alias for Result<T, E>

fn sqrt(x: f64) -> Result<f64, Error> {
    if x < 0.0 {
        Err(Error::NegativeSquareRoot)
    } else {
        Ok(x.sqrt())
    }
}

fn main() {
    // Vector
    let mut vec: Vec<i64> = vec![1, 2, 3, 4, 5];
    vec.len();
    vec[0];
    vec.push(6);
    vec.remove(0);
    println!("{:?}", vec);

    // HashMap
    let mut map = HashMap::new();
    map.insert(0, "one".to_string());
    map.insert(1, "two".to_string());
    map.insert(2, "three".to_string());
    map.insert(3, "four".to_string());
    println!("{:?}", map);
    match map.get(&0) {
        Some(value) => println!("{}", value),
        None => println!("None"),
    }
    map.remove(&0);
    println!("{:?}", map);

    // Option
    let devide1: Option<i32> = devide(10, 2);
    // let devide2: Option<i32> = devide(2, 3);

    // unwraping a Some
    println!("{:?} unwraps => {}", devide1, devide1.unwrap());
    // unwraping a None will throw a `panic!`
    // println!("{:?} unwraps => {}", devide2, devide2.unwrap());

    // square root
    let sqrt1: Result<f64, Error> = sqrt(9.0);
    let sqrt2: Result<f64, Error> = sqrt(-9.0);
    if sqrt1.is_ok() {
        println!("{:?}", sqrt1.unwrap());
    }
    if sqrt2.is_err() {
        println!("{:?}", sqrt2.unwrap_err());
    }
    // unwrap_or
    let sqrt3: f64 = sqrt(9.0).unwrap_or(0.0);
    println!("{:?}", sqrt3);
    // unwrap_or_else
    let sqrt4: Result<f64, Error> = sqrt(-9.0);
    let errExpect = sqrt4.expect("We crashed");
    println!("{:?}", errExpect);
}
