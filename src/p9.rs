pub fn answer() -> (u32, u32, u32) {
    let (mut r1, mut r2, mut r3) = (0, 0, 0);

    for c in 1..499 {
        for b in 1..(1000 - c - 1) {
            let a = 1000 - b - c;
            if c * c == a * a + b * b {
                r1 = a;
                r2 = b;
                r3 = c;
                break;
            }
        }
        if r1 > 0 {
            break;
        }
    }
    (r1, r2, r3)
}
