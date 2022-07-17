use std::io;
//use std::cmp::Ordering;
//use std::io::Write;
//use std::{thread, time};

fn get_user_number_input(range_min:u128, input_message:String) -> u128 {
    let mut token = String::new();

    return loop {
        println!("{}", input_message);

        token.clear();
        io::stdin()
            .read_line(&mut token)
            .expect("Failed to read line");

        break match token.trim().parse() {
            Ok(number) if number > range_min => number,
            Ok(_) => {
                println!("Number too small.");
                continue;
            }
            Err(_) => {
                println!("Please enter a valid number");
                continue;
            }
            }
        };
}

fn main() {
    let range_min:u128 = 1;
    let max_range_input_message = String::from("Enter a max range value: ");
    let fizz_value_input_message = String::from("Enter a value for 'fizz': ");
    let buzz_value_input_message = String::from("Enter a value for 'buzz': ");

    //Get max number
    let range_max = get_user_number_input(range_min, max_range_input_message);
    let fizz_value = get_user_number_input(range_min, fizz_value_input_message);
    let buzz_value = get_user_number_input(range_min, buzz_value_input_message);

    //Temporary Value
    let mut output: String;

    for n in range_min..(range_max+1) {
        //Reset Output
        output = String::from("");

        //Concatenate Fizz/Buzz/Number
        if n % fizz_value == 0 {
            output += "Fizz";
        }

        if n % buzz_value == 0 {
            output += "Buzz";
        }

        if output == "" {
            output += &n.to_string();
        }
        
        println!("{}", output);
    }

}
