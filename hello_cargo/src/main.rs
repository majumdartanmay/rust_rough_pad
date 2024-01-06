use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number");

    
    let secret_number = rand::thread_rng().gen_range(1..=100); 
    
    loop {
        println!("Please enter your guess");
        let apples = 3;
        let mut guess = String::new(); // new is an associated function.

        std::io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read a line");
        
        
        let mut another_string = String::new();
        std::io::stdin()
            .read_line(&mut another_string)
            .expect("Unable to take input");
        
        another_string.push_str("123123");
        println!("{another_string} is another string");

        let guess: u32 = guess
            .trim()
            .parse()
            .expect("Please type a number");

        match guess.cmp(&secret_number) {
            Ordering::Less => {
                println!("Its smaller");
            },
            Ordering::Equal => {
                println!("You win");
                break;
            },
            Ordering::Greater => {
                println!("its greater")
            }
        };
            
        if guess < 20 {
            println!("We have a number < than 20. btw!");
        } else {
            println!("We have a number > 20");
        }
        
        let template = if guess > 23 {"outcome1"} else {"outcome2"};
        
        println!("The template is {template}");
        
        returning_values_from_loops();
        another_function();
        another_function_param(get_return_value());
        looping_through_for();
        another_function_param(returning_values_from_loops());
        another_function_param(returning_values_from_loop_with_labels());
        another_function_param(get_return_value2());
        while_loops();
        another_function_param(get_return_value3());
        test_scoping();
        print!("You guessed: {guess}. Also apples - {apples}");
        print!("\nAlso, I created a secret number. {secret_number}");
        scoping_test();
        
        print!("Using calculate_length");
        let str_data: String = String::from("Hello World. This is Tanmay Majumdar");
        let len_return : usize = calculate_length(&str_data);
        println!("Size returned {len_return}");
        
        let mut int_d : u32= 10;
        use_int_ref(&mut int_d);
        use_int_ref(&mut int_d);
        
        println!("Now value of int_d {int_d}");
    }
}


fn another_function() {
    println!("This is another function!");
}

fn another_function_param(x: i32) {
    println!("This is a param function. {x} . ");

}

fn get_return_value() -> i32 {
    let x = 23;
    println!("Hello World");
    return x;
}

fn get_return_value2() -> i32 {
    5
}

fn scoping_test() {

    let s = String::from("hello");  // s comes into scope

    takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here

    let x = 5;                      // x comes into scope

    makes_copy(x);                  // x would move into the function,
}

fn takes_ownership(x: String) {
    println!("Ownership taken {x}");
}

fn makes_copy(x: i32) {
    println!("X is copied {x}");
}

fn get_return_value3() -> i32 {
    let x = 23;
    x
}

//https://doc.rust-lang.org/book/ch03-05-control-flow.html#returning-values-from-loops
fn returning_values_from_loops() -> i32 {
    let mut counter = 0;
    
    let result = loop {
        if counter == 10 {
            break 2;
        }
        counter = counter + 1;
    };

    result
}

//https://doc.rust-lang.org/book/ch03-05-control-flow.html#loop-labels-to-disambiguate-between-multiple-loops
fn returning_values_from_loop_with_labels() -> i32 {
    let mut counter = 0;
    let mut result: i32 ;
    result = 'loop_label_1: loop {
        println!("Count value {counter}");
        result = loop {
            counter = counter + 1;
            if counter == 10 {
                break 101;
            }
        };
        break 'loop_label_1 result;
    };
    result
}

//https://doc.rust-lang.org/book/ch03-05-control-flow.html#conditional-loops-with-while
fn while_loops() {
    let mut i = 0;
    while i < 10 {
        println!("Value i -> {i}");
        i += 1;
    }
}

//https://doc.rust-lang.org/book/ch03-05-control-flow.html#looping-through-a-collection-with-for
fn looping_through_for() {
    let a = [1, 3, 4, 6, 8, 3];
    for elem in a {
        println!("Elem => {elem}");
    }
}

fn test_scoping() {
    let s = "test";
    let  s1 = "test2";
    let s2 = String::from("Hello World");
    println!("Values {s} {s1} {s2}");
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn use_int_ref(int_d: &mut u32) {
    println!("use_int_ref {int_d}");
}
