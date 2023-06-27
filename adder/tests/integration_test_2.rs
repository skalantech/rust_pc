use adder;

mod common2;

#[test]
fn sum() {
    common2::setup();
    assert_eq!(4, adder::add(2,2));
}

