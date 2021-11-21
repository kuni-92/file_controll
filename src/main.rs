mod file_controll;
use crate::file_controll::reader;

fn main() {
    let f = reader::read_file();
    let f = match f {
        Ok(str_data) => str_data,
        Err(error) => panic!("read file panic! {:?}", error),
    };
    println!("test.txt is: {}", f);
}



