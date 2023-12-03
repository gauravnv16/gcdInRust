// below function is used to implement gcd in rust by gaurav nv
fn gcd(a:i32,b:i32) -> i32{
    if b == 0 {
        return a;
    }
    return gcd(b, a%b);
}
fn main() {
    println!("GCD of {} and {} is {}",98,56,gcd(98,56));
}
