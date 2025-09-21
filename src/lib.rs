use std::collections::HashSet;

pub use rand_core;

/// DodoPoW graph storage.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Graph(Box<[(u32, u32)]>);

impl Graph {
    /// Generate a new graph.
    ///
    /// - `rng` is used to randomly generate edges of the graph. It is expected
    ///   that an RNG of good quality is provided.
    /// - `n` specifies amount of nodes and edges in the graph: `N = 2^n` edges,
    ///   `2N` nodes. `n` must be lower or equal to 32 due to internal graph
    ///   storage structure.
    pub fn new(rng: &mut impl rand_core::RngCore, n: u8) -> Self {
        assert!(n <= 32, "graph n param must be lower or equal to 32");

        let edges_number = 1_usize << n;

        let mut nodes = Vec::with_capacity(edges_number);

        for _ in 0..edges_number {
            let from_node = rng.next_u32() % edges_number as u32;
            let to_node = rng.next_u32() % edges_number as u32;

            // Can (and will) contain duplicates.
            nodes.push((from_node, to_node));
        }

        Self(nodes.into_boxed_slice())
    }

    /// Search for cycles of the graph.
    ///
    /// - `max_depth` specifies maximal length of a potential cycle.
    /// - `handler` accepts all the found cycles, and if `true` is returned then
    ///   search is stopped.
    pub fn solve(
        &self,
        max_depth: usize,
        mut handler: impl FnMut(&[u32]) -> bool
    ) -> Option<Box<[u32]>> {
        let edges_number = self.0.len();

        // Build transition matrices.
        let mut top_nodes = vec![vec![]; edges_number];
        let mut bottom_nodes = vec![vec![]; edges_number];

        for (top_node, bottom_node) in &self.0 {
            if !top_nodes[*top_node as usize].contains(bottom_node) {
                top_nodes[*top_node as usize].push(*bottom_node);
            }

            if !bottom_nodes[*bottom_node as usize].contains(top_node) {
                bottom_nodes[*bottom_node as usize].push(*top_node);
            }
        }

        // Prune top nodes with less than 2 edges.
        #[allow(clippy::needless_range_loop)]
        for top_node in 0..edges_number {
            if top_nodes[top_node].len() < 2 {
                for bottom_node in &top_nodes[top_node] {
                    bottom_nodes[*bottom_node as usize].retain(|node| {
                        node != &(top_node as u32)
                    });
                }

                top_nodes[top_node].clear();
            }
        }

        // Prune bottom nodes with less than 2 edges.
        #[allow(clippy::needless_range_loop)]
        for bottom_node in 0..edges_number {
            if bottom_nodes[bottom_node].len() < 2 {
                for top_node in &bottom_nodes[bottom_node] {
                    top_nodes[*top_node as usize].retain(|node| {
                        node != &(bottom_node as u32)
                    });
                }

                bottom_nodes[bottom_node].clear();
            }
        }

        // Run iterative DFS over the graph.
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        enum GraphSide {
            Top(u32),
            Bottom(u32)
        }

        for target_top_node in 0..edges_number as u32 {
            if top_nodes[target_top_node as usize].is_empty() {
                continue;
            }

            let mut stack: Vec<(GraphSide, Box<[u32]>)> = Vec::new();
            let mut visited = HashSet::new();

            stack.push((
                GraphSide::Top(target_top_node),
                Box::new([target_top_node])
            ));

            while let Some((node, path)) = stack.pop() {
                if visited.insert(node) && path.len() < max_depth {
                    match node {
                        GraphSide::Top(top_node) => {
                            for bottom_node in &top_nodes[top_node as usize] {
                                let mut path = path.to_vec();

                                path.push(*bottom_node);

                                stack.push((
                                    GraphSide::Bottom(*bottom_node),
                                    path.into_boxed_slice()
                                ));
                            }
                        }

                        GraphSide::Bottom(bottom_node) => {
                            for top_node in &bottom_nodes[bottom_node as usize] {
                                let mut path = path.to_vec();

                                path.push(*top_node);

                                stack.push((
                                    GraphSide::Top(*top_node),
                                    path.into_boxed_slice()
                                ));
                            }
                        }
                    }
                }

                else if node == GraphSide::Top(target_top_node)
                    && path.len() > 3
                    && handler(&path)
                {
                    return Some(path);
                }
            }
        }

        None
    }

    /// Search for any cycle of length `diff`.
    ///
    /// The `diff` value must be odd due to the graph structure, and longer
    /// than 3 to form a proper cycle, meaning `diff` is an odd number starting
    /// from 5.
    pub fn solve_for(&self, diff: usize)-> Option<Box<[u32]>> {
        if diff % 2 == 0 || diff < 5 {
            return None;
        }

        self.solve(diff, |cycle| cycle.len() == diff)
    }

    /// Verify the cycle.
    pub fn verify(&self, cycle: &[u32]) -> bool {
        let n = cycle.len();

        // IQ test (cycle must be of odd length due to graph structure).
        if n % 2 == 0 {
            return false;
        }

        let mut i = 1;

        while i < n {
            let mut found = false;

            for (top_node, bottom_node) in &self.0 {
                if (i % 2 != 0 && top_node == &cycle[i - 1] && bottom_node == &cycle[i])
                    || (i % 2 == 0 && bottom_node == &cycle[i - 1] && top_node == &cycle[i])
                {
                    found = true;

                    break;
                }
            }

            if !found {
                return false;
            }

            i += 1;
        }

        true
    }
}

#[test]
fn test() {
    use rand_core::SeedableRng;

    let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(123);

    let graph = Graph::new(&mut rng, 16);

    assert!(graph.solve_for(9).is_some());

    assert!(graph.verify(&[
        1981,
        19107,
        3084,
        24653,
        6267,
        46608,
        34728,
        11923,
        1981
    ]));
}
