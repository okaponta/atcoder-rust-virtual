use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n:usize,
        abc:[(Usize1,Usize1,usize);n-1],
        d:[usize;n],
    }
    let mut edges = vec![vec![]; 2 * n];
    for (a, b, c) in abc {
        edges[a].push((b, c));
        edges[b].push((a, c));
    }
    for i in 0..n {
        edges[i].push((n + i, d[i]));
        edges[n + i].push((i, d[i]));
    }
    let (a, b, _) = tree_diameter(&edges);
    let mut da = vec![0; 2 * n];
    let mut db = vec![0; 2 * n];
    dfs(!0, a, &edges, &mut da);
    dfs(!0, b, &edges, &mut db);
    for i in 0..n {
        if i + n == a {
            println!("{}", db[i]);
        } else if i + n == b {
            println!("{}", da[i]);
        } else {
            println!("{}", da[i].max(db[i]));
        }
    }
}

fn tree_diameter(edges: &Vec<Vec<(usize, usize)>>) -> (usize, usize, usize) {
    let l = tree_diameter_dfs(edges, 0, !0);
    let r = tree_diameter_dfs(edges, l.1, !0);
    (l.1, r.1, r.0)
}

fn tree_diameter_dfs(
    edges: &Vec<Vec<(usize, usize)>>,
    cur: usize,
    parent: usize,
) -> (usize, usize) {
    let mut ret = (0, cur);
    for &(to, cost) in &edges[cur] {
        if to == parent {
            continue;
        }
        let mut next = tree_diameter_dfs(edges, to, cur);
        next.0 += cost;
        ret = ret.max(next);
    }
    ret
}

fn dfs(prev: usize, cur: usize, edges: &Vec<Vec<(usize, usize)>>, d: &mut Vec<usize>) {
    for &(next, cost) in &edges[cur] {
        if next == prev {
            continue;
        }
        d[next] = d[cur] + cost;
        dfs(cur, next, edges, d);
    }
}
