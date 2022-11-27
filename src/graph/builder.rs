#[macro_export]
macro_rules! extend_nodes {
    ($graph:ident => [$(($id:expr, $p:expr)),+ $(,)?]) => {{
         $(
            $graph.add_node($id,$p);
         )+
        $graph
    }};

    ($graph:ident => [$($id:expr),+ $(,)?]) => {{
         $(
            $graph.add_bare_node($id);
         )+
      $graph
    }};
}

#[macro_export]
macro_rules! extend_edges {
    ($graph:expr => {}) => {{
       $graph
    }};

    ($graph:expr => {[$(($from:expr,$p:expr)),+ $(,)?] => $to:expr; $($rest:tt)*}) => {{
        $(
            $graph.add_edge($from,$to,$p);
         )+
       extend_edges!($graph => {$($rest)*})
    }};

    ($graph:expr => {[$(($from:expr,$p:expr)),+ $(,)?] => $to:expr $(;)?}) => {{
         $(
            $graph.add_edge($from,$to,$p);
         )+
       $graph
    }};
    ($graph:expr => {[$($from:expr),+ $(,)?] => ($to:expr,$p:expr) $(;)?}) => {{

         $(
            $graph.add_edge($from,$to,$p);
          )+

       $graph
    }};
    ($graph:expr => {[$($from:expr),+ $(,)?] => ($to:expr,$p:expr) ; $($rest:tt)*}) => {{

         $(
            $graph.add_edge($from,$to,$p);
          )+

        extend_edges!($graph => {$($rest)*})
    }};
    ($graph:expr => {[$($from:expr),+ $(,)?] => $to:expr $(;)?}) => {{
         $(
           $graph.add_bare_edge($from,$to);
         )+
        $graph
    }};
    ($graph:expr => {[$($from:expr),+ $(,)?] => $to:expr ; $($rest:tt)*}) => {{
         $(
           $graph.add_bare_edge($from,$to);
         )+
         extend_edges!($graph => {$($rest)*})
    }};

    ($graph:expr => {$from:expr => [$(($to:expr,$p:expr)),+ $(,)?] $(;)? }) => {{
         $(
           $graph.add_edge($from,$to,$p);
         )+
         $graph
    }};
    ($graph:expr => {$from:expr => [$(($to:expr,$p:expr)),+ $(,)?] ; $($rest:tt)*}) => {{
         $(
           $graph.add_edge($from,$to,$p);
         )+
          extend_edges!($graph => {$($rest)*})
    }};
    ($graph:expr => {$from:expr => [$($to:expr),+ $(,)?] $(;)?}) => {{
         $(
           $graph.add_bare_edge($from,$to);
         )+
         $graph
    }};
    ($graph:expr => {$from:expr => [$($to:expr),+ $(,)?] ; $($rest:tt)*}) => {{
         $(
           $graph.add_bare_edge($from,$to);
         )+
         extend_edges!($graph => {$($rest)*})
    }};

    ($graph:expr => {$from:expr => ($to:expr,$p:expr) $(;)?}) => {{
         $graph.add_edge($from,$to,$p);
         $graph
    }};
    ($graph:expr => {$from:expr => ($to:expr,$p:expr); $($rest:tt)*}) => {{
         $graph.add_edge($from,$to,$p);
         extend_edges!($graph => {$($rest)*})
    }};
    ($graph:expr => {$from:expr => $to:expr $(;)?}) => {{
         $graph.add_bare_edge($from,$to);
         $graph
    }};
    ($graph:expr => {$from:expr => $to:expr; $($rest:tt)*}) => {{
         $graph.add_bare_edge($from,$to);
         extend_edges!($graph => {$($rest)*})
    }};
}

#[macro_export]
macro_rules! digraph {
    () => {
        DiGraph::empty()
    };

    (_ , $n:ty,_ ) => {
        DiGraph::<usize,$n,EmptyPayload>::new()
    };
   ( $id:ty,_,_) => {
      DiGraph::<$id,EmptyPayload,EmptyPayload>::new()
    };
    ( _,_,$e:ty) => {
        DiGraph::<usize,EmptyPayload,$e>::new()
    };
    ( _, $n:ty, $e:ty) => {
        DiGraph::<usize,$n,$e>::new()
    };

    ( $id:ty,_, $e:ty) => {
        DiGraph::<$id,EmptyPayload,$e>::new()
    };

    ( $id:ty,$n:ty, $e:ty) => {
        DiGraph::<$id,$n,$e>::new()
    };

    (($($tps:tt)+) => $nodes:tt ) => {{
       let mut g = digraph!($($tps)+);
       extend_nodes!(g => $nodes)
     }};


   (($($tps:tt)+)=> $nodes:tt => $edges:tt) => {{
       let mut g = digraph!($($tps)+);
       let mut g = extend_nodes!(g => $nodes);
       extend_edges!(g => $edges)
     }};


    ( => $nodes:tt ) => {{
       let mut g = digraph!();
       extend_nodes!(g => $nodes)
     }};

    (=> $nodes:tt => $edges:tt) => {{
       let mut g = digraph!();
       let mut g = extend_nodes!(g => $nodes);
       extend_edges!(g => $edges)
     }};

}

#[cfg(test)]
mod tests {
    use crate::digraph;
    use crate::graph::{DiGraph, EmptyPayload};
    use std::collections::HashMap;

    #[derive(Default)]
    struct S(i32);

    impl S {
        pub fn new(i: i32) -> Self {
            Self { 0: i }
        }
    }

    #[test]
    fn simple_macro_v_d_test() {
        let d = digraph!();
        let d = digraph!(_, S, usize);
        let d = digraph!(_,S, S);
        let d = digraph!(_,S,_);
        let d = digraph!(_,_, S);
        let d = digraph!(_,bool, S);
        let d = digraph!(=> [1,2,3]);
        assert_eq!(
            d.nodes,
            HashMap::from([(1, EmptyPayload), (2, EmptyPayload), (3, EmptyPayload)])
        );

        let d = digraph!((_,i32,_) => [(1,1),(2,2),(3,0)]);
        assert_eq!(d.nodes, HashMap::from([(1, 1), (2, 2), (3, 0)]));

        let d = digraph!(
            (_,_,i32) => [1,2,3,4] => {
                1 => 2;
                2 => (3,1);
                [4,3] => 1;
            }
        );
        assert_eq!(
            d.nodes,
            HashMap::from([
                (1, EmptyPayload),
                (2, EmptyPayload),
                (3, EmptyPayload),
                (4, EmptyPayload)
            ])
        );
        assert_eq!(
            d.edges,
            HashMap::from([
                (1, HashMap::from([(2, 0)])),
                (2, HashMap::from([(3, 1)])),
                (3, HashMap::from([(1, 0)])),
                (4, HashMap::from([(1, 0)])),
            ])
        );
    }
    #[test]
    fn simple_diff_key_macro_test() {
        let d = digraph!(String,_,_);
        let d = digraph!((&str,_,_) => ["1","2","3"]);
        assert_eq!(
            d.nodes,
            HashMap::from([("1", EmptyPayload), ("2", EmptyPayload), ("3", EmptyPayload)])
        );

        let mut g = digraph!((&str,_,i32) => ["a","b","c"] => {
            "a" => "b";
            ["a","b"] => "c";
        });

        let f = g.edges;

        assert_eq!(f.get("a"), Some(&HashMap::from([("b", 0), ("c", 0)])));
        assert_eq!(f.get("b"), Some(&HashMap::from([("c", 0)])));


    }

    #[test]
    fn builder_nodes_test() {
        let mut g = digraph!(_,usize,_);
        let g = extend_nodes!(g => [(1,1),(2,1)]);
        assert_eq!(g.nodes, HashMap::from([(1, 1), (2, 1)]));

        let mut g = digraph!(_,usize,_);
        let def_p = 100;
        let g = extend_nodes!(g => [(1,def_p),(2,def_p)]);
        assert_eq!(g.nodes, HashMap::from([(1, 100), (2, 100)]));
    }

    #[test]
    fn builder_edges_arr_to_test() {
        let mut g = digraph!(_,_, i32);
        let mut g = extend_nodes!(g => [1,2,3,4,5,6,7]);
        let mut g = extend_edges!(g => {
            [(2,1),(3,1)] => 4
        });
        assert_eq!(
            g.edges,
            HashMap::from([(2, HashMap::from([(4, 1)])), (3, HashMap::from([(4, 1)])),])
        );

        let mut g = digraph!(_,_, i32);
        let mut g = extend_nodes!(g => [1,2,3,4,5,6,7]);
        let g = extend_edges!(g => {
            [(4,1),(1,1)] => 5;
            [(5,1),(6,1)] => 7;
        });
        assert_eq!(
            g.edges,
            HashMap::from([
                (4, HashMap::from([(5, 1)])),
                (1, HashMap::from([(5, 1)])),
                (5, HashMap::from([(7, 1)])),
                (6, HashMap::from([(7, 1)])),
            ])
        );

        let mut g = digraph!(_,_, i32);
        let mut g = extend_nodes!(g => [1,2,3,4,5,6,7]);
        let mut g = extend_edges!(g => {
            [1,2] => (3,10)
        });
        assert_eq!(
            g.edges,
            HashMap::from([(2, HashMap::from([(3, 10)])), (1, HashMap::from([(3, 10)])),])
        );

        let mut g = extend_edges!(g => {
            [1,2] => (3,10);
            [3,4] => (5,1);
            [(5,1),(6,10)] => 7
        });
        assert_eq!(
            g.edges,
            HashMap::from([
                (2, HashMap::from([(3, 10)])),
                (1, HashMap::from([(3, 10)])),
                (3, HashMap::from([(5, 1)])),
                (4, HashMap::from([(5, 1)])),
                (5, HashMap::from([(7, 1)])),
                (6, HashMap::from([(7, 10)])),
            ])
        );

        let mut g = digraph!();
        let mut g = extend_nodes!(g => [1,2,3,4,5,6,7]);
        let mut g = extend_edges!(g => {
            [1,2] => 3
        });
        assert_eq!(
            g.edges,
            HashMap::from([
                (2, HashMap::from([(3, EmptyPayload)])),
                (1, HashMap::from([(3, EmptyPayload)])),
            ])
        );
        let mut g = extend_edges!(g => {
            [1,2] => 3;
            [3,4] => 5;
        });
        assert_eq!(
            g.edges,
            HashMap::from([
                (2, HashMap::from([(3, EmptyPayload)])),
                (1, HashMap::from([(3, EmptyPayload)])),
                (3, HashMap::from([(5, EmptyPayload)])),
                (4, HashMap::from([(5, EmptyPayload)])),
            ])
        );

        let mut g = extend_edges!(g => {
            5 => [6,7];
            6 => [7,1];
        });
        assert_eq!(
            g.edges,
            HashMap::from([
                (2, HashMap::from([(3, EmptyPayload)])),
                (1, HashMap::from([(3, EmptyPayload)])),
                (3, HashMap::from([(5, EmptyPayload)])),
                (4, HashMap::from([(5, EmptyPayload)])),
                (5, HashMap::from([(7, EmptyPayload), (6, EmptyPayload)])),
                (6, HashMap::from([(7, EmptyPayload), (1, EmptyPayload)])),
            ])
        );

        let mut g = digraph!(_,_, i32);
        let mut g = extend_nodes!(g => [1,2,3,4,5,6,7]);
        let g = extend_edges!(g => {
            [(1,1),(2,1)] => 3;
            [(3,1)] => 4;
            4 => [(5,1),(6,1)];
            7 => [1,2]

        });
        assert_eq!(
            g.edges,
            HashMap::from([
                (1, HashMap::from([(3, 1)])),
                (2, HashMap::from([(3, 1)])),
                (3, HashMap::from([(4, 1)])),
                (4, HashMap::from([(5, 1), (6, 1)])),
                (7, HashMap::from([(1, 0), (2, 0)])),
            ])
        );
    }

    #[test]
    fn builder_edges_simple_to_test() {
        let mut g = digraph!(_,_, i32);
        let mut g = extend_nodes!(g => [1,2,3,4,5,6,7]);
        let mut g = extend_edges!(g => {
            1 => [1,2];
            2 => 3;
            3 => 4;
            4 => (5,7);
            5 => (6,1);
            7 => 1
        });

        let f = g.edges;

        assert_eq!(f.get(&1), Some(&HashMap::from([(1, 0), (2, 0)])));
        assert_eq!(f.get(&2), Some(&HashMap::from([(3, 0)])));
        assert_eq!(f.get(&3), Some(&HashMap::from([(4, 0)])));
        assert_eq!(f.get(&4), Some(&HashMap::from([(5, 7)])));
        assert_eq!(f.get(&7), Some(&HashMap::from([(1, 0)])));
    }
}
