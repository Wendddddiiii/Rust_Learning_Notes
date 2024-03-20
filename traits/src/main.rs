// struct Animal {
//     fn sound(&self) -> String;
// }

// struct Sheep;
// struct Cow;

// impl Animal for Sheep {
//     fn sound(&self) -> String {
//         String::from("Math")
//     }
// }

// impl Animal for Cow {
//     fn sound(&self) -> String {
//         String::from("Math")
//     }
// }


// trait Hello {
//     fn say_hi(&self) -> String {
//         String::from("hi")
//     }

//     fn say_something(&self) -> String {
//         String::from("I am not a good teacher")
//     }
// }

// struct Student {}
// impl Hello for Student {
//     fn say_something(&self) -> String {
//         String::from("I am a good student")
//     }
// }

// struct Teacher {}
// impl Hello for Teacher {
//     fn say_hi(&self) -> String {
//         String::from("Hi, I am your new teacher")
//     }
//     fn say_something(&self) -> String {
//         String::from("I am not a bad person")
//     }
// }

// fn main() {
//     let s: Student = Student{};
//     assert_eq!(s.say_hi(), "hi");
//     assert_eq!(s.say_something(), "I am a good student");

//     let t: Teacher = Teacher {};
//     assert_eq!(t.say_hi(), "Hi, I am your new teacher");
//     assert_eq!(t.say_something(), "I am not a bad person");

//     println!("success");
// }

// #[derive(PartialEq, PartialOrd)]
// struct Centimeters(f64);

// #[derive(Debug)]
// struct Inches(i32);

// impl Inches {
//     fn to_centimeters(&self) -> Centimeters {
//         let &Inches(inches) = self;

//         Centimeters(inches as f64 * 2.54)
//     }
// }

// // Add some attributes to make the code work
// #[derive(Debug, PartialEq, PartialOrd)]
// struct Seconds(i32);
// fn main() {
//     let _one_seconds: Seconds = Seconds(1);
//     println!("One second looks like: {:?}", _one_seconds);
//     let _this_is_true = _one_seconds > _one_seconds; 
//     let _this_is_true = _one_seconds == _one_seconds;

//     let foot: Inches = Inches(12);
//     println!("One foot equals {:?}", foot);

//     let meter: Centimeters = Centimeters(100.0);

//     let cmp = 
//         if foot.to_centimeters() < meter {
//             "smaller"
//         } else {
//             "bigger"
//         };

//     println!("One foot is {} than one meter.", cmp);

// } 


// use std::ops;

// struct Foo;
// struct Bar;

// #[derive(PartialEq, Debug)]
// struct FooBar;
// #[derive(PartialEq, Debug)]
// struct BarFoo;

// //Foo + Bar -> Foo.add(Bar)
// //Foo + Foo -> Foo.add(Foo)
// impl ops::Add<Self> for Foo {
//     type = Output = FooBar;

//     fn add(self, _rhs: Bar) -> FooBar {
//         FooBar
//     } 
// }

// //Foo - Bar -> Foo.sub(Bar)
// //Foo + Foo -> Foo.add(Foo)
// impl ops::Sub<Bar> for Foo {
//     type = Output = FooBar;

//     fn sub(self, _rhs: Foo) -> BarFoo{
//         BarFoo
//     }
// }

// fn main() {
//     assert_eq!(Foo + Bar, FooBar);
//     assert_eq!(Foo - Bar, BarFoo);

//     println!("success!");
// }

trait Summary {
    fn summarize(&self) -> String;
}

#[derive(Debug)]
struct Post {
    title: String,
    author: String,
    content: String,
}

impl Summary for Post {
    fn summarize(&self) -> String {
        format!("The author of post {} is {}", self.title, self.author)
    }
}

#[derive(Debug)]
struct Weibo {
    username: String,
    content: String,
}

impl Summary for Weibo {
    fn summarize(&self) -> String {
        format!("The content of Weibo by {} is: {}", self.username, self.content)
    }
}

fn main() {
    let post:Post = Post {
        title: "Popular Rust".to_string(),
        author: "Sunface".to_string(),
        content: "Rust is awesome!".to_string(),
    };

    let weibo:Weibo = Weibo {
        username: "surface".to_string(),
        content: "weibo seems to worse than Tweet.".to_string(),
    };

    summary(&post);
    summary(&weibo);

    println!("{:?}", post);
    println!("{:?}", weibo);
}

//Implement `fn summary` below.
fn summary<T: Summary>(a: &T) {
    let output: String = a.summarize();
    println!("{}", output);
}