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

// trait Summary {
//     fn summarize(&self) -> String;
// }

// #[derive(Debug)]
// struct Post {
//     title: String,
//     author: String,
//     content: String,
// }

// impl Summary for Post {
//     fn summarize(&self) -> String {
//         format!("The author of post {} is {}", self.title, self.author)
//     }
// }

// #[derive(Debug)]
// struct Weibo {
//     username: String,
//     content: String,
// }

// impl Summary for Weibo {
//     fn summarize(&self) -> String {
//         format!("The content of Weibo by {} is: {}", self.username, self.content)
//     }
// }

// fn main() {
//     let post:Post = Post {
//         title: "Popular Rust".to_string(),
//         author: "Sunface".to_string(),
//         content: "Rust is awesome!".to_string(),
//     };

//     let weibo:Weibo = Weibo {
//         username: "surface".to_string(),
//         content: "weibo seems to worse than Tweet.".to_string(),
//     };

//     summary(&post);
//     summary(&weibo);

//     println!("{:?}", post);
//     println!("{:?}", weibo);
// }

// //Implement `fn summary` below.
// fn summary<T: Summary>(a: &T) {
//     let output: String = a.summarize();
//     println!("{}", output);
// }

// fn main() {
//     assert_eq!(sum(1, 2), 3);
//     println!("{}", sum(5, 5));
// }

// fn sum<T: std::ops::Add<Output = T>>(x: T, y: T) -> T {
//     x + y 
// }
// struct Pair<T> {
//     x: T,
//     y: T,
// }

// impl<T> Pair<T> {
//     fn new(x: T, y: T) -> Self {
//         Self {
//             x, 
//             y,
//         }
//     }
// }

// impl <T: std::fmt::Debug + PartialOrd + PartialEq> Pair<T> {
//     fn cmp_display(&self) {
//         if self.x >= self.y {
//             println!("The largest member is x = {:?}", self.x);
//         } else {
//             println!("The largest member is y = {:?}", self.y);
//         }
//     }
// }
// #[derive(Debug, PartialEq, PartialOrd)]
// struct Unit(i32);

// fn main() {
//     let pair: Pair<Unit> = Pair::new(Unit(9), Unit(2));
//     pair.cmp_display();
// }

// trait MyTrait {
//     type MyType;

//     fn get_my_type(&self) -> Self::MyType;
// }

// struct MyStruct {}

// impl MyTrait for MyStruct {
//     type MyType = i32;

//     fn get_my_type(&self) -> Self::MyType {
//         return 42;
//     }
// }


//array with Trait Objects
// trait Bird {
//     fn quack(&self);
// }

// struct Duck;
// impl Duck {
//     fn fly(&self) {
//         println!("Look, the duck is flying")
//     }
// }

// struct Swan;
// impl Swan {
//     fn fly(&self) {
//         println!("Look, the duck, oh, sorry, the swan is flying")
//     }
// }

// impl Bird for Duck {
//     fn quack(&self) {
//         println!("{}", "duck duck");
//     }
// }

// impl Bird for Swan {
//     fn quack(&self) {
//         println!("{}", "swan swan");
//     }
// }

// fn main() {
//     let birds: [&dyn Bird; 2] = [&Duck, &Swan]; //usize
//     for bird in birds {
//         bird.quack(); 
//     }

//     println!("success!");
// }

// fn hatch_a_bird(species: u8) -> Box<dyn Bird> {
//     match species {
//         1 => Box::new(Swan),
//         2 => Box::new(Duck),
//         _ => panic!(),
//     }
// }


// //&dyn and Box<&dyn>
// trait Draw {
//     fn draw(&self) -> String;
// }

// impl Draw for u8 {
//     fn draw(&self) -> String {
//         format!("u8: {}", self)
//     }
// }
// impl Draw for f64 {
//     fn draw(&self) -> String {
//         format!("f64:{}", self)
//     }
// }

// fn main() {
//     let x: f64 = 1.19f64;
//     let y: u8 = 8u8;


//     //draw x
//     draw_with_box(Box::new(x));

//     //draw y 
//     draw_with_ref(&y);

//     println!("success!");
// }

// fn draw_with_box(x: Box<dyn Draw>) {
//     x.draw();
// }

// fn draw_with_ref(x: &dyn Draw) {
//     x.draw();
// }


// trait Foo {
//     fn method(&self) -> String;
// }

// impl Foo for u8 {
//     fn method(&self) -> String {format!("u8: {}", *self)}
// }

// impl Foo for String {
//     fn method(&self) -> String {format!("string: {}", *self)}
// }

// fn static_dispatch<T: Foo>(a: T) {
//     a.method();
// }

// fn dynamic_dispatch(a: &dyn Foo) {
//     a.method();
// }   

// fn main() {
//     let x: u8 = 5u8;
//     let y: String = "hello".to_string();
//     static_dispatch(x);
//     dynamic_dispatch(&y);

//     println("success!");
// }


//object Safe
trait MyTrait {
    fn f(&self) -> Box<dyn MyTrait>;
}

impl MyTrait for u32 {
    fn f(&self) -> Box<dyn MyTrait> {Box::new(42)}
}

impl MyTrait for String {
    fn f(&self) -> Box<dyn MyTrait> {Box::new(self.clone())}
}

fn my_function(x: Box<dyn MyTrait>) -> Box<dyn MyTrait> {
    x.f()
}

fn main() {
    my_function(Box::new(13_u32));
    my_function(Box::new(String::from("abc")));
    println!("success!");
}