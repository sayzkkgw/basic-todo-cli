use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    let todo_file: File = File::open("todos.data").unwrap();
    let todo_file_reader: BufReader<File> = BufReader::new(todo_file);
    for todo_item in todo_file_reader.lines() {
        if let Ok(item) = todo_item {
            println!("[ ] - {}", item);
        }
    }
}
