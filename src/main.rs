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

    // Implemented via Prototype
    let one: User<String,i32> = User::new(
        String::from("Robles"),
        String::from("robles@gmail.com"),
        String::from("32cx4374b2849732b09432984v32042"),
        124523682
    );

    // Implemented via Generic Method
    let new_user = create_new_user(String::from("Robles"),
                    String::from("robles@gmail.com"),
                    String::from("32cx4374b2849732b09432984v32042"),
                    124523682);

    println!("{:#?}",one);

    println!("{:#?}",new_user);

    Ok(())
}

// fn get_largest<T>(item_list: &[T]) -> T {
//     let mut largest = item_list[0];
//     for &item in item_list {
//         if item > largest {
//             largest = item;
//         }
//     }
//     largest
// }

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

#[derive(Debug)]
struct User<T,V> {
    name: T,
    email: T,
    id: T,
    created_at: V,
}

impl<T,V> User<T,V> {
    fn new(name: T, email: T, id: T, created_at: V) -> User<T,V> {
        User {
            email,
            name,
            id,
            created_at
        }
    }
    fn name(&self) -> &T {
        &self.name
    }
    fn email(&self) -> &T {
        &self.email
    }
    fn id(&self) -> &T {
        &self.id
    }
}

fn create_new_user(name: String, email: String, id: String, created_at: i32) -> User<String,i32> {
    User {
        email,
        name,
        id,
        created_at
    }
}

