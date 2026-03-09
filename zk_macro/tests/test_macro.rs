// test 2
 use zk_macro::zk_optimize;

// We slap our new macro on top of a standard function
#[zk_optimize]
fn check_price(price: u32) -> u32 {
    if price > 100 {
        return 5;
    } else {
        return 9;
    }
}

#[test]
fn dummy_test() {
    // Just a dummy test to force the compiler to run
    assert_eq!(check_price(150), 5);
}
