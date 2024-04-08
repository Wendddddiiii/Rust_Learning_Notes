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


// fn longest_common_subsequence(x: Vec<i32>, y: Vec<i32>) -> usize {
//     let i = 0;
//     let shorter_len = usize::min(x.len(), y.len());

//     let mut current_run = 0;
//     let mut longest_run = 0;
//     for mut i in 0..shorter_len {
//         let a = x[i];
//         let b = y[i];


//         if a == b {
//             current_run += 1;
//             if current_run > longest_run {
//                 longest_run = current_run;
//             }

//         } else {
//             current_run = 0;
//         }
//         i += 1;
//     }
//     longest_run
// }


// //Imperative(longest subsequence) code: Explaining how to do your task
// //Functional code: Desrcibing how to do your task
// fn longest_equal_run_function(x: Vec<i32>, y: Vec<i32>) -> usize {
//     x.into_iter().zip(y.into_iter())
//         .map(|(a, b)| a == b)
//         .fold((0, 0), |(current, longest), eq| {
//             if eq {
//                 let current = current + 1;
//                 let longest = usize::max(current, longest);
//                 (current, longest)
//             } else {
//                 (0, longest)
//             }
//         }).1
//         //The .1 at the end of the fold call is used to extract the second element of this tuple
// }

// fn main() {
//     dbg!(longest_common_subsequence(vec![6, 9,9,1,1, 2, 3], vec![1, 9,9, 1,3,2,3]));
//     dbg!(longest_common_subsequence(vec![1, 2, 3], vec![1]));
//     dbg!(longest_common_subsequence(vec![1, 2, 3], vec![4]));
//     dbg!(longest_equal_run_function(vec![1, 2, 3], vec![1, 2]));

//     if true { return }


// }


// zero cost abstractions
//1. if do not use, do not need pay for it 
//2. if use, should be as performanat as handwritten codes
//

// use std::fs::File;
// use std::io::Read;

// fn main() {
//     let file = File::open("hello.txt").unwrap();
//     let file: File = match File::open("Hello.txt") {
//         Ok(file) => file,
//         Err(err) => {
//             eprintln!("Failed to open the file: {err:?}");
//             return;
//         }
//     }

//     let mut line = String::new();
//     match file.read_to_string(&mut line) {
//         Ok(_) => {}
//         Err(err) => {
//             eprintln!("Failed to read from the file: {err:?}")
//             return;
//         }
//     }

//     println!("Line: {line}");

// }


// #![allow(unused)]
// fn main() {
//     enum Outer {
//         Double(Option<u8>, Option<String>),
//         Single(Option<u8>),
//         Empty
// }


// let get_inner = Outer::Double(None, Some("hello".to_string()));
//     match get_inner {
//         Outer::Double(None, Some(ref st)) if !st.is_empty() => println!("{}",st),
//         Outer::Single(opt) => println!("{opt:?}"),
//         _ => panic!(),
//     }
// }


extern crate bmp;
use bmp::{Image, Pixel};

fn main() {
    let mut img = bmp::Image::new(100, 100);
    for (x, y) in img.coordinates() {
        img.set_pixel(x, y, bmp::consts::BLUE);
    }

    img.save("output.bmp").unwrap();
    println!("Image saved");
}
