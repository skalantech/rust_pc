use std::thread;
use std::time::Duration;
use std::io;
use rand::Rng;

fn generate_workout(intensity: u32, random_number: u32) {
    let expensive_closure = |num: u32| -> u32 {
        println!("Calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_closure(intensity));
        println!("Next, do {} situps!", expensive_closure(intensity));
    } else {
        if random_number == 3 {
            println!("Take a brake today! Remember to stay hydrated!");
        } else { 
            println!(
                "Today, run for {} minutes!",
                expensive_closure(intensity)
            );
        }
    }
}

fn main() {

    loop {
        println!("Enter the intensity of your training");
        let mut simulated_user_specified_value = String::new();

        let simulated_random_number = rand::thread_rng().gen_range(2..=3);

        io::stdin()
            .read_line(&mut simulated_user_specified_value)
            .expect("Failed to read line");

        let simulated_user_specified_value: u32 = match simulated_user_specified_value.trim().parse() { 
            Ok(num)=>num, 
            Err(_)=> {  println!("Number is required");
                        break; 
                    },
        };

        generate_workout(simulated_user_specified_value, simulated_random_number);
    }
}
