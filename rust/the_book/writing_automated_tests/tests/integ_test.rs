mod common;

use common::{
    add_two,
    helper::{another_helper, some_helper},
};
use writing_automated_tests::add;

#[test]
fn some_test() {
    assert_eq!(add_two(2), add(2, 2))
}

#[test]
fn test_helper() {
    assert_ne!(some_helper(3), 10)
}

#[test]
fn another_test_helper() -> Result<(), String> {
    if another_helper(4) == 16 {
        Ok(())
    } else {
        Err("Not 16".to_string())
    }
}
