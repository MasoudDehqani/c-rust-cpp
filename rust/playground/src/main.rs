fn main() {
    let opt = Some(String::from("hello"));

    let len = match &opt {
        Some(s) => s.len(),
        None => 0,
    };

    // let len = match opt {
    //     Some(ref s) => s.len(),
    //     None => 0,
    // };

    // let len = match opt.as_ref() {
    //     Some(s) => s.len(),
    //     None => 0,
    // };

    println!("{:?}", opt);
    println!("{}", len)
}

// struct Rust {
//     name: String,
//     ver: f64,
// }

// impl Rust {
//     fn print_name(&self) {
//         println!("{}", self.name)
//     }

//     fn print_version(&self) {
//         println!("{}", self.ver)
//     }
// }

// fn main() {
//     let rust = Rust {
//         name: String::from("rust"),
//         ver: 1.19,
//     };

//     rust.print_name();
//     rust.print_version();
// }

// fn main() {
//     let rust_string = String::from("rust");

//     print_rust(rust_string);

//     println!("{rust_string}");
// }

// fn print_rust(s: String) {
//     println!("{}", s + " prigramming language");
// }

// fn main() {
//     let s = String::from("some string");
//     let s3 = second(s);
//     println!("{s3}");

//     let mut n = Box::new(1);
//     println!("{n}");
//     n = Box::new(2);
//     *n = 3;
//     println!("{n}")
// }

// fn second(s: String) -> String {
//     let mut s2 = String::from("second string ");
//     s2.push_str(&s);
//     s2
// }
