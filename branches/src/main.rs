fn main() {
    // IFS
    // let number = 8;

    // print!("Divisible by ");

    // if number % 4 == 0 {
    //     println!("4");
    // } else if number % 3 == 0 {
    //     println!("3");
    // } else if number % 2 == 0 {
    //     println!("2");
    // } else {
    //     println!(" NOTHING@");
    // }
    // Dynamic bound variables
    // let condition = true;
    // let dynamic_bound = if condition { 5 } else { 6 };

    // println!("The dynamic bound value of number is: {dynamic_bound}");

    //Loops
    // let mut index = 0;

    // loop {
    //     println!("again!");
    //     index = index + 1;
    //     if index == 5 {
    //         break;
    //     } else {
    //         if index >= 3 {
    //             continue;
    //         } else {
    //             println!("less than 3 {index}")
    //         }
    //     }
    // }

    // let mut index = 0;
    // let res = loop {
    //     index += 1;
    //     if index == 10 {
    //         break index * 3;
    //     }
    // };
    // println!("Returned index {res} ")

    // LOOP LABELS
    // let mut index = 0;
    // 'up_count: loop {
    //     println!("index = {index}");
    //     let mut remaining = 10;
    //     loop {
    //         println!("remaining = {remaining}");
    //         if remaining == 9 {
    //             break;
    //         }
    //         if index == 2 {
    //             break 'up_count;
    //         }
    //         remaining -= 1;
    //     }
    //     index += 1;
    // }
    // println!("End count = {index}")

    //WHILE
    // let mut number = 3;
    // while number != 0 {
    //     println!("{number}!");
    //     number -= 1;
    // }
    // println!("KABLOW");

    //FOR
    // let a = [10, 20, 30, 40, 50];
    // let mut index = 0;
    // while index < 5 {
    //     println!("value @ index {} is {}", index, a[index]);
    //     index += 1;
    // }

    // for el in a {
    //     println!("val is {el}")
    // }

    //RANGE
    for number in (1..4).rev() {
        println!("NUM {number} ");
    }
    println!("CRAB");
}
