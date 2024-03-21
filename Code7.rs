struct Resource {
    id: u32,
}

impl Drop for Resource {
    fn drop(&mut self) {
        // Perform cleanup logic here
        println!("Cleaning up resource with id {}", self.id);
    }
}
fn main() {
    let res = Resource { id: 100 };
}
