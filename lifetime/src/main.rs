
fn main() {
    println!("Hello, world!");
}

fn foo() {

    let really_long_lifetime: &'static str = "abc";

    let result;
    {
        {
            {
                {
                    let really_short_lifetime = String::from("helloooooo");
                    let borrow_that =  really_short_lifetime.as_str();

                    result = longest_string_2(really_long_lifetime, borrow_that);
                }
            }
        }
    }
}

fn longest_string<'a, 'b, 'c>(x: &'a str, y: &'b str) -> &'c str
where 
    //outlives relations
    //foo outlives bar
    'a: 'c,//tick A outlives tick C lifetime of a >= c
    'b: 'c,
{
    if x.chars().count() >= y.chars().count() {
        x
    } else {
        y
    }
}


//two tick A does not mean they have exact same lifetime
//Rust will shorten the longer one to be equal to the shorter one
fn longest_string_2<'a>(x: &'a str, y: &str) -> &'a str {
    if x.chars().count() >= y.chars().count() {
        x
    } else {
        y
    }
}
