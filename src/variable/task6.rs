#[test]
fn main() {
    let mut x: i32 = 1;
    x = 7;
    // Затінення та повторне прив'язування
    let x = x;

    let y = 4;
    // Затінення
    let y = "Я також можу бути прив'язаний до тексту!";

    println!("Success!");
}