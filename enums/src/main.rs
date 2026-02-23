use std::fs;
fn main() {
    println!("Hello, world!");
    let s = String::from("hello world");
    let index = find_first_a(s);
    match index {
        Some(i) => println!("The first 'a' is at index: {}", i),
        None => println!("'a' not found in the string."),
    }

    //error handling with enums of type result => ok value or error value
    //example a function that reads content of another file
    //we dont have try catch in rust 


    let greeting = fs::read_to_string("greeting.txt"); //returning result type => ok or error
    match greeting{
        Ok(content)=> println!("File content: {}", content),
        Err(e) => println!("Error reading file: {}", e),
    }


    let result = read_file_from_diya(String::from("book.txt"));
    //match result here instead of in the function to handle error at the call site

    match result {
        Ok(content) => println!("File content: {}", content),
        Err(e) => println!("Error: {}", e),
    }       
    
}           

fn read_file_from_diya(file_path:String)->Result<String, String>{
    let content = fs::read_to_string(file_path);
    match content {
        Ok(c) => Ok(c),
        // Err(e) => String::from("Content not found"), //if we file content is also Content not found and actual content is lost
        Err(e)=> Err(format!("Error reading file: {}", e)),
    }
}

fn find_first_a(s:String)->Option<i32>{
    for (index, char) in s.chars().enumerate(){
        if char=='a'{
            return Some(index as i32);
        }
    }
    return None;
}

