use std::{collections::HashMap, io};
fn main() {
    let mut memo: HashMap<u128, u128> = HashMap::new();

    loop {
        let mut num = String::new();
        io::stdin().read_line(&mut num).expect("Failed reading;");
        let num = match num.trim().parse() {
            Ok(num) => {
                if num > 186 {
                    println!("Max fib number is 186.");
                    continue;
                } else {
                    num
                }
            }
            Err(_) => continue,
        };
        let n_th_num = get_fib_num(num, &mut memo);
        println!("{num} fib number is {n_th_num}");
    }
}

fn get_fib_num(n: u128, memo: &mut HashMap<u128, u128>) -> u128 {
    let has_val = memo.contains_key(&n);
    println!("{has_val} for {n}");
    if has_val {
        let memoed_val = memo.get(&n);
        return *memoed_val.unwrap();
    }
    if n <= 1 {
        return n;
    }
    let num = get_fib_num(n - 1, memo) + get_fib_num(n - 2, memo);
    memo.insert(n, num);
    return num;
}
