use std::io;

fn main() {
    let sum = 5 + 10;
    println!("The value of sum is: {sum}");

    let difference = 95.5 - 4.3;
    println!("The value of difference is: {difference}");

    let product = 4 * 30;
    println!("The value of product is: {product}");

    let quotient = 56.7 / 32.2;
    println!("The value of quotient is: {quotient}");

    let truncated = -5 / 3;
    println!("The value of truncated is: {truncated}");

    let remainder = 43 % 5;
    println!("The value of remainder is: {remainder}");

    //Tuples
    let tuple: (i32, f64, u8) = (500, 6.4, 1);

    let (x, y, z) = tuple;

    println!("The value of y is: {y}");

    let five_hundred = tuple.0;

    let six_point_four = tuple.1;

    let one = tuple.2;
    //
    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];

    let a = [1, 2, 3, 4, 5];
    println!("Enter index!");

    let mut index = String::new();
    io::stdin().read_line(&mut index).expect("Failed readln");
    let index: usize = index.trim().parse().expect("Index not int");
    let element = a[index];
    println!("Val @ {index} is {element}");
}
