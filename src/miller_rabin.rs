use rand::Rng;

pub fn is_prime(n: u128) -> bool {
    if n == 2 {
        return true;
    }
    if n == 1 || n & 1 == 0 {
        return false;
    }

    let mut d = n - 1;
    while d & 1 == 0 {
        d >>= 1;
    }

    let mut rng = rand::thread_rng();
    for _ in 1..=20 {
        let a = rng.gen_range(0..=(n - 2)) + 1;
        let mut t = d;
        let mut y = mod_math_pow(a, t, n);
        while t != n - 1 && y != 1 && y != n - 1 {
            y = (y * y) % n;
            t <<= 1;
        }
        if y != n - 1 && t & 1 == 0 {
            return false;
        }
    }

    true
}

fn mod_math_pow(mut base: u128, mut power: u128, mod_num: u128) -> u128 {
    let mut result = 1u128;
    while power > 0 {
        if power & 1 == 1 {
            result = (result * base) % mod_num
        }
        base = (base % mod_num) * (base % mod_num) % mod_num;
        power >>= 1;
    }
    result
}
