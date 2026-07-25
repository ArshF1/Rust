fn main() {
    let mut num:u16=4;
    print!("The number is {}\n", num);

    for i in 1..5 {
        if i%2==0 {
            println!("{}",i);
        }
    }

    while num>2 {
        println!("{}",num);
        num=num-1;
    }

    let str=String::from("Hi, how are you?");
    println!("The string to be printed is '{}'", str);

    print!("Let's try printing the first letter of the string str\n");
    let index=20;
    let char=str.chars().nth(index);

    // This is a safe way of getting a value at a particular index in a string
    match char{
        Some(c)=> println!("The {} character of '{}' is {}", index, str, c),
        None=> println!("No character at {}", index),
    }

    // The unwrap function below will throw a runtime error, because of a non existend index
    // println!("The {} character of '{}' is {}", index, str, char.unwrap());

    // Conditionals
    let is_even:bool=false;

    if is_even {
        println!("The number is even");
    }
    else if !is_even {
        println!("The number is not even");
    }

    let sentence=String::from("Hello");
    let first_word=get_first_word(sentence);
    println!("The first word is '{}'", first_word);

    let num1=4;
    let num2=6;

    let total=add(num1, num2);
    println!("The sum of {} and {} is {}", num1, num2, total);
    multiply(num1, num2);
    

    println!("===============================");
    // Ownership
    // In the below example, s1 had it's own owner, when we wrote s2=s1, s1 went out of scope, now the ownership got transferred from s1 to s2
    // If you need the behaviour of s2=s1, you will have to use s1.clone(), it creates an entirely new variables
    // This happens only for the heap memory
    let s1=String::from("Hi, I am sentence 1");
    let s2=s1;
    // println!("This is Sentence 1 {}", s1);
    println!("Sentence 1 is being borrowed by s2, '{}'", s2);
    
    // This behaviour is the same for functions as well
    let s3=String::from("Hi, I am sentence 3");
    println!("The string s3 is stored at pointer-> {:p}", &s3);
    println!("Before transfer: {}", s3);
    let _owner=take_ownership(s3);
    // println!("After transfer: {}", s3);
    println!("===============================");
    
    // Borrowing and references
    // Mutable references

    // Here to update a variable in a heap (in this case s4), you need to provide a mutable reference
    let mut s4=String::from("I am sentence 4.");
    update_str(&mut s4);

    // Structs
    struct User {
        name: String,
        age: u8,
        active: bool
    }
    let name=String::from("arsh");
    let user_details: User= User { name: name, age: 25, active: true };
    println!("Name is {}, age is {}, He/She is active-> {}", user_details.name, user_details.age, user_details.active);


    struct Rectangle {
        height: u32,
        width: u32
    }

    impl Rectangle {
        fn perimater(&self)->u32 {
            println!("The perimeter of the rectangle is {}", 2*(self.height+self.width));
            return 2*(self.height+self.width);
        }
    }

    let rect: Rectangle= Rectangle { height: 32, width: 20 };
    rect.perimater();

    // Enums
    #[derive(Debug)]
    enum Direction {
        North,
        South,
        East,
        West
    }
    let direction: Direction=Direction::North;
    println!("The direction is {:?}.", direction);

}

fn update_str(str: &mut String)-> () {
    str.push_str("Pushed right after sentence 4");
}

fn take_ownership(str:String)-> () {
    println!("{}", str);
}

fn get_first_word(sentence:String) -> String {
    let mut ans=String::from("");
    for char in sentence.chars() {
        ans.push_str(char.to_string().as_str());
        if char == ' ' {
            break;
        }
    }
    return  ans;
}

fn add(num1:u16, num2:u16)->u16 {
    let total:u16=num1+num2;
    return total;
}

fn multiply(num1:u16, num2:u16) ->u16 {
    let total=num1*num2;
    println!("The multiplication of {}x{} is {}", num1, num2, total);
    return total;
}
