#[test]
fn main() {
    let b = true; // Зміна на true

    let _v = match b {
        true => 1,
        false => {
            println!("Success!");
            panic!("we have no value for `false`, but we can panic");
        }
    };

    println!("Exercise Failed if printing out this line!"); // Цей рядок виконається
}
