fn main() {
    let var: i32 = 100;
    let var1: Box<i32> = Box::new(var);
    println!("The value stored in the Box: {}", *var1);
}
