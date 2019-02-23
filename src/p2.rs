pub fn answer() -> i32 {
    let mut sum = 2;
    let mut odd1;
    let mut odd2 = 5;
    let mut even = 8;
    while even < 4_000_000 {
        sum = sum + even;
        odd1 = odd2 + even;
        odd2 = even + odd1;
        even = odd1 + odd2;
    }
    sum
}
