for i in 0..n {
    let mut stack = Vec::new();
    stack.push(i);
    let mut visited = vec![false; n];
    visited[i] = true;
    while let Some(node) = stack.pop() {
        for next_node in &graph[node] {
            if visited[*next_node] { continue }
            visited[*next_node] = true;
            stack.push(*next_node);
        }
    }
}

