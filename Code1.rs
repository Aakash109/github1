fn main() {
    let name="John";
    greeting(name);
}
fn greeting(name: &str){
    println!("Hello,{}!",name);
}
