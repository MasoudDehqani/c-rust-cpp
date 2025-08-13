use session_one::fizzbuzz;

fn main() {
    let x = 5;
    let x = x + 1;

    println!("{}", x);

    let mut y = 0;

    loop {
        y += 1;
        if y > 5 {
            break;
        }
    }

    println!("{}", y);

    const GRADE_STR: &str = "Your grade is";
    match grade(48) {
        Ok(g) => println!("{GRADE_STR}: {g}"),
        Err(e) => println!("{GRADE_STR}: {e}"),
    }

    println!("{}", fizzbuzz(60));
}

fn grade(n: i32) -> Result<char, String> {
    match n {
        0..=20 => Ok('E'),
        21..=40 => Ok('D'),
        41..=60 => Ok('C'),
        61..=80 => Ok('B'),
        81..=100 => Ok('A'),
        _ => Err("Out of range".to_string()),
    }
}
