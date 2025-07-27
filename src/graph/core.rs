use rustc_hash::FxHasher;
use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::{BuildHasherDefault, Hash};
use std::marker::PhantomData;

use super::types::{GraphType, Node};

#[derive(Debug, Clone)]
pub struct Graph<I: Debug, EW: Debug, NW: Debug, T: GraphType> {
    pub coord_map: HashMap<I, usize, BuildHasherDefault<FxHasher>>,
    pub reverse_map: Vec<I>,
    pub nodes: Vec<Node<NW>>,
    pub adj: Vec<Vec<(usize, Option<EW>)>>,
    _phantom: PhantomData<T>,
}

impl<I: Clone + Eq + Hash + Debug, EW: Debug, NW: Debug, T: GraphType> Graph<I, EW, NW, T> {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Graph {
            coord_map: HashMap::<I, usize, BuildHasherDefault<FxHasher>>::default(),
            reverse_map: Vec::new(),
            nodes: Vec::new(),
            adj: Vec::new(),
            _phantom: PhantomData,
        }
    }

    fn get_or_create_id(&mut self, key: I) -> usize {
        if let Some(&id) = self.coord_map.get(&key) {
            id
        } else {
            let id = self.reverse_map.len();
            self.coord_map.insert(key.clone(), id);
            self.reverse_map.push(key);
            self.nodes.push(Node { weight: None });
            self.adj.push(Vec::new());
            id
        }
    }

    pub fn add_edge(&mut self, from: I, to: I, weight: Option<EW>) {
        let from_id = self.get_or_create_id(from);
        let to_id = self.get_or_create_id(to);
        self.adj[from_id].push((to_id, weight));
    }

    pub fn add_weight_to_node(&mut self, id: I, weight: NW) {
        let node_id = self.get_or_create_id(id);
        self.nodes[node_id].weight = Some(weight);
    }
}