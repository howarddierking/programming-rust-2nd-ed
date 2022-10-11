use std::str::FromStr;
use std::env;

fn main() {
    // Question: did we not have to pull in Vec in the above use statement?
    let mut numbers = Vec::new();

    for arg in env::args().skip(1) {
        numbers.push(u64::from_str(&arg).expect("error parsing argument"));
    }

    if numbers.len() == 0 {
        eprintln!("Usage: gcd NUMBER ...");
        std::process::exit(1);
    }

    let mut d = numbers[0];
    for m in &numbers[1..] {
        d = gcd(d, *m);
    }

    println!("The greatest common divisor of {:?} is {}", numbers, d);
}

fn gcd(mut n: u64, mut m: u64) -> u64 {
    // Question: is this the idiomatic way to handle guards
    assert!(n != 0 && m != 0);

    while m != 0 {
        if m < n {
            let t = m;
            m = n;
            n = t;
        }
        m = m % n;
    }
    // note that for this expression to be treated as a return 
    // value, you need to remember to leave off the semicolon
    //
    // idiomatic rust would suggest using this form for the end
    // of the function and only use return statements when returning
    // from the middle of the function
    n 
}

#[test]
fn test_gcd(){
    assert_eq!(gcd(14, 15), 1);

    assert_eq!(gcd(2 * 3 * 5 * 11 * 17,
                   3 * 7 * 11 * 13 * 19),
               3 * 11);
}