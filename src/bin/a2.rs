fn add(num1: i32, num2: i32) -> i32 {
    return num1 + num2;
}
fn display_result(result: i32) {
    println!("{result:?} is your answer!");
}
fn main() {
    display_result(add(5, 10));
}
