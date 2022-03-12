#![allow(non_snake_case)]
use std::io;

fn main() {
    println!("*****Chooe from the given options*****");
    println!("1. Celcius to Farhenheit");
    println!("2. Farhenheit to Celcius");
    println!("*****pls enter 1 or 2*****");
    let mut int = String::new();
    io::stdin().read_line(& mut int).expect("Failed to read");
    let n:i8 =  int.trim().parse().unwrap();
    
    println!("Enter the temperature you want to convert: ");
    let mut t = String::new();
    io::stdin().read_line(& mut t).expect("Failed to read the temperature..");
    let temp:f32 = t.trim().parse().expect("invalid temp");

    match n{
        1 => println!("Your C to F conversion is: {}",Farhenheit(temp)),
        2 => println!("Your F to C conversion is: {}", Celcius(temp)),
        _ => println!("t = {:?}", t),
    }
}
fn Farhenheit(c: f32) -> f32{
    (c*(9.0/5.0))+32.0
}
fn Celcius(f: f32) -> f32{
    (f-32.0)*(5.0/9.0)
}

