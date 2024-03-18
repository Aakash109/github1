fn extractor(input: &str) -> Option<char> {
    let mut chars = input.chars();
    if let Some(first_char) = chars.next() {
        Some(first_char)
    } 
    else {
        None
    }
}

fn main() {
    let string = "Aakash";
    let first_char = extractor(string);
    println!("{:?}", first_char); 
    let empty_string = "";
    let first_char_empty = extractor(empty_string);
    println!("{:?}", first_char_empty); 

}
