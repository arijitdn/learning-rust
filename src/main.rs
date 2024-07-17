fn main() {
    // enum Option<T> {
    //     Some(T),
    //     None,
    // }

    let some_number = Some(5);
    let some_string = Some("hi there");

    let absent_number: Option<i32> = None;


    let x: i8 = 5;
    let y: Option<i8> = Some(8); // Or None

    let sum = x + y.unwrap_or(0);
}