fn main() {
    // let mut s = String::from("hello world");
    // let word = first_word(&s);
    // s.clear();

    // let mut s = String::from("hello world");
    // let hello: &str = &s[0..5];
    // let world: &str = &s[6..11];
    // let hel = &hello[..2];
    // let lo = &hello[3..];
    // let hello2 = [..];

    let s = String::from("hello world");
    let s_lit = "hello1 world";

    let fw = first_word_slice(s_lit);

    // s.clear();

    println!("first word is: {fw}");

    let a = [1, 2, 3, 4, 5];
    let arr_slice = &a[1..3];
    assert_eq!(arr_slice, &[2, 3])
}

fn first_word_no_slice(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}
fn first_word_slice(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
