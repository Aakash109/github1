fn main() {
    let var = 100;
    let ref_var = &var;
    println!("Dereferencing reference: {}", *ref_var);
    let ptr_var: *const i32 = &var as *const i32;
    unsafe {
        println!("Dereferencing pointer: {}", *ptr_var);
    }
}
