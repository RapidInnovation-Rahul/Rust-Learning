// This store is having a sale where if the price is an even number, you get
// 10 Rustbucks off, but if it's an odd number, it's 3 Rustbucks off.

fn main() {
    let original_price = 51;
    let op = 50;
    println!("Your sale price is {}", sale_price(original_price));
    println!("Your sale price is {}", sale_price(op));
}

fn sale_price(price: u32) -> u32 {
    if is_even(price) {
        return price - 10;
    } else {
        return price - 3;
    }
}

fn is_even(num: u32) -> bool {
    if num % 2 == 0 {return true}
    return false;
}