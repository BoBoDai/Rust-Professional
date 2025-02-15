pub fn goldbach_conjecture() -> String {
    let mut count = 0;
    let mut ret = vec![];
    for z in (9..u64::MAX).step_by(2) {
        if count == 2 {
            break;
        }
        if !is_prime(z) {
            let mut finded = false; // any odd and not prime
            for x in (3..z).step_by(2) {
                // 1 <= x < z
                if is_prime(x) {
                    // get prime x
                    let y = (((z - x) / 2) as f64).sqrt(); // get y
                    if y.fract() == 0.0 {
                        finded = true;
                        break;
                    }
                }
            }
            if !finded {
                count += 1;
                ret.push(z.to_string());
            }
        }
    }
    ret.join(",")
}

fn is_prime(num: u64) -> bool {
    if num % 6 != 1 && num % 6 != 5 {
        return false;
    }
    let sqrt_num = (num as f64).sqrt() as u64;
    for i in (5..=sqrt_num).step_by(6) {
        if num % i == 0 || num % (i + 2) == 0 {
            return false;
        }
    }
    true
}
