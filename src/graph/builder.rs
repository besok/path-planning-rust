#[macro_export]
macro_rules! extend_nodes {
    ($graph:ident => [$(($id:expr, $p:expr)),+ $(,)?]) => {{
         $(
            $graph.add_node($id,$p);
         )+
        $graph
    }};
    ($graph:ident => p[$($p:expr),+ $(,)?]) => {{
         $(
           $graph.gen_node($p);
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

    ($graph:expr => {$from:expr => $to:expr,$p:expr $(;)?}) => {{
         $graph.add_edge($from,$to,$p);
         $graph
    }};
    ($graph:expr => {$from:expr => $to:expr,$p:expr; $($rest:tt)*}) => {{
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
    ( _, $e:ty) => {
        DiGraph::<EmptyPayload,$e>::new()
    };
    ( $n:ty, $e:ty) => {
        DiGraph::<$n,$e>::new()
    };

    ( $n:ty) => {
        DiGraph::<$n,EmptyPayload>::new()
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
    use crate::graph::{DiGraph, EmptyPayload, NodeId};

    #[derive(Default)]
    struct S(i32);

    impl S {
        pub fn new(i: i32) -> Self {
            Self { 0: i }
        }
    }

    #[test]
    fn simple_macro_v_d_test() {
        let d = digraph!( S,usize);
        let d = digraph!( S,S);
        let d = digraph!(S);
        let d = digraph!(_,S);
        let d = digraph!( bool,S);
        let d = digraph!(=> [1,2,3]);
        let d = digraph!((i32) => [(1,1),(2,2),(3,0)]);
        let d =
            digraph!(
                (_,i32) => [1,2,3,4] => {
                    1 => 2;
                    2 => 3,1;
                    [4,3] => 1;
                }
            );
        println!("{:?}", d)
    }

    #[test]
    fn simple_macro_e_d_test() {
        let mut d = digraph!( => [1,2,3] => { 1 => 2 });
        println!("{:?}", d)
    }

    #[test]
    fn builder_nodes_test() {
        let mut g = digraph!();

        let ids = extend_nodes!(g => [1,2,3]);


        let mut g = digraph!(usize);
        let g = extend_nodes!(g => [(1,1),(2,1)]);
        println!("{:?}", g.nodes);

        let mut g = digraph!(usize);
        let g = extend_nodes!(g => p[1,2]);
        println!("{:?}", g.nodes);

        let mut g = digraph!(usize);
        let def_p = 100;
        let g = extend_nodes!(g => [(1,def_p),(2,def_p)]);
        println!("{:?}", g.nodes);
    }


    #[test]
    fn builder_edges_arr_to_test() {
        let mut g = digraph!(_,i32);
        let mut g = extend_nodes!(g => [1,2,3,4,5,6,7]);
        let mut g = extend_edges!(g => {
            [(2,1),(3,1)] => 4
        });
        // println!("{:?}", g.edges);
        let g = extend_edges!(g => {
            [(4,1),(1,1)] => 5;
            [(5,1),(6,1)] => 7;
        });

        // println!("{:?}", g.edges);

        let mut g = digraph!(_,i32);
        let mut g = extend_nodes!(g => [1,2,3,4,5,6,7]);
        let mut g = extend_edges!(g => {
            [1,2] => (3,10)
        });
        // println!("{:?}", g.edges);
        let mut g = extend_edges!(g => {
            [1,2] => (3,10);
            [3,4] => (5,1);
            [(5,1),(6,10)] => 7
        });
        // println!("{:?}", g.edges);

        let mut g = digraph!();
        let mut g = extend_nodes!(g => [1,2,3,4,5,6,7]);
        let mut g = extend_edges!(g => {
            [1,2] => 3
        });
        println!("{:?}", g.edges);
        let mut g = extend_edges!(g => {
            [1,2] => 3;
            [3,4] => 5;
        });
        println!("{:?}", g.edges);
        let mut g = extend_edges!(g => {
            [1,2] => 3;
            [3,4] => 5;
        });
        println!("{:?}", g.edges);
        let mut g = extend_edges!(g => {
            5 => [6,7];
            6 => [7,1];
        });
        println!("{:?}", g.edges);

        let mut g = digraph!(_,i32);
        let mut g = extend_nodes!(g => [1,2,3,4,5,6,7]);
        let g = extend_edges!(g => {
            [(1,1),(2,1)] => 3;
            [(3,1)] => 4;
            4 => [(5,1),(6,1)];
            7 => [1,2]

        });
        println!("{:?}", g.edges);
    }


    #[test]
    fn builder_edges_simple_to_test() {
        let mut g = digraph!(_,i32);
        let mut g = extend_nodes!(g => [1,2,3,4,5,6,7]);
        let mut g = extend_edges!(g => {
            1 => [1,2];
            2 => 3;
            3 => 4;
            4 => 5,7;
            5 => 6,1;
            7 => 1
        });
        println!("e = {:?}", g.edges);
    }

}