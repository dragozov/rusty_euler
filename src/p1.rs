pub fn answer() -> i32 {
    let mut sum = 0;
    for n in 1..1000 {
        if n % 3 == 0 || n % 5 == 0 {
            sum = sum + n;
        }
    }
    sum
}
