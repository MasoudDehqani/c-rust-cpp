use std::cmp::PartialOrd;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

struct Rectangle<T: PartialOrd> {
    width: T,
    height: T,
}

impl Rectangle<i32> {
    fn can_hold(&self, other: &Self) -> bool {
        self.width > other.width && self.height > other.height
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn exploration() {
        assert_eq!(add(3, 8), 11);
    }

    #[test]
    #[should_panic(expected = "Alaki")]
    fn panic_without_fail() {
        panic!("Hamintory Alaki");
    }

    #[test]
    #[should_panic(expected = "Dolaki")]
    #[ignore]
    fn panic_whith_fail() {
        panic!("Hamintory Alaki");
    }

    #[test]
    #[should_panic]
    fn simply_panic_and_pass() {
        panic!("panic but passed");
    }

    #[test]
    fn can_hold() {
        let s = Rectangle {
            width: 2,
            height: 8,
        };
        let l = Rectangle {
            width: 32,
            height: 21,
        };

        assert!(l.can_hold(&s));
    }
}
