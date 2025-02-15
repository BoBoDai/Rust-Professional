use std::cmp::max;
use rand::Rng;

pub fn find_max_prime_factor(number: u128) -> u128 {
    let mut largest = number;

    while !miller_rabin(largest) {
        let factor = pollard_rho(largest);
        largest = max(largest / factor, factor);
    }
    largest
}

fn pollard_rho(n: u128) -> u128 {
    if n % 2 == 0 {
        return 2;
    }
    let mut rng = rand::rng();
    let c = rng.random_range(1..n);
    let mut x = rng.random_range(0..n);
    let mut y = f(x, c, n);
    while x != y {
        let mut mul = 1;
        for _ in 0..127 {
            x = f(x, c, n);
            y = f(f(y, c, n), c, n);
            let diff = if x > y { x - y } else { y - x };
            if diff == 0 {
                return n;
            }
            mul = (mul * diff) % n;
            if mul != 0 {
                break;
            }
        }
        let gcd = gcd(mul, n);
        if n > gcd && gcd > 1 {
            return gcd;
        }
    }
    n
}

fn f(x: u128, c: u128, n: u128) -> u128 {
    let x_2 = modular_pow(x, 2, n);
    (x_2 + c) % n
}

fn gcd(mut a: u128, mut b: u128) -> u128 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

fn miller_rabin(n: u128) -> bool {
    if n <= 1 {
        return false;
    }
    if n <= 3 {
        return true;
    }
    if n % 2 == 0 {
        return false;
    }
    let mut r = 0;
    let mut d = n - 1;
    while d % 2 == 0 {
        r += 1;
        d /= 2;
    }

    let mut rng = rand::rng();
    for _ in 0..10 {
        let a = rng.random_range(2..n - 1);
        let mut x = modular_pow(a, d, n);
        if x == 1 || x == n - 1 {
            continue;
        }
        let mut passed = false;
        for _ in 0..r - 1 {
            x = modular_pow(x, 2, n);
            if x == n - 1 {
                passed = true;
                break;
            }
        }
        if !passed {
            return false;
        }
    }
    true
}

fn modular_pow(mut base: u128, mut exp: u128, mode: u128) -> u128 {
    let mut result = 1;
    base %= mode;
    while exp > 0 {
        if exp & 1 == 1 {
            result = add_mod(result, base, mode)
        }
        base = add_mod(base, base, mode);
        exp >>= 1;
    }
    result
}
fn add_mod(mut a: u128, mut b: u128, mode: u128) -> u128 {
    let mut result = 0;
    a %= mode;
    while b > 0 {
        if b & 1 == 1 {
            result = (result + a) % mode;
        }
        a = (a + a) % mode;
        b >>= 1;
    }
    result
}