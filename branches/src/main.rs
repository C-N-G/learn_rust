// fn main() {
//     let number = 7;

//     if number < 5 {
//         println!("condition was true");
//     } else {
//         println!("condition was false");
//     }
// }

// fn main() {
//     let number = 6;

//     if number % 4 == 0 {
//         println!("number is divisible by 4");
//     } else if number % 3 == 0 {
//         println!("number is divisible by 3");
//     } else if number % 2 == 0 {
//         println!("number is divisible by 2");
//     } else {
//         println!("number is not divisible by 4, 3, or 2");
//     }
// }

fn main() {
    let condition = true;
    let number = if condition { 5 } else { 6 };
    let x;
    if condition {
        x = 1;
    } else {
        x = 2;
    }
    // Note that Rust does not require x to be initially 
    // declared with let mut in the second snippet. This is 
    // because Rust can determine that x is only ever assigned once, 
    // since only one branch of the if-statement will ever execute.

    println!("The value of number is: {number} {x}");
}