#[test]
fn main() {
    let unit: () = ();
    assert!(size_of_val(&unit) == 0); 

    println!("Success!");
}

use std::mem::size_of_val;
