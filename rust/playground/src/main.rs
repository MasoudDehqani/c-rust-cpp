fn main() {
    let s = String::from("some string");
    let s3 = second(s);
    println!("{s3}");

    let mut n = Box::new(1);
    println!("{n}");
    n = Box::new(2);
    *n = 3;
    println!("{n}")
}

fn second(s: String) -> String {
    let mut s2 = String::from("second string ");
    s2.push_str(&s);
    s2
}
