use std::fs::File;
use dialoguer::{FuzzySelect, Input};
use std::fs::read_to_string;
use std::io::Write;


fn main() {
    let modes = vec!["read","write"];
    let selection = FuzzySelect::new().with_prompt("What do you choose: ").items(&modes).interact().unwrap();
    let selected_mode = modes[selection];
    let file_name = Input::<String>::new().with_prompt("File Name: ").interact_text().unwrap();
    if selected_mode == "read"{
        let data = read_to_string(file_name).expect("Cannot open file");
        println!("{}",data);
    }
    else if selected_mode == "write"{
        let temp_name = &file_name;
        let data_to_write = Input::<String>::new().with_prompt("Enter the data to be written: ").interact_text().unwrap();
        let mut temp_file = File::create(temp_name).expect("Error creating file");
        temp_file.write(data_to_write.as_bytes()).expect("Writing Failed");
        println!("Created a file {}",temp_name);
    }
    hello_world();
}

fn hello_world(){
    println!("Hello World");
}