use indexmap::IndexMap;

#[derive(Debug, Clone, Copy)]
pub struct Node(pub usize);

impl Node {
    pub fn empty() -> Self {
        Node(0)
    }
}

#[derive(Debug, Default)]
pub struct GraphBuilder<'a> {
    raw_links: IndexMap<&'a str, Vec<&'a str>>,
}

impl<'a> GraphBuilder<'a> {
    pub fn link(&mut self, id: &'a str, links: Vec<&'a str>) {
        let entry = self
            .raw_links
            .entry(id)
            .or_insert(Vec::with_capacity(links.len()));
        entry.extend(links);
    }

    fn reduce_graph(self, states_to_visit: &[&str]) -> IndexMap<&'a str, Vec<&'a str>> {
        let mut reduced_graph = IndexMap::new();
        for (state, neighbours) in self.raw_links {
            if states_to_visit.contains(&state) {
                let neighbours = neighbours
                    .iter()
                    .filter(|neighbour| states_to_visit.contains(neighbour))
                    .cloned()
                    .collect::<Vec<_>>();
                if !neighbours.is_empty() {
                    reduced_graph.insert(state, neighbours);
                }
            }
        }
        reduced_graph
    }

    pub fn build(self, reduce: Option<Vec<&str>>) -> Graph<'a> {
        let retain = reduce.unwrap_or_else(|| self.raw_links.keys().map(|it| *it).collect());
        let reduced = self.reduce_graph(&retain);
        Graph::build_from(reduced)
    }
}

#[derive(Debug, Default)]
pub struct Graph<'a> {
    state_to_index: IndexMap<&'a str, usize>,
    index_to_state: Vec<&'a str>,
    nodes: Vec<Node>,
}

impl<'a> Graph<'a> {
    fn build_from(links: IndexMap<&'a str, Vec<&'a str>>) -> Self {
        let mut state_to_index = IndexMap::new();
        let mut index_to_state = Vec::with_capacity(links.len());
        for &state in links.keys() {
            let index = state_to_index.len();
            state_to_index.insert(state, index);
            index_to_state.insert(index, state);
        }
        let mut m = vec![Node::empty(); links.len()];
        for (entry_key, value) in links {
            let a: Vec<_> = value.iter().map(|state| state_to_index[state]).collect();
            let mut key = 0usize;
            for state in a {
                key |= 1 << state;
            }
            m[state_to_index[entry_key]].0 = key;
        }
        Self {
            state_to_index,
            index_to_state,
            nodes: m,
        }
    }

    pub fn size(&self) -> usize {
        self.nodes.len()
    }

    pub fn get_node(&self, id: &'a str) -> Node {
        let index = self.state_to_index.get(id).expect(":D");
        self.get_node_by_index(*index)
    }

    pub fn get_node_idx(&self, id: &'a str) -> usize {
        *self.state_to_index.get(id).expect(":D")
    }

    pub fn get_node_by_index(&self, index: usize) -> Node {
        *self.nodes.get(index).expect(":D")
    }

    pub fn get_node_names(&self, nodes: &[usize]) -> Vec<String> {
        nodes
            .into_iter()
            .map(|n| self.index_to_state[*n].to_string())
            .collect()
    }
}
