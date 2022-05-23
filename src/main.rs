use std::error::Error;
use std::io::Read;
use std::fs::File;
use std::io;

fn main() -> Result<(),Box<dyn Error>>{
    let num_a: &str = "12";

    let num_b: &str = "150";
    // Aquire ownership to allow concat
    let concat: String = num_a.to_owned() + num_b;

    println!("{}",concat);

    let file_ref: &str = "./src/text.txt";

    let file_contents: Result<String,io::Error> = read_all(file_ref);

    println!("{:#?}",file_contents);

    let number_list: Vec<i32> = vec![120,1230,11,9,187];

    let largest_number: i32 = get_largest(&number_list);

    println!("Largest number: {}",largest_number);

    Ok(())
}

fn get_largest(item_list: &[i32]) -> i32 {
    let mut largest_num = item_list[0];
    for &item in item_list {
        if item > largest_num {
            largest_num = item;
        }
    }
    largest_num
}
// Error handling using Result -> Ok/Err
fn read_all(ref_text: &str) -> Result<String,io::Error> {
    let mut string_info = String::new();
    File::open(ref_text)?.read_to_string(&mut string_info)?;
    Ok(string_info)
}
// Error handling using Option -> Some/None
fn last_char(ref_text: String) -> Option<char> {
    ref_text.lines().next()?.chars().last()
}
