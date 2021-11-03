#[allow(dead_code)]
fn make_divisors(n: usize) -> Vec<usize> {
    let mut lower_divisors = vec![];
    let mut upper_divisors = vec![];
    let mut i = 1;
    while (i * i) <= n {
        if n % i == 0 {
            lower_divisors.push(i);
            if i != n / i {
                upper_divisors.push(n / i);
            }
        }
        i += 1;
    }
    lower_divisors.append(&mut upper_divisors);
    return lower_divisors;
}

#[allow(dead_code)]
fn prime_factorize(mut n: usize) -> Vec<usize> {
    let mut v = vec![];
    while n % 2 == 0 {
        v.push(2);
        n /= 2;
    }
    let mut f = 3;
    while f * f <= n {
        if n % f == 0 {
            v.push(f);
            n /= f;
        } else {
            f += 2;
        }
    }
    if n != 1 {
        v.push(n);
    }
    return v;
}

#[allow(dead_code)]
fn is_prime(x: usize) -> bool {
    if x < 2 {
        return false;
    }
    if x == 2 || x == 3 || x == 5 {
        return true;
    }
    if x % 2 == 0 || x % 3 == 0 || x % 5 == 0 {
        return false;
    }
    let mut prime = 7.;
    let mut step = 4.;
    while prime <= (x as f64).sqrt() {
        if x as f64 % prime == 0. {
            return false;
        }
        prime += step;
        step = 6. - step;
    }

    return true;
}

#[allow(dead_code)]
fn prime_factorize_count(n: usize) -> std::collections::HashMap<usize, usize> {
    let prime_facts = prime_factorize(n);
    let mut facts = std::collections::HashMap::<usize, usize>::new();
    for &j in &prime_facts {
        *facts.entry(j).or_insert(0) += 1;
    }
    return facts;
}

#[allow(dead_code)]
fn mod_pow(m: usize, n: usize, mod_n: usize) -> usize {
    if n == 0 {
        return 1;
    }
    if n % 2 == 0 {
        let x = mod_pow(m, n / 2, mod_n);
        return (x * x) % mod_n;
    } else {
        return mod_pow(m, n - 1, mod_n) * m as usize % mod_n;
    }
}

#[allow(dead_code)]
fn mod_inv(a: usize, mod_n: usize) -> usize {
    mod_pow(a, mod_n - 2, mod_n)
}
