use std::{
    collections::{BTreeMap, BTreeSet},
    fmt::Debug,
};

#[derive(Debug)]
pub struct EdgeSet<T> {
    edges: BTreeMap<T, BTreeSet<T>>,
}

impl<T: Copy + Clone + Debug + Eq + Ord + PartialEq + PartialOrd> EdgeSet<T> {
    pub fn new() -> Self {
        Self {
            edges: BTreeMap::new(),
        }
    }

    pub fn add_edge(&mut self, from: T, to: T) {
        assert!(self.edges.entry(from).or_default().insert(to));
        assert!(self.edges.entry(to).or_default().insert(from));
    }

    pub fn is_empty(&self) -> bool {
        self.edges.is_empty()
    }

    // Hierholzer's algorithm
    pub fn trace_trail(&mut self) -> Option<Vec<T>> {
        if !self.is_empty() {
            if let Some(start) = self.find_odd_degree_vertex() {
                let mut trail = self.trace_partial_trail_from(start).unwrap();

                let mut i = 0;

                while !self.is_empty() && i < trail.len() {
                    while let Some(cycle) = self.trace_partial_trail_from(trail[i]) {
                        assert_eq!(cycle.first().unwrap(), cycle.last().unwrap());
                        trail.splice(i..=i, cycle.into_iter());
                    }

                    i += 1;
                }

                if self.is_empty() {
                    return Some(trail);
                }
            }

            None
        } else {
            Some(Vec::new())
        }
    }

    fn find_odd_degree_vertex(&self) -> Option<T> {
        for (vertex, edges) in &self.edges {
            if (edges.len() & 1) == 1 {
                return Some(*vertex);
            }
        }

        None
    }

    fn trace_partial_trail_from(&mut self, start: T) -> Option<Vec<T>> {
        if let Some(to) = self.dequeue_edge(start) {
            let mut trail = vec![start, to];

            while let Some(to) = self.dequeue_edge(*trail.last().unwrap()) {
                trail.push(to);
            }

            Some(trail)
        } else {
            None
        }
    }

    fn dequeue_edge(&mut self, from: T) -> Option<T> {
        if let Some(to_set) = self.edges.get_mut(&from) {
            let to = to_set.pop_first().unwrap();

            assert!(self.edges.get_mut(&to).unwrap().remove(&from));

            if self.edges[&from].is_empty() {
                assert!(self.edges.remove(&from).is_some());
            }

            if self.edges[&to].is_empty() {
                assert!(self.edges.remove(&to).is_some());
            }

            Some(to)
        } else {
            None
        }
    }
}
