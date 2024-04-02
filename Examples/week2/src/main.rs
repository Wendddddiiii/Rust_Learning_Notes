// #![allow(unused)]
// const MY_SPECIAL_NUM:i32 = 42;

// fn main() {
//     let my_option: Option<i32> = Some(42);
//     // let my_option: Option<i32> = None;

//     let expression_value = match my_option {
//         Some(42) => {
//             println!("Haha, 42!");

//             42
//         }
//         Some(value) => {
//             println!("The value is {}", value);

//             value
//         }
        
//         None => {
//             println!("The value was none!");
//             999
//         }
//     };

//     if let Some(value) = my_option {
//         println!("The unwrapped value: {}", value);
//     }
//     println!("Expression value: {}", expression_value);

//     let my_result: Result<i32, String> = Err(String::from("oh no!"));
//     match my_result {
//         Ok(number) => {
//             println!("Ok! The number is {number}");
//         }
//         Err(message) => {
//             println!("Err! The message is {message}");
//         }
//     }

// }

// fn find_the_mean(vec: Vec<i32>) -> f64 {
//     if vec.is_empty() {
//         return None;
//     }
//     let mut sum = 0;
//     let len = dbg!(vec.len()) as f64;

//     for num in vec {
//         sum += num;
//     }

//     sum as f64/len
// }

// fn find_the_median(mut vec: Vec<i32>) -> Option<f64> {
//     if vec.is_empty() {
//         return None;
//     }
//     vec.sort();
//     if vec.len() % 2 == 0 {
//         //even
//         let lower_middle = vec[vec.len() / 2 - 1];
//         let upper_middle = vec[vec.len() / 2];
//         Some((lower_middle + upper_middle) as f64 / 2.0)
//     } else {
//         //odd
//         Some(vec[vec.len() / 2] as f64)
//     }
// }

// fn main() {
//     println!("find the median([2, 3, 4, 5]) = {:?}", find_the_median(vec![2, 3, 4, 5]));
//     dbg!(find_the_median(vec![2, 3, 4, 5]));
//     dbg!(find_the_median(vec![1, 2, 3, 4, 5]));
//     dbg!(find_the_median(vec![]));
// }

