// fn main() {
//     let mut s: String = String::from("hello, ");

//     s.push_str("world");
//     s.push('!');

//     // move_ownership(s.clone());
//     s = s.clone();
//     assert_eq!(s, "hello, world!");
//     println!("success!");
// }

// // fn move_ownership(s: String) {
// //     println!("ownership of {} is moved here", s);
// // }

// fn main() {
//     let mut s = String::from("hello, world");

//     let slice1: &str = s.as_str();
//     assert_eq!(slice1, "hello, world");

//     let slice2: &str = &s[..=4] ;
//     assert_eq!(slice2, "hello");

//     let mut slice3: String = s;
//     slice3.push('!');
//     assert_eq!(slice3, "hello, world!");
//     println!("success");
// }

fn main() {
    let s: String = String::from("hello, 世界");
    let slice1: &str = &s[..1];

    assert_eq!(slice1, "h");
    let slice2 = &s[7..10];
    assert_eq(slice2, "");

    for (i, c) in s.chars().enumerate() {
        if i == 7 {
            assert_eq!(c, '世')
        }
    }

    println!("success!");
}

