fn fun1(s: String) -> String {
    s
}

fn fun2(s: String) -> String {
    s
}

fn hof(f: fn(String) -> String, s: String) -> String {
    f(s)
}

fn main() {
    let f1: fn fun1(String) -> String = fun1;
    let f2: fn fun2(String) -> String = fun2;
    let prefix: &str = "prefix";
    let closure: impl Fn(String) = |s: String|
        prefix.to_string().push_str(string: &s)
    hof(f: f1, s: "!".to_string());
    hof(f: f2, s: "!".to_string());
    hof(f: closure, s: "!".to_string());

}