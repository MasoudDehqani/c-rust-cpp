pub fn square(s: u32) -> u64 {
    assert!(s > 0 && s <= 64);

    1 << (s - 1)
}

// pub fn square_v1(s: u32) -> u64 {
//     assert!(s > 0 && s <= 64);

//     if s < 3 {
//         return s as u64;
//     }

//     2 * square(s - 1)
// }

pub fn total() -> u64 {
    u64::MAX
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn grains_on_square_1() {
        assert_eq!(square(1), 1);
    }
    #[test]
    fn grains_on_square_2() {
        assert_eq!(square(2), 2);
    }
    #[test]
    fn grains_on_square_3() {
        assert_eq!(square(3), 4);
    }
    #[test]
    fn grains_on_square_4() {
        assert_eq!(square(4), 8);
    }
    #[test]
    fn grains_on_square_16() {
        assert_eq!(square(16), 32_768);
    }
    #[test]
    fn grains_on_square_32() {
        assert_eq!(square(32), 2_147_483_648);
    }
    #[test]
    fn grains_on_square_64() {
        assert_eq!(square(64), 9_223_372_036_854_775_808);
    }
    #[test]
    #[should_panic]
    fn square_0_is_invalid() {
        square(0);
    }
    #[test]
    #[should_panic]
    fn square_greater_than_64_is_invalid() {
        square(65);
    }
    #[test]
    fn returns_the_total_number_of_grains_on_the_board() {
        assert_eq!(total(), 18_446_744_073_709_551_615);
    }
}
