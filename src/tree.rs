#[test]
fn draw_trees() {
    let height = 3;
    for _ in 0..3 {  //
        for i in 0..height {
            let spaces = " ".repeat(height - i - 1);
            let stars = "*".repeat(2 * i + 1);
            println!("{}{}", spaces, stars);
        }
    }
}