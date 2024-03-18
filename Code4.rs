fn main()
{
    let day=8;
    let day_type=get_data_type(day);
    println!("{}",day_type);
    
}
fn get_data_type(day: u32)-> &'static str{
    match day{
     1 => "Weekend",
     2...6 => "weekday",
     7 => "Weekend",
     _ => "Invalid day",
    }
}
