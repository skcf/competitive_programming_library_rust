fn dijkstra(g: &Vec<Vec<(usize, i64)>>, n: usize, src: usize) -> Vec<i64> {
    let mut dp = vec![i64::max_value() / 2; n];
    dp[src] = 0;
    let mut h = std::collections::BinaryHeap::new();
    h.push((0, src));
    while let Some((d, v)) = h.pop() {
        let d = -d;
        if d > dp[v] {
            continue;
        }
        for &(u, w) in g[v].iter() {
            let d = d + w;
            if d < dp[u] {
                dp[u] = d;
                h.push((-d, u));
            }
        }
    }
    dp
}
