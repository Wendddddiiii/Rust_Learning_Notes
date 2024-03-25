#![allow(unused)]
const MY_SPECIAL_NUM:i32 = 42;

fn main() {
    let my_option: Option<i32> = Some(42);
    // let my_option: Option<i32> = None;

    let expression_value = match my_option {
        Some(42) => {
            println!("Haha, 42!");

            42
        }
        Some(value) => {
            println!("The value is {}", value);

            value
        }
        
        None => {
            println!("The value was none!");
            999
        }
    };

    if let Some(value) = my_option {
        println!("The unwrapped value: {}", value);
    }
    println!("Expression value: {}", expression_value);

    let my_result: Result<i32, String> = Err(String::from("oh no!"));
    match my_result {
        Ok(number) => {
            println!("Ok! The number is {number}");
        }
        Err(message) => {
            println!("Err! The message is {message}");
        }
    }

}