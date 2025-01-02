use crate::graph::{Graph, INF, GraphError};
use crate::graph_heap::GraphHeap;

pub fn shortest_path(g : &Graph, start : usize) -> Result<(Vec<f64>, Vec<Option<usize>>), GraphError> {
    if start >= g.size() {
        return Err(GraphError::InvalidVertex);
    }
    let mut distance = vec![INF; g.size()];
    let mut pred = vec![None; g.size()];
    distance[start] = 0.0;

    let mut queue = GraphHeap::default();
    for (vertex, distance) in distance.iter().enumerate() {
        queue.enqueue(vertex, *distance);
    }

    while let Some(vertex) = queue.dequeue() {
        for edge in g.edges(vertex).unwrap() {
            if distance[vertex] + edge.weight < distance[edge.dest_id] {
                distance[edge.dest_id] = distance[vertex] + edge.weight;
                pred[edge.dest_id] = Some(vertex);
                let _ = queue.decrease_distance(edge.dest_id, distance[edge.dest_id]);
            }
        }
    }
    Ok((distance, pred))

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dijkstra_shortest_path() {
        let mut graph = Graph::new(5);
        let _ = graph.add_edge(0, 1, 6.0);
        let _ = graph.add_edge(0, 3, 4.0);
        let _ = graph.add_edge(1, 2, 3.0);
        let _ = graph.add_edge(1, 3, 2.0);
        let _ = graph.add_edge(2, 4, 4.0);
        let _ = graph.add_edge(3, 1, 1.0);
        let _ = graph.add_edge(3, 2, 9.0);
        let _ = graph.add_edge(3, 4, 3.0);
        let _ = graph.add_edge(4, 2, 5.0);
        let _ = graph.add_edge(4, 0, 7.0);

        let result = shortest_path(&graph, 1);
        assert!(result.is_ok());
        let (dist,pred) = result.unwrap();
        assert_eq!(dist, vec![12.0, 0.0, 3.0, 2.0, 5.0]);
        assert_eq!(pred, vec![Some(4), None, Some(1), Some(1), Some(3)]);
    }
}