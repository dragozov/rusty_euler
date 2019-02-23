pub fn answer() -> i32 {
    diff(100)
}

fn diff(max_n: i32) -> i32 {
    let mut result = 0;
    for i in 1..(max_n + 1) {
        for j in 1..i {
            result += 2 * i * j;
        }
    }
    result
}
