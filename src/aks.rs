use std::collections::HashSet;

pub fn is_prime(n: u128) -> bool {
    if n == 1 || n & 1 == 0 {
        return false;
    }
    if is_perfect_power(n) {
        return false;
    }
    let r = enough_order_modulo(n);
    for a in 2..std::cmp::min(r + 1, n) {
        if n % a == 0 {
            return false;
        }
    }
    if n <= r {
        return true;
    }
    for a in 1..((tortient(r) as f64).sqrt() * (n as f64).log(std::f64::consts::E)) as u128 + 1 {
        if !is_congruent(a, n, r) {
            return false;
        }
    }
    true
}

fn is_perfect_power(n: u128) -> bool {
    let size = n.log2() + 1;
    for k in 2..size {
        let pn = n.pow(1 / k);
        if pn.pow(k) == n || (pn + 1).pow(k) == n {
            return true;
        }
    }
    false
}

fn enough_order_modulo(n: u128) -> u128 {
    let a = (n as f64).log(std::f64::consts::E);
    let a = a * a;
    for r in 1..n {
        let mut order = 0;
        let mut prod = 1;
        for e in 1..r {
            prod = prod * n % r;
            if prod == 1 {
                order = e;
                break;
            }
        }
        if order > (a as u128) {
            return r;
        }
    }
    n
}

fn is_congruent(a: u128, n: u128, r: u128) -> bool {
    let ls = vec![a, 1];
    let ls1 = pow(n, n, &ls, r);
    let i = (n % r) as usize;
    let mut ls2 = vec![0; i + 1];
    ls2[0] = a % n;
    ls2[i] = 1;
    ls1 == ls2
}

fn pow(n: u128, m: u128, ls: &[u128], r: u128) -> Vec<u128> {
    if m == 1 {
        return ls.to_owned();
    }
    if m % 2 == 0 {
        let pls = pow(n, m / 2, ls, r);
        product(&pls, &pls, n, r)
    } else {
        let pow1 = pow(n, m - 1, ls, r);
        let pow2 = pow(n, 1, ls, r);
        product(&pow1, &pow2, n, r)
    }
}

fn product(ls1: &[u128], ls2: &[u128], n: u128, r: u128) -> Vec<u128> {
    let r_usize = r as usize;
    let mut res: Vec<u128> = vec![0; std::cmp::min(ls1.len() + ls2.len() - 1, r_usize)];
    for i in 0..ls1.len() {
        for j in 0..ls2.len() {
            res[(i + j) % r_usize] += ls1[i] * ls2[j];
        }
    }
    for k in (0..res.len()).into_iter().rev() {
        res[k] %= n;
        if k == res.len() - 1 && res[k] == 0 {
            res.remove(k);
        }
    }
    res
}

fn tortient(r: u128) -> u128 {
    let ps = primes(r);
    let mut res = r;
    for p in ps {
        res = res * (p - 1) / p;
    }
    res
}

fn primes(r: u128) -> HashSet<u128> {
    let mut n = r;
    let mut res = HashSet::new();
    for p in 2..(((r as f64).sqrt() as u128) + 1) {
        while n % p == 0 {
            res.insert(p);
            n /= p;
        }
    }
    res
}
