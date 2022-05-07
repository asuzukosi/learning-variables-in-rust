
const MAX_VALUE: u32 = 100_000;
use std::io;

fn main() {
    let scanner = io::stdin();

    let mut response = String::from("");

    println!("Type in a message: ");
    scanner.read_line(&mut response).expect("Failed to read type");

    println!("The response is {}", response);

    let mut x = 5;
    println!("The value of x is {}", x);

    x = 7;
    println!("The value of x is {}", x);
    println!("The maximum value is {}", MAX_VALUE);

    // using shadowing for variable mutability in rust
    let y = 5; 
    println!("The value of y is {}", y);

    // using if statements in rust

    if x < 5 {
        println!("X is less than 5");
    }else {
        println!("X is more than 5");
    }


    // variable y is being shadowed to an entirely different type
    let y = "Pink panther";
    println!("The value of y is {}", y);


    // Examples of floating point numbers in action in rust

    let x = 2.0; // f64
    let y: f32 = 3.0; // f32

    println!("The value of x is {}", x);
    println!("The value of y is {}", y);


    let my_bool = true;
    let another_bool: bool = false;

    println!("My first boolean value is : {}, my second boolean value is {}", my_bool, another_bool);

    // Using tuples in rust
    let sample_tup: (&str, u16) = ("Hello", 21);
    println!("The first value in the sample tuple is : {}", sample_tup.0);

    let (x, y) = sample_tup;

    println!("The value of x is {} and the value of y is {}", x, y);

    // create a tuple and use each index to create a new variable
    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;

    println!("the values are {}, {} and {}",  five_hundred, six_point_four, one);

    // using arrays in rust
    let my_arr = [0, 1, 2, 3, 4];
    println!("{:?}", my_arr);

    // using functions in rust
    let x = 6;
    say_something_nice(&x, 10);

    let five_value = five();

    println!("five value gotten from a fuction {}", five_value);

    // using else if statements in rust
    if five_value % 5 == 0 && five_value % 3 == 0 {
        println!("FizzBuzz");
    } else if five_value % 5 == 0 {
        println!("Buzz");
    } else if five_value % 3 == 0 {
        println!("Fizz");
    }


    // using conditionals to assign variables in rust
    let condition = true;
    let number = if condition {
                    5
                } else {
                    6
                };

    println!("The value of number is: {}", number);

    let mut num_times = 5;

    loop {
        println!("This is the message again");
        num_times -= 1;

        if num_times < 0 {
            break;
        }
    }

    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number = number - 1;
        }
    println!("LIFTOFF!!!");

    // using for loop to iterate through values in a list

    let my_list = [2, 4, 3, 2, 5, 7];

    for a in my_list.iter() {
        println!("The value is {}", a);
    }

    for number in 1..4 {
        println!("{}!", number);
        }
        println!("LIFTOFF!!!"); 

    let my_range_arr = [1..4];
    
    println!("{:?}", my_range_arr)
    
}


// declaring functions in rust
fn say_something_nice(x: &u32, y: u32) {
    println!("This is me saying something nice {}", x);
    println!("The value of y is {}", y);
}

// functions with return values
fn five() -> u32 {
    15
}