mod common;

use common::add_two;
use writing_automated_tests::add;

#[test]
fn some_test() {
    assert_eq!(add_two(2), add(2, 2))
}
