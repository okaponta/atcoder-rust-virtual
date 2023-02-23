use proconio::input;

fn main() {
    input! {
        t:usize,
        nsk:[(usize,usize,usize);t],
    }
    for (n, s, k) in nsk {
        let g = gcd(n, k);
        if (n - s) % g != 0 {
            println!("-1");
            continue;
        }
        let ans = (modinv((k / g) as i64, (n / g) as i64) * ((n - s) / g)) % (n / g);
        println!("{}", ans);
    }
}

fn gcd(a: usize, b: usize) -> usize {
    match b {
        0 => a,
        _ => gcd(b, a % b),
    }
}

fn modinv(mut a: i64, modulo: i64) -> usize {
    let mut b = modulo;
    let mut u = 1;
    let mut v = 0;
    while b > 0 {
        let t = a / b;
        a -= t * b;
        std::mem::swap(&mut a, &mut b);
        u -= t * v;
        std::mem::swap(&mut u, &mut v);
    }
    u %= modulo;
    if u < 0 {
        u += modulo;
    }
    u as usize
}
