use std::cmp::PartialOrd;
use std::fs::File;
use std::fmt::Debug;

pub fn main() {
    dbg!(smallest_two_i32(100, 42));
    dbg!(smallest_two_f32(3.14, 9.99));
    dbg!(smallest_two_char('a', 'z'));


    dbg!(smallest_two::<i32>(100, 42));
    dbg!(smallest_two::<f32>(10.0, 4.2));
    dbg!(smallest_two::<char>('a', 'z'));

    dbg!(smallest_two(100, 42));
    dbg!(smallest_two(10.0, 4.2)); 
    dbg!(smallest_two('a', 'z'));

    let v: Vec<i32> = vec![];
    dbg!(smallest_n(vec![2, 3, 9, 0, 8]));
    dbg!(smallest_n(vec![7]));
    dbg!(smallest_n(vec!["hello", "foo", "barr"]));


    // let f1 = File::open("foo1.txt");
}

// restrictions on X, Y, Z
// 1. Z = X (in case X is smaller)
// 2. Z = Y (in case Y is smaller)
// => X = Z = Y = type T

// 3. T must be comparable


// fn print_debug_repr(value: T) {
// where 
//     T: Debug 
//     {
//         println!("{value:?")
//     }
// }

fn smallest_n<T>(mut list: Vec<T>) -> Option<T> 
where 
    T: PartialOrd,
{
    if list.is_empty() {
        return None;
    }
    let mut smallest = list.pop().expect("Because the list is never empty");
    while let Some(item) = list.pop() {
        if item < smallest {
            smallest = item;
        }
    } 
    Some(smallest)
}


fn smallest_two<T>(x: T, y: T) -> T
where 
    T: PartialOrd,
{   
    println!("{:?}", std::any::type_name::<T>());
    if x < y {
        x
    } else {
        y
    }
}

fn smallest_two_i32(x: i32, y: i32) -> i32 {
    if x < y {
        x
    } else {
        y
    }
}

fn smallest_two_f32(x: f32, y: f32) -> f32 {
    if x < y {
        x
    } else {
        y
    }
}

fn smallest_two_char(x: char, y: char) -> char {
    if x < y {
        x
    } else {
        y
    }
}