use std::io;
fn main() {
    let mut int = String::new();
    io::stdin().read_line(& mut int).expect("Failed to read input!!");
    let n: u32 = int.trim().parse().unwrap();
    println!("The {}th fibbonacci number is: {}", n, fib(n));
}
fn fib(n: u32) -> u32{
    if n==0{
        return 0;
    } else if n==1{return 1;}
    fib(n-1)+fib(n-2)
}
