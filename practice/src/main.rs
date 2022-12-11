use std::io;

fn main() {

    let a = [1, 2, 3, 4, 5];
    let mut index = 0;
    for element in a {
        println!("the value is: {element}");
    }

    while index < 5 {
        println!("the value is: {}", a[index]);
        index += 1;
    }

    for number in (1..6).rev() {
        println!("{number}");
    }
    println!("Hello, world!");

    for number in (1..100).rev() {
        print!("{number}: ");
        if number % 15 == 0 {
            println!("Number divisible by 3 and 5")
        } else if number % 5 == 0 {
            println!("Divisible by 5");
        } else if number % 3 == 0 {
            println!("Only by 3");
        } else {
            print!("NO ");
        }
    }
    another_function(7);
    print_labeled_measurement(45, 'm');
    let x = five();
    println!("x is equal to: {}", x);
    let y = times_five(7);
    println!("x is NOW equal to: {}", y);

    println!("Please enter an array index.");
    let mut index = String::new();
    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

        let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");

    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is {}.", s2, len);

    let s3 = String::from("holala");

    let len = calculate_length2(&s3);

    println!("The length of '{}' is {}.", s3, len);

    let mut na = String::from("Carlos");

    println!("{na}");

    change(&mut na);

    println!("{na}");

    value_in_cents(Coin::Quarter(UsState::Alaska));

    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {}", max),
        _ => (),
    }

    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }
    
    let coin = Coin::Quarter(UsState::Alabama);

    let mut count = 0;

    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }

    println!("{count}");
}

fn another_function(x: i32) {
    println!("THE VALUE IS {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("THE MEASUREMENT IS: {value}{unit_label}");
}

fn five() -> i32 {
    return 5;
}

fn times_five(x: i32) -> i32 {
    return x*5;
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}

fn calculate_length2(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(" Escalante")
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}










