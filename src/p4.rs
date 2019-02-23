pub fn answer() -> (i32, i32) {
    let mut n1 = 0;
    let mut n2 = 0;

    for i in (100..999).rev() {
        let p = gen_palindrome(i);
        for j in (100..999).rev() {
            if p % j == 0 {
                n2 = j;
                n1 = p / n2;
                if n1 < 1000 {
                    break;
                }
                n1 = 0;
                n2 = 0;
            }
        }
        if n1 > 0 {
            break;
        }
    }
    (n1, n2)
}

fn gen_palindrome(n: i32) -> i32 {
    let nstr = n.to_string();
    let s = format!("{}{}", nstr, nstr.chars().rev().collect::<String>());
    s.parse().unwrap()
}
