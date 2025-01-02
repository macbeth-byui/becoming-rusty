use crate::graph::{Graph, GraphError, INF};

fn sort(g : &Graph) -> Vec<usize> {
    let mut in_degree = vec![0; g.size()];
    let mut linear_order = Vec::<usize>::new();
    let mut stack = Vec::<usize>::new();

    for vertex in 0..g.size() {
        for edge in g.edges(vertex).unwrap() {
            in_degree[edge.dest_id] += 1;
        }
    }

    for (i, v) in in_degree.iter().enumerate() {
        if *v == 0 {
            stack.push(i);
        }
    }

    while let Some(vertex) = stack.pop() {
        linear_order.push(vertex);
        for edge in g.edges(vertex).unwrap() {
            in_degree[edge.dest_id] -= 1;
            if in_degree[edge.dest_id] == 0 {
                stack.push(edge.dest_id);
            }
        }
    }

    linear_order
}

pub fn shortest_path(g : &Graph, start : usize) -> Result<(Vec<f64>, Vec<Option<usize>>), GraphError> {
    if start >= g.size() {
        return Err(GraphError::InvalidVertex);
    }
    let topo = sort(g);
    let mut distance = vec![INF; g.size()];
    let mut pred = vec![None; g.size()];
    distance[start] = 0.0;
    for vertex in topo {
        if distance[vertex] != INF {
            for edge in g.edges(vertex).unwrap() {
                if distance[vertex] + edge.weight < distance[edge.dest_id] {
                    distance[edge.dest_id] = distance[vertex] + edge.weight;
                    pred[edge.dest_id] = Some(vertex);
                }
            }
        }
    }
    Ok((distance, pred))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dag_shortest_path() {
        let mut graph = Graph::new(9);
        let _ = graph.add_edge(0, 3, 3.0);
        let _ = graph.add_edge(1, 3, 2.0);
        let _ = graph.add_edge(2, 5, 5.0);
        let _ = graph.add_edge(3, 6, 9.0);
        let _ = graph.add_edge(5, 6, 3.0);
        let _ = graph.add_edge(4, 8, 7.0);
        let _ = graph.add_edge(6, 8, 7.0);
        let _ = graph.add_edge(7, 8, 5.0);

        let result = shortest_path(&graph, 2);
        assert!(result.is_ok());
        let (dist,pred) = result.unwrap();
        assert_eq!(dist, vec![INF, INF, 0.0, INF, INF, 5.0, 8.0, INF, 15.0]);
        assert_eq!(pred, vec![None, None, None, None, None, Some(2), Some(5), None, Some(6)]);
    }
}