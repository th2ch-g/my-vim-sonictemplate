for i in 0..n {
    let mut deq = VecDeque::new();
    deq.push_back(i);
    let mut visited = vec![false; n];
    visited[i] = true;
    while let Some(node) = deq.pop_front() {
        for next_node in &graph[node] {
            if visited[*next_node] { continue }
            visited[*next_node] = true;
            deq.push_back(*next_node);
        }
    }
}

