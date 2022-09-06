use crate::graphs::Cardinal::{EST, NORTH, SOUTH, WEST};
use crate::graphs::Color::{BLACK, GREY, WHITE};
use crate::graphs::GraphType::GraphUndirected;
use crate::MinHeap;
use queues::{IsQueue, Queue};

pub struct Edge {
    src: usize,
    dst: usize,
    weight: usize,
    pub direction: Cardinal,
}

pub enum Cardinal {
    NORTH,
    SOUTH,
    WEST,
    EST,
}

impl Cardinal {
    pub fn value(&self) -> char {
        match *self {
            NORTH => 'N',
            SOUTH => 'S',
            WEST => 'W',
            EST => 'E',
        }
    }
}

#[derive(PartialEq, Eq)]
pub enum GraphType {
    GraphUndirected,
    GraphDirected,
}

///This represents a non-negative weighted graph
pub struct Graph {
    ///Number of nodes
    n_nodes: usize,
    ///Number of edges
    n_edges: usize,
    ///Graph type
    g_type: GraphType,
    ///Adjacency list of each node:
    ///All the edges starting from a particular node
    edges: Vec<Vec<Edge>>,
    ///Numbers of edges that ends in the i node
    in_deg: Vec<usize>,
    ///Numbers of edges that starts from the i node
    out_deg: Vec<usize>,
}

impl Edge {
    pub fn new(src: usize, dst: usize, weight: usize, direction: Cardinal) -> Edge {
        Edge {
            src,
            dst,
            weight,
            direction,
        }
    }
}

impl Graph {
    /// Return a new graph.
    ///
    /// # Arguments
    /// * `n_nodes` - number of nodes of the graph
    /// * `g_type` - type of graph : directed or non-directed
    ///
    /// # Examples
    ///
    /// let graph = Graph::new(10, GraphUndirected);
    pub fn new(n_nodes: usize, g_type: GraphType) -> Graph {
        let mut graph = Graph {
            n_nodes,
            n_edges: 0,
            g_type,
            edges: Vec::new(),
            in_deg: Vec::new(),
            out_deg: Vec::new(),
        };
        for _ in 0..n_nodes {
            graph.edges.push(Vec::new());
            graph.in_deg.push(0);
            graph.out_deg.push(0);
        }
        return graph;
    }

    pub fn n_nodes(&self) -> usize {
        self.n_nodes
    }

    fn insert_edge(&mut self, src: usize, dst: usize, weight: usize, direction: Cardinal) {
        self.edges[src].push(Edge::new(src, dst, weight, direction));
        self.in_deg[dst] += 1;
        self.out_deg[src] += 1;
    }

    /// Add new edge.
    ///
    /// # Arguments
    ///
    /// * `src` - source of the edge
    /// * `dst` - destination of the edge
    /// * `weight` - weight of the edge
    /// * `vertical` - (when the graph is a grid) if is a vertical edge or not
    ///
    pub fn add_edge(&mut self, src: usize, dst: usize, weight: usize, vertical: bool) {
        let mut direction = if vertical { SOUTH } else { EST };
        self.insert_edge(src, dst, weight, direction);
        if self.g_type == GraphUndirected {
            direction = if vertical { NORTH } else { WEST };
            self.insert_edge(dst, src, weight, direction);
        }
        self.n_edges += 1;
    }
}

#[derive(PartialEq, Eq)]
enum Color {
    BLACK,
    WHITE,
    GREY,
}

/// Returns the shortest path tree with the Dijkstra Algorithm (O(|E|+|V|log(|V|) with |V| = number of nodes, |E| = number of edges) of the given graph
/// from the give source.
///
/// The first value of the output tuple is the list of the predecessors (not necessary).
///
/// The second value is the list of the distances from the source.
///
/// The third value is the list of the previous edge.
///
/// # Arguments
/// * `graph` - graph where to execute the algorithm
/// * `source` - source node of the shortest path tree

pub fn dijkstra(
    graph: &Graph,
    source: usize,
) -> (Vec<Option<usize>>, Vec<usize>, Vec<Option<&Edge>>) {
    let mut distances: Vec<usize> = Vec::new();
    let mut predecessors: Vec<Option<usize>> = Vec::new();
    let mut heap = MinHeap::new();
    let mut added = Vec::new();
    let mut prev_edge: Vec<Option<&Edge>> = Vec::new();

    for i in 0..graph.n_nodes {
        distances.push(if i == source {
            0
        } else {
            usize::MAX - (1000 * 1000)
        });
        predecessors.push(None);
        added.push(false);
        prev_edge.push(None);
        heap.insert(i, distances[i])
    }

    while !heap.is_empty() {
        let u = heap.delete_min();
        added[u] = true;
        for edge in graph.edges[u].as_slice() {
            let weight = edge.weight;
            let dst = edge.dst;
            if !added[dst] && (distances[u] + weight < distances[dst]) {
                distances[dst] = distances[u] + weight;
                heap.change_prio(dst, distances[dst]);
                predecessors[dst] = Some(u);
                prev_edge[dst] = Some(edge);
            }
        }
    }

    return (predecessors, distances, prev_edge);
}

/// Returns the shortest path tree with the BFS Algorithm
/// ( O(|E|+|V|) with |V| = number of nodes, |E| = number of edges) of the given graph
/// from the give source.
///
/// The first value of the output tuple is the list of the predecessors (not necessary).
///
/// The second value is the list of the distances from the source.
///
/// The third value is the list of the previous edge.
///
/// # Arguments
/// * `graph` - graph where to execute the algorithm
/// * `source` - source node of the shortest path tree
pub fn bfs(graph: &Graph, source: usize) -> (Vec<Option<usize>>, Vec<i32>, Vec<Option<&Edge>>) {
    let mut colors: Vec<Color> = Vec::new();
    let mut distances: Vec<i32> = Vec::new();
    let mut predecessors: Vec<Option<usize>> = Vec::new();
    let mut prev_edge: Vec<Option<&Edge>> = Vec::new();
    let mut queue: Queue<usize> = Queue::new();

    for _ in 0..(graph.n_nodes) {
        colors.push(WHITE);
        distances.push(-1);
        predecessors.push(None);
        prev_edge.push(None);
    }

    colors[source] = GREY;
    distances[source] = 0;

    let _ = queue.add(source);

    while queue.size() != 0 {
        let src = queue.remove().unwrap();
        for edge in graph.edges[src].as_slice() {
            if colors[edge.dst] == WHITE {
                colors[edge.dst] = GREY;
                distances[edge.dst] = distances[src] + 1;
                predecessors[edge.dst] = Some(src);
                prev_edge[edge.dst] = Some(edge);
                let _ = queue.add(edge.dst);
            }
        }
        colors[src] = BLACK;
    }

    return (predecessors, distances, prev_edge);
}

pub fn print_bfs(src: usize, dst: usize, pred: Vec<Option<usize>>, dist: Vec<i32>) {
    println!("  src | dest | distance | path");
    println!("------+------+----------+-------------------------");
    print!("  {}  |  {}  |    {}    | ", src, dst, dist[dst]);
    print_path(src, dst, pred);
    println!();
}

pub fn print_path(src: usize, dst: usize, pred: Vec<Option<usize>>) {
    if src == dst {
        print!("{}", src)
    } else {
        match pred[dst] {
            Some(prev_dst) => {
                print_path(src, prev_dst, pred);
                print!("->{}", dst);
            }
            None => println!("-1"),
        }
    }
}
