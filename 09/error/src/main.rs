use std::io;
use std::io::Read;
use std::fs::File;
fn read_username_from_file() -> Result<String,io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}
pub struct Guess{
    value: u32,
}
impl Guess {
    pub fn new(value: u32) -> Guess{
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.",value);
        }
        Guess{value}
    }
    pub fn value (&self) -> u32{
        self.value
    }
}
fn main() {
    println!("{:?}",read_username_from_file());
}

