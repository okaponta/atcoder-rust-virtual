use proconio::input;

fn main() {
    input! {
        n:usize,
        k:usize,
        a:[usize;n],
        b:[usize;n],
    }
    let mut c = vec![a[0], b[0]];
    for i in 1..n {
        let mut next = vec![];
        for e in c {
            if abs(e, a[i]) <= k {
                next.push(a[i]);
            }
            if abs(e, b[i]) <= k {
                next.push(b[i]);
            }
        }
        next.sort();
        next.dedup();
        c = next;
    }
    println!("{}", if c.len() == 0 { "No" } else { "Yes" });
}

fn abs(a: usize, b: usize) -> usize {
    if a < b {
        b - a
    } else {
        a - b
    }
}
