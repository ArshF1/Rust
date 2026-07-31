use std::fs;

fn for_loop()-> () {
    for i in 0..3 {
        println!("Loop value: {}", i);
    }
}

fn while_loop()-> () {
    // We need to declare the var value as mutable in order to update it
    let mut value=10;
    while value>1 {
        println!("Inverted loop value: {}", value);
        value=value-1
    }
}

fn conditionals()-> () {
    let is_even=true;
    if is_even {
        println!("It is even");
    }
    else {
        println!("It is odd");
    }
}

fn get_char_at_index(index:usize)-> () {
    let word:String=String::from("Czechoslovakia");
    let char=word.chars().nth(index);

    // This is called graceful handling
    match char {
        Some(c)=> println!("The character at index {} is {}", index, c),
        None=> println!("No character at index {}", index),
    }
}

fn get_first_word(sentence:String)-> String {
    let mut word=String::from("");
    for char in sentence.chars() {
        word.push_str(&char.to_string().as_str());
        if char==' ' {
            break ;
        }
    }
    return word;
}

fn take_ownership()-> () {
    let s1=String::from("Hi");
    let s2=s1;

    let response:Result<String, String>=Ok(s2);

    match response {
        Ok(content)=> println!("Response ok {}", content),
        Err(err)=> println!("Error: {}", err),
    }
}

fn error_handling()-> () {
    let file_buffer=fs::read_to_string("test.txt");

    match file_buffer {
        Ok(content)=> println!("The contents of file are {}", content),
        Err(err)=> println!("No file found. Error: {}", err),
    }
}

fn update_str_reference(str: &mut String)->String {
    // This is regarding a mutable reference
    str.push_str(" This is pushed right after sentence");
    let response=str.to_string();
    println!("The updated sentence is '{}'", response);
    return response
}

fn main(){
    // Basic variables
    let _str:String=String::from("Hi");
    let _num:u8=255;

    get_char_at_index(4);
    take_ownership();
    error_handling();

    let mut sentence=String::from("Hi, this is a sentence");
    update_str_reference(&mut sentence);


}