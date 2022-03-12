fn main(){
    let mut n = bigger(1,2);
    println!("the bigger nubmer is:{}", n);
}
pub fn bigger(a: i32, b: i32) -> i32 {
    
    if a>b {return a;}
    return b;
}