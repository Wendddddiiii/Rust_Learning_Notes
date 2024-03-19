pub use std::string::String;
pub mod util; //under same folder of main
mod util;
mod foo;
// fn main() {
//     crate::String;
// }
use foo::a::hello;
use bar::b::hello;
use mylib::my_favo_number;


fn main() {
    let the_number: i32 = my_favo_number();
    println!("Hello, world!");

    let mut studnet = hello::foo();
    // student.name = String::from("hahahaha");
    let name: &str = studnet.name();
    let x: i32 = 42;
    let y: i32 = util::add_5(x);
    dbg!(y);

    foo::a::hello();
    foo::b::hello();
}


pub mod hello {
    pub fn foo() ->  Student {
        println!("foo");
        Student{
            name: String::from("foo"),
            zid: 5555555,
            wam: None,
        }
    }

    pub struct Student {
        pub name: String,
        zid: u32,
        wam: Option<f64>,
    }

    impl Student {
        pub fn name(&self) -> &str {
            &self.name
        }

        pub fn zid(& self) -> &u32 {
            &self.zid
        }
    }

    mod world {
    }
}