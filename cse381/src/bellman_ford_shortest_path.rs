use crate::graph::{Graph, INF, GraphError};

pub fn shortest_path(g : &Graph, start : usize) -> Result<(Vec<f64>, Vec<Option<usize>>), GraphError> {
    if start >= g.size() {
        return Err(GraphError::InvalidVertex);
    }
    let mut distance = vec![INF; g.size()];
    let mut pred = vec![None; g.size()];
    distance[start] = 0.0;

    for i in 0..g.size() {
        let mut changed = false;
        for vertex in 0..g.size() {
            for edge in g.edges(vertex).unwrap() {
                if distance[vertex] + edge.weight < distance[edge.dest_id] {
                    if i == g.size()-1 {
                        return Err(GraphError::NegativeCycle)
                    }
                    changed = true;
                    distance[edge.dest_id] = distance[vertex] + edge.weight;
                    pred[edge.dest_id] = Some(vertex);
                }
            }
        }
        if !changed {
            break;
        }
    }
    Ok((distance, pred))

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1_no_negative_cycle() {
        let mut graph = Graph::new(5);
        let _ = graph.add_edge(0, 1, 6.0);
        let _ = graph.add_edge(0, 3, 7.0);
        let _ = graph.add_edge(1, 2, 5.0);
        let _ = graph.add_edge(1, 3, 8.0);
        let _ = graph.add_edge(1, 4, -4.0);
        let _ = graph.add_edge(2, 1, -2.0);
        let _ = graph.add_edge(3, 2, -3.0);
        let _ = graph.add_edge(3, 4, 9.0);
        let _ = graph.add_edge(4, 0, 2.0);
        let _ = graph.add_edge(4, 2, 7.0);

        let result = shortest_path(&graph, 0);
        assert!(result.is_ok());
        let (dist,pred) = result.unwrap();
        assert_eq!(dist, vec![0.0, 2.0, 4.0, 7.0, -2.0]);
        assert_eq!(pred, vec![None, Some(2), Some(3), Some(0), Some(1)]);
    }

    #[test]
    fn test2_negative_cycle() {
        let mut graph = Graph::new(5);
        let _ = graph.add_edge(0, 1, 6.0);
        let _ = graph.add_edge(0, 3, 7.0);
        let _ = graph.add_edge(1, 2, 5.0);
        let _ = graph.add_edge(1, 3, -1.0);
        let _ = graph.add_edge(1, 4, -4.0);
        let _ = graph.add_edge(2, 1, -2.0);
        let _ = graph.add_edge(3, 2, -3.0);
        let _ = graph.add_edge(3, 4, 9.0);
        let _ = graph.add_edge(4, 0, 2.0);
        let _ = graph.add_edge(4, 2, 7.0);

        let result = shortest_path(&graph, 0);
        assert_eq!(result, Err(GraphError::NegativeCycle));
    }
}