use std::fs;
use std::io::{BufRead, BufReader, Error};
use crate::graphs::Cardinal::{EST, NORTH, SOUTH, WEST};
use crate::graphs::Color::{BLACK, GREY, WHITE};
use crate::graphs::GraphType::{GraphDirected, GraphUndirected};
use crate::MinHeap;
use queues::{IsQueue, Queue};

pub struct Edge {
    src: usize,
    dst: usize,
    weight: f32,
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
    pub fn new(src: usize, dst: usize, weight: f32, direction: Cardinal) -> Edge {
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
    ///
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

    fn insert_edge(&mut self, src: usize, dst: usize, weight: f32, direction: Cardinal) {
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
    pub fn add_edge(&mut self, src: usize, dst: usize, weight: f32, vertical: bool) {
        let mut direction = if vertical { SOUTH } else { EST };
        self.insert_edge(src, dst, weight, direction);
        if self.g_type == GraphUndirected {
            direction = if vertical { NORTH } else { WEST };
            self.insert_edge(dst, src, weight, direction);
        }
        self.n_edges += 1;
    }

    pub fn print(&self){
        println!("{}", if self.g_type == GraphDirected {String::from("DIRECTED")} else {String::from("UNDIRECTED")});
        for i in 0..self.edges.len() {
            print!("[ {}]", i);
            for edge in self.edges[i].as_slice() {
                print!(" -> ({}, {}, {})", edge.src, edge.dst, edge.weight);
            }
            println!();
        }
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
///
pub fn dijkstra(graph: &Graph, source: usize, ) -> (Vec<Option<usize>>, Vec<f32>, Vec<Option<&Edge>>) {
    let mut distances: Vec<f32> = Vec::new();
    let mut predecessors: Vec<Option<usize>> = Vec::new();
    let mut heap = MinHeap::new();
    let mut added = Vec::new();
    let mut prev_edge: Vec<Option<&Edge>> = Vec::new();

    for i in 0..graph.n_nodes {
        distances.push(if i == source {
            0.0
        } else {
            f32::MAX - (1000.0 * 1000.0)
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
///
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

pub fn dfs(graph : &Graph) -> (Vec<Option<usize>>, Vec<usize>, Vec<usize>){
    let mut discover: Vec<usize> = Vec::new();
    let mut finish: Vec<usize> = Vec::new();
    let mut color: Vec<Color> = Vec::new();
    let mut prev: Vec<Option<usize>> = Vec::new();
    let mut time = 0;

    for _ in 0..graph.n_nodes() {
        color.push(WHITE);
        prev.push(None);
        discover.push(0);
        finish.push(0);
    }

    for i in 0..graph.n_nodes() {
        if color[i] == WHITE {
            dfs_visit(graph, i, &mut discover, &mut finish, &mut color, &mut prev, &mut time);
        }
    }

    (prev, discover, finish)
}

fn dfs_visit(graph : &Graph, src : usize, discover: &mut Vec<usize>, finish: &mut Vec<usize>, color: &mut Vec<Color>, prev: &mut Vec<Option<usize>>, time: &mut usize){
    *time += 1;
    discover[src] = *time;
    color[src] = GREY;

    for edge in graph.edges[src].as_slice() {
        let dst = edge.dst;
        if color[dst] == WHITE {
            prev[dst] = Some(src);
            dfs_visit(graph, dst, discover, finish, color, prev, time);
        }
    }

    color[src] = BLACK;
    *time += 1;
    finish[src] = *time;
}

pub fn print_dfs(prev : &Vec<Option<usize>>, discover : &Vec<usize>, finish : &Vec<usize>){
    println!(" elem | prev | discover | finish |");
    println!("------+------+----------+--------+");
    for i in 0..prev.len() {
        match prev[i] {
            Some(prev) => {
                println!("  {}  |  {}  |    {}    |   {}   |", i, prev, discover[i], finish[i])
            },
            None => {
                println!("  {}  |  {}  |    {}    |   {}   |", i, String::from("none"), discover[i], finish[i])
            }
        }

    }
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

pub fn from_file(path : String) -> Result<Graph, Error>{
    let buff_reader = BufReader::new(fs::File::open(path)?);
    let mut first = true;
    let mut n_edges : usize = 0;
    let mut graph = Graph::new(0, GraphDirected);
    for line in buff_reader.lines() {
        if first {
            let graph_data : (usize, usize, usize) = match line {
                Ok(line) => sscanf::scanf!(line, "{} {} {}", usize, usize, usize).expect("First line wrongly formatted"),
                //I don't really now when this can happen
                Err(e)  => {
                    return Err(Error::new(
                        std::io::ErrorKind::InvalidInput,
                        e.to_string(),
                    ));
                }
            };
            n_edges = graph_data.1;
            let graph_type = if graph_data.2 == 1 {GraphDirected} else {GraphUndirected};
            graph = Graph::new(graph_data.0, graph_type);
            first = false;
        } else {
            let edge_data : (usize, usize, f32) = match line {
                Ok(line) => sscanf::scanf!(line, "{} {} {}", usize, usize, f32).expect("Line not formatted properly"),
                //I don't really now when this can happen
                Err(e)  => {
                    return Err(Error::new(
                        std::io::ErrorKind::InvalidInput,
                        e.to_string(),
                    ));
                }
            };
            graph.add_edge(edge_data.0, edge_data.1, edge_data.2, false)
        }
    }

    if graph.n_edges != n_edges {
        return Err(Error::new(
            std::io::ErrorKind::InvalidInput,
            "Lines prompted not matching grid size",
        ));
    }
    Ok(graph)
}
