fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    let val = vec[10]; // This will panic!
    println!("The value is {}", val);
}