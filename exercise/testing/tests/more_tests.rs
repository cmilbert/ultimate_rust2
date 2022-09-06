use testing::splish;
use testing::sploosh;

#[test]
fn sploosh_and_splish_test() {
    assert_eq!(sploosh(splish(-1, 0), splish(1, 1), splish(3, 2)), 4);
}
