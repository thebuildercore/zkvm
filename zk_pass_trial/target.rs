// target.rs
#[no_mangle]
pub fn check_price(price: u32) -> u32 {
    let result: u32;
    
    // Here is the branch that ruins ZK performance
    if price > 100 {
        result = 5;
    } else {
        result = 9;
    }
    
    return result;
}