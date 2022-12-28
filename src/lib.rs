#[cfg(test)]
mod tests {
    use digraph_rs::{
        analyzer::dijkstra::{DijkstraPath, MinPathProcessor},
        digraph, extend_edges, extend_nodes, DiGraph, EmptyPayload,
    };

    #[test]
    fn it_works() {
        let mut graph = digraph!((usize,_,usize) => [1,2,3,4,5,6,7,8] => {
          1 => [(2,3),(3,1),(4,2)];
          [2,3,4] => (5,2);
          5 => (6,1);
          6 => [(7,2),(8,3)];
        });

        let v_res = graph.visualize().str_to_dot_file("dots/graph.svg");
        assert!(v_res.is_ok());

        assert!(graph.analyze().edge(&1, &2).is_some());
        assert!(graph.analyze().edge(&1, &6).is_none());

        let mut path_finder = DijkstraPath::new(&graph);
        let paths = path_finder.on_edge(1);
        let trail = paths.trail(&8).unwrap();
        assert_eq!(trail, vec![1, 3, 5, 6, 8]);
        let r = graph
            .visualize()
            .to_dot_file("dots/graph_path_1_8.svg", MinPathProcessor::new(trail));
        assert!(r.is_ok());
    }
}
