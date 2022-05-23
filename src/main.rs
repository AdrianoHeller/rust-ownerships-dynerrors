use std::error::Error;
use std::io::Read;
use std::fs::File;
use std::io;

fn main() -> Result<(),Box<dyn Error>>{
    let num_a: &str = "12";

    let num_b: &str = "150";

    let concat: String = num_a.to_owned() + num_b;

    println!("{}",concat);

    let file_ref: &str = "./src/text.txt";

    let file_contents: Result<String,io::Error> = read_all(file_ref);

    println!("{:#?}",file_contents);

    Ok(())
}

fn read_all(ref_text: &str) -> Result<String,io::Error> {
    let mut string_info = String::new();
    File::open(ref_text)?.read_to_string(&mut string_info)?;
    Ok(string_info)
}

fn last_char(ref_text: String) -> Option<char> {
    ref_text.lines().next()?.chars().last()
}
