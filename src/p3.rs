pub fn answer() -> u64 {
    let mut number: u64 = 600851475143;
    let mut i = 3;
    let mut result = 0;
    loop {
        println!("Checking {} {}", number, i);
        while number % i == 0 {
            println!("next prime {}", i);
            result = i;
            number = number / i;
        }
        if number >= i + 2 {
            i += 2;
        } else {
            break;
        }
    }
    result
}
