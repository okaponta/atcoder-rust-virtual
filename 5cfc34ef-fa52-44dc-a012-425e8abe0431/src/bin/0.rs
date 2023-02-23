use proconio::input;

fn main() {
    input! {
        mut pqr:[usize;3],
    }
    pqr.sort();
    println!("{}", pqr[0] + pqr[1]);
}
