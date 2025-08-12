use std::collections::HashMap;

fn main() {
    let v = vec![[1, 2, 3]];
    let mut v2 = Vec::new();
    v2.push(4);
    v2.push(5);
    v2.push(6);

    let mut s = String::from("Hello");
    s.push_str(", World!");
    s.push('c');
    let _s2 = "Hello, World!".to_string();

    let mut h = HashMap::from([("key1", 1), ("key2", 2)]);
    h.insert("key3", 4);

    println!("{v:?}, {s}, {h:?}");
}
