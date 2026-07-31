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

fn main(){
    // Basic variables
    let str:String=String::from("Hi");
    let num:u8=255;

    get_char_at_index(4);



}