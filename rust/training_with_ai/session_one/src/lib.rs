pub fn fizzbuzz(n: u32) -> String {
    match (n % 3 == 0 && n % 5 == 0, n % 3 == 0, n % 5 == 0) {
        (true, _, _) => String::from("FizzBuzz"),
        (false, true, false) => String::from("Fizz"),
        (false, false, true) => String::from("Buzz"),
        _ => String::from(n.to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fizzbuzz() {
        assert_eq!("FizzBuzz", fizzbuzz(60));
    }
}
