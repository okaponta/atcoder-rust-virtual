use proconio::input;

const MOD: usize = 1_000_000_007;

fn main() {
    input! {
        n:usize,
        a:usize,
        b:usize,
    }
    let mut ans = ModInt::new(2).pow(n);
    ans -= ModInt::one();
    let comb = Comb::init(n, a.max(b), MOD);
    ans -= ModInt::new(comb.get(a));
    ans -= ModInt::new(comb.get(b));
    println!("{}", ans.val);
}

#[derive(Clone, Copy)]
pub struct ModInt {
    val: usize,
}

impl ModInt {
    pub const fn zero() -> Self {
        Self { val: 0 }
    }

    pub const fn one() -> Self {
        Self { val: 1 }
    }

    pub fn new(i: usize) -> Self {
        ModInt { val: i % MOD }
    }

    pub fn inv(&self) -> Self {
        let mut a = self.val as i64;
        let mut b = MOD as i64;
        let modulo = MOD as i64;
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
        ModInt { val: u as usize }
    }

    pub fn pow(&self, mut n: usize) -> Self {
        let mut x = self.val;
        let mut ret = if x == 0 { 0 } else { 1 };
        while n > 0 {
            if n & 1 == 1 {
                ret = ret * x % MOD;
            }
            x = x * x % MOD;
            n >>= 1;
        }
        ModInt { val: ret }
    }
}

impl std::ops::Add for ModInt {
    type Output = ModInt;
    fn add(self, other: Self) -> Self {
        ModInt::new(self.val + other.val)
    }
}

impl std::ops::AddAssign for ModInt {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}

impl std::ops::Sub for ModInt {
    type Output = ModInt;
    fn sub(self, other: Self) -> Self {
        ModInt::new(MOD + self.val - other.val)
    }
}

impl std::ops::SubAssign for ModInt {
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}

pub struct Comb {
    n: usize,
    fact_inv: Vec<usize>,
    modulo: usize,
}

impl Comb {
    pub fn init(n: usize, k: usize, modulo: usize) -> Self {
        let mut inv = vec![0; k + 1];
        let mut fact_inv = vec![0; k + 1];
        inv[1] = 1;
        fact_inv[0] = 1;
        fact_inv[1] = 1;
        for i in 2..=k {
            inv[i] = modulo - (inv[modulo % i] * (modulo / i)) % modulo;
            fact_inv[i] = (fact_inv[i - 1] * inv[i]) % modulo;
        }
        Self {
            n,
            fact_inv,
            modulo,
        }
    }

    // nCk
    pub fn get(&self, k: usize) -> usize {
        let mut ans = 1;
        for i in self.n - k + 1..=self.n {
            ans *= i;
            ans %= MOD;
        }
        ans * self.fact_inv[k] % self.modulo
    }
}
