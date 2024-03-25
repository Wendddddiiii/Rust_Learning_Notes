#![allow(unused)]

static My_array: [i32; 3] = [1, 2, 3];

fn main() {
    let string_literal = "foo";
    let string_slice_borrow = &My_array;
    let static_i32_borrow = Box::leak(Box::new());

    let hello = String::from("hello");
    foo(&mut hello);
    dbg!(hello);
    println!("Hello, World!");
}

fn foo(s: &mut str) {
    s.index_mut(0);
}

fn bool_to_str(b: bool) -> &'static str {
    if b {
        "true"
    } else {
        "false"
    }
}


