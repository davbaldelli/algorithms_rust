use std::borrow::Borrow;
use std::cmp::Ordering::{Equal, Greater, Less};
use std::fmt::{Debug, Display};
use std::fs;
use std::io::{BufRead, BufReader, Error};
use std::str::FromStr;
use crate::graphs::Cardinal::{EST, NORTH, SOUTH, WEST};
use crate::graphs::Color::{BLACK, GREY, WHITE};
use crate::graphs::GraphType::{GraphDirected, GraphUndirected};
use crate::MinHeap;
use queues::{IsQueue, Queue};
use sscanf::scanf;

pub trait Edge {
    fn new(src: usize, dst: usize, weight: f32) -> Self;

    fn source(&self) -> usize;

    fn destination(&self) ->usize;

    fn weight(&self) -> f32;
}

pub struct NormalEdge{
    src: usize,
    dst: usize,
    weight: f32,
}

pub struct Graph<T> where T : Edge{
    ///Number of nodes
    n_nodes: usize,
    ///Number of edges
    n_edges: usize,
    ///Graph type
    g_type: GraphType,
    ///Adjacency list of each node:
    ///All the edges starting from a particular node
    pub(crate) edges: Vec<Vec<T>>,
    ///Numbers of edges that ends in the i node
    in_deg: Vec<usize>,
    ///Numbers of edges that starts from the i node
    out_deg: Vec<usize>,
}

pub enum Cardinal {
    NORTH,
    SOUTH,
    WEST,
    EST,
}

#[derive(PartialEq, Eq)]
pub enum GraphType {
    GraphUndirected,
    GraphDirected,
}

pub trait Printable {
    fn print(&self, src : usize, dst : usize) -> ();
}

type ShortestPathTree<'a, T> = (Vec<Option<&'a T>>, Vec<f32>);
type BFSTree<'a, T> = (Vec<Option<&'a T>>, Vec<i32>);
type DFSTree<'a, T> = (Vec<Option<&'a T>>, Vec<usize>, Vec<usize>);
type AllShortestPathTree<'a, T>  = (Vec<Vec<Option<&'a T>>>, Vec<Vec<f32>>);

impl<T> Printable for ShortestPathTree<'_, T> where T : Edge {
    fn print(&self, src : usize, dst : usize) -> () {
        let (prevs, dists) = self;
        print!("{} to {} | weight : {} | path : ", src, dst, dists[dst]);
        print_path(prevs.clone(),src, dst);
        println!();
    }
}

impl<T> Printable for BFSTree<'_, T>  where T : Edge {
    fn print(&self, src: usize, dst: usize) -> () {
        let (prevs, dists) = self;
        print!("{} to {} | weight : {} | path : ", src, dst, dists[dst]);
        print_path(prevs.clone(),src, dst);
        println!();
    }
}

impl<T> Printable for DFSTree<'_, T>  where T: Edge {
    fn print(&self, src: usize, dst: usize) -> () {
        let (prevs, discover, finish) = self;
        println!(" elem | prev | discover | finish |");
        println!("------+------+----------+--------+");
        for i in 0..prevs.len() {
            match prevs[i] {
                Some(prev) => {
                    println!("  {}  |  {}  |    {}    |   {}   |", i, prev.source(), discover[i], finish[i])
                },
                None => {
                    println!("  {}  |  {}  |    {}    |   {}   |", i, String::from("none"), discover[i], finish[i])
                }
            }

        }
    }
}

impl<T> Printable for AllShortestPathTree<'_, T>  where T : Edge {
    fn print(&self, src: usize, dst: usize) -> () {
        let (prevs, dists) = self;
        print!("{} to {} | weight : {} | path : ", src, dst , dists[src][dst]);
        print_all_pairs_sp(prevs.clone(), src, dst);
        println!();
    }
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

impl Edge for NormalEdge{
    fn new(src: usize, dst: usize, weight: f32) -> Self {
        NormalEdge {
            src,
            dst,
            weight,
        }
    }

    fn source(&self) -> usize{
        self.src
    }

    fn destination(&self) ->usize{
        self.dst
    }

    fn weight(&self) -> f32 {
        self.weight
    }
}

impl<T> Graph<T>  where T : Edge{

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
    pub fn new(n_nodes: usize, g_type: GraphType) -> Self {
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

    fn insert_edge(&mut self, src: usize, dst: usize, weight: f32) {
        self.edges[src].push(T::new(src, dst, weight));
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
    pub fn add_edge(&mut self, src: usize, dst: usize, weight: f32){
        self.insert_edge(src, dst, weight);
        if self.g_type == GraphUndirected {
            self.insert_edge(dst, src, weight);
        }
        self.n_edges += 1;
    }

    pub fn print(&self){
        println!("{}", if self.g_type == GraphDirected {String::from("DIRECTED")} else {String::from("UNDIRECTED")});
        for i in 0..self.edges.len() {
            print!("[ {}]", i);
            for edge in self.edges[i].as_slice() {
                print!(" -> ({}, {}, {})", edge.source(), edge.destination(), edge.weight());
            }
            println!();
        }
    }

    /// Returns the shortest path tree with the BFS Algorithm
    /// ( O(|E|+|V|) with |V| = number of nodes, |E| = number of edges) of the given graph
    /// from the give source.
    ///
    /// The first value is the list of the previous edge.
    ///
    /// The second value is the list of the distances from the source.
    ///
    ///
    /// # Arguments
    /// * `graph` - graph where to execute the algorithm
    /// * `source` - source node of the shortest path tree
    ///
    pub fn bfs(&self, source: usize) -> BFSTree<T> {

        let mut colors: Vec<Color> = vec![WHITE; self.n_nodes];
        let mut distances: Vec<i32> = vec![-1; self.n_nodes];
        let mut prev_edge: Vec<Option<&T>> = vec![None; self.n_nodes];
        let mut queue: Queue<usize> = Queue::new();

        colors[source] = GREY;
        distances[source] = 0;

        let _ = queue.add(source);

        while queue.size() != 0 {
            let src = queue.remove().unwrap();
            for edge in self.edges[src].as_slice() {
                if colors[edge.destination()] == WHITE {
                    colors[edge.destination()] = GREY;
                    distances[edge.destination()] = distances[src] + 1;
                    prev_edge[edge.destination()] = Some(edge);
                    let _ = queue.add(edge.destination());
                }
            }
            colors[src] = BLACK;
        }

        return (prev_edge, distances);
    }

    /// Returns a spanning tree of the given graph with the Depth First Search
    /// ( O(|E|+|V|) with |V| = number of nodes, |E| = number of edges)
    ///
    /// The first output is the predecessor list.
    ///
    /// The second output is the discover time list.
    ///
    /// The second output is the finish time list.
    ///
    /// # Arguments
    /// * `graph` - graph where to execute the algorithm
    pub fn dfs(&self) -> DFSTree<T>{
        let mut discover: Vec<usize> = vec![0;self.n_nodes];
        let mut finish: Vec<usize> = vec![0;self.n_nodes];
        let mut color: Vec<Color> = vec![WHITE;self.n_nodes];
        let mut prev: Vec<Option<&T>> = vec![None;self.n_nodes];
        let mut time = 0;

        for i in 0..self.n_nodes() {
            if color[i] == WHITE {
                self.dfs_visit(i, &mut discover, &mut finish, &mut color, &mut prev, &mut time);
            }
        }

        (prev, discover, finish)
    }

    fn dfs_visit<'a>(&'a self, src : usize, discover: &mut Vec<usize>, finish: &mut Vec<usize>
                     , color: &mut Vec<Color>, prev: &mut Vec<Option<&'a T>>, time: &mut usize){
        *time += 1;
        discover[src] = *time;
        color[src] = GREY;

        for edge in self.edges[src].as_slice() {
            let dst = edge.destination();
            if color[dst] == WHITE {
                prev[dst] = Some(edge);
                self.dfs_visit(dst, discover, finish, color, prev, time);
            }
        }

        color[src] = BLACK;
        *time += 1;
        finish[src] = *time;
    }

    /// Returns the shortest path tree with the Dijkstra Algorithm (O(|E|+|V|log(|V|) with |V| = number of nodes, |E| = number of edges) of the given graph
    /// from the give source.
    ///
    /// # Arguments
    /// * `graph` - graph where to execute the algorithm
    /// * `source` - source node of the shortest path tree
    pub fn dijkstra(&self, source: usize) -> ShortestPathTree<T> {
        let mut distances: Vec<f32> = vec![f32::MAX - (1000.0 * 1000.0); self.n_nodes];
        let mut heap = MinHeap::new();
        let mut added = vec![false; self.n_nodes];
        let mut prev_edge: Vec<Option<&T>> = vec![None; self.n_nodes];

        distances[source] = 0.0;

        for i in 0..self.n_nodes {
            heap.insert(i, distances[i])
        }

        while !heap.is_empty() {
            let u = heap.delete_min();
            added[u] = true;
            for edge in self.edges[u].as_slice() {
                let weight = edge.weight();
                let dst = edge.destination();
                if weight < 0.0 {
                    panic!("Dijkstra algorithms do not handles negative weights!")
                }
                if !added[dst] && (distances[u] + weight < distances[dst]) {
                    distances[dst] = distances[u] + weight;
                    heap.change_prio(dst, distances[dst]);
                    prev_edge[dst] = Some(edge);
                }
            }
        }

        return (prev_edge, distances);
    }

    ///Missing Doc.
    pub fn bellman_ford(&self, source: usize) -> Option<ShortestPathTree<T>> {
        let mut distances: Vec<f32> = vec![f32::MAX - (1000.0 * 1000.0); self.n_nodes];
        let mut prev_edge: Vec<Option<&T>> = vec![None; self.n_nodes];

        distances[source] = 0.0;

        for i in 0..self.n_nodes-1 {
            for edge in self.edges[i].as_slice() {
                if distances[edge.destination()] > (distances[edge.source()] + edge.weight()) {
                    distances[edge.destination()] = distances[edge.source()] + edge.weight();
                    prev_edge[edge.destination()] = Some(edge);
                }
            }
        }

        for i in 0..self.n_nodes {
            for edge in self.edges[i].as_slice(){
                if distances[edge.destination()] > (distances[edge.source()] + edge.weight()) {
                    println!("{}>{}",distances[edge.destination()], distances[edge.source()] + edge.weight());
                    return None
                }
            }
        }
        return Some((prev_edge, distances));
    }

    ///Missing Doc.
    pub fn floyd_warshall(&self) -> AllShortestPathTree<T> {
        let n = self.n_nodes();
        let mut dists : Vec<Vec<f32>> = Vec::new();
        let mut prevs: Vec<Vec<Option<&T>>> = Vec::new();

        for i in 0..n {
            prevs.push(Vec::new());
            dists.push(Vec::new());
            for j in 0..n {
                prevs[i].push(None);
                dists[i].push(if i == j { 0.0 } else {f32::MAX - (1000.0 * 1000.0) });
            }
        }

        for i in 0..n {
            for edge in self.edges[i].as_slice() {
                prevs[edge.source()][edge.destination()] = Some(edge);
                dists[edge.source()][edge.destination()] = edge.weight();
            }
        }

        for k in 0..n {
            let old_dists = dists.clone();
            let old_prevs = prevs.clone();
            for i in 0..n {
                for j in 0..n {
                    if j != k && old_dists[i][j] > old_dists[i][k] + old_dists[k][j] {
                        dists[i][j] = old_dists[i][k] + old_dists[k][j];
                        prevs[i][j] = old_prevs[k][j];
                    }
                }
            }
        }
        (prevs, dists)
    }

    ///Missing Doc
    pub fn approx_vertex_cover(&self) -> Vec<&T> {
        let mut cover : Vec<&T> = Vec::new();
        let mut covered : Vec<bool> = vec![false; self.n_nodes];
        let mut edges = &self.edges;
        for i in 0..self.n_nodes{
            if !covered[i] {
                for edge in &edges[i] {
                    if !covered[edge.destination()] {
                        cover.push(edge);
                        covered[edge.destination()] = true;
                        covered[i] = true;
                        break ;
                    }
                }
            }
        }
        cover
    }

}

#[derive(PartialEq, Eq, Clone)]
enum Color {
    BLACK,
    WHITE,
    GREY,
}

pub fn print_path<T>(pred: Vec<Option<&T>>, src: usize, dst: usize) where T : Edge{
    if src == dst {
        print!("{}", src)
    } else {
        match pred[dst] {
            Some(edge) => {
                print_path(pred,src, edge.source());
                print!("->{}", dst);
            }
            None => println!("-1"),
        }
    }
}

pub fn print_all_pairs_sp<T>(prevs: Vec<Vec<Option<&T>>>, src : usize, dst: usize)where T : Edge {
    if src == dst {
        print!("{}", src)
    } else {
        match prevs[src][dst] {
            Some(edge) => {
                print_all_pairs_sp(prevs, src, edge.source());
                print!("->{}", dst)
            },
            None => print!("-1")
        }
    }
}

pub fn from_file(path : String) -> Result<Graph<NormalEdge>, Error>{
    let buff_reader = BufReader::new(fs::File::open(path)?);
    let mut first = true;
    let mut n_edges : usize = 0;
    let mut graph = Graph::new(0, GraphDirected);
    for line in buff_reader.lines() {
        if first {
            let (n_nodes, edges_count, g_type) = match line {
                Ok(line) => scanf!(line, "{} {} {}", usize, usize, usize).expect("First line wrongly formatted"),
                //I don't really now when this can happen
                Err(e)  => {
                    return Err(Error::new(
                        std::io::ErrorKind::InvalidInput,
                        e.to_string(),
                    ));
                }
            };
            n_edges = edges_count;
            let graph_type = if g_type == 1 {GraphDirected} else {GraphUndirected};
            graph = Graph::new(n_nodes, graph_type);
            first = false;
        } else {
            let (src, dst, weight) = match line {
                Ok(line) => scanf!(line, "{} {} {}", usize, usize, f32).expect("Line not formatted properly"),
                //I don't really now when this can happen
                Err(e)  => {
                    return Err(Error::new(
                        std::io::ErrorKind::InvalidInput,
                        e.to_string(),
                    ));
                }
            };
            graph.add_edge(src, dst, weight);
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
