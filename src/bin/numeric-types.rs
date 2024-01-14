fn sub(num1: i32, num2: i32) -> i32 {
    return num1 - num2;
}
fn main() {
    let _sum = 2 + 3;
    let _name = "vikram";
    let _subtract = 10 - 5;
    let _divide = 10 / 3;
    let _multiply = 3 * 7;

    let five = sub(10, 5);
    println!("{five:?}");
    let remainder = 6 % 3;
    println!("6 goes into 3 properly so remainder is: {remainder:?}");
}
