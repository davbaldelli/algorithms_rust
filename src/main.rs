extern crate termion;
use std::io;
use std::io::{stdout, Write};
use std::str::FromStr;
use std::thread;
use std::time::{Duration, Instant};
use rand::Rng;
use termion::{color, style};
use crate::binary_trees::BinaryTree;
use crate::graphs::{Edge, from_file, print_all_pairs_sp, Printable};
use crate::heap::MinHeap;
use crate::robot::{robot_graph_from_file, robot_print_bfs};

mod graphs;
mod heap;
mod sort;
mod binary_trees;
mod robot;

#[test]
fn test_insertion_sort() {
    let mut array = vec![2, 3, -1, 7, 6, 9, 5];
    sort::insertion_sort(&mut array);
    assert_eq!(array, vec![-1, 2, 3, 5, 6, 7, 9])
}

#[test]
fn test_quicksort() {
    let mut array = Vec::new();
    let mut rng = rand::thread_rng();
    for _ in 0..1000 {
        array.push(rng.gen_range(0..1000))
    }
    let mut copy = array.clone();
    copy.sort();
    assert_ne!(array, copy);
    let len = array.len();
    sort::quicksort(&mut array, 0, len - 1);
    assert_eq!(array, copy)
}

#[test]
fn test_selection() {
    let mut array = vec![2, 3, -1, 7, 6, 9, 5];
    let len = array.len();
    sort::rand_select(&mut array, 0, len - 1, 3);
    assert_eq!(sort::rand_select(&mut array, 0, len - 1, 3), 3)
}

#[test]
fn test_approx_vertex_cover() {
    let graph = from_file(String::from("/home/davide/Documenti/rust/algorithm/algorithms/src/graph10.in")).expect("Error converting file to graph");
    let cover = graph.approx_vertex_cover();
    graph.print();
    for edge in cover {
        println!("{}->{}", edge.source(), edge.destination());
    }
}

#[test]
fn test_binary_tree() {
    let mut array = vec![2, 3, -1, 7, 6, 9, 5];
    let mut bt = BinaryTree::new();
    for i in 0..array.len() {
        bt.insert_key(array[i]);
    }
}

#[test]
fn test_heap() {
    let mut min_heap = MinHeap::new();
    let array: Vec<f32> = vec![2.0, 3.0, 0.0, 7.0, 6.0, 1.0, 5.0];
    let mut heaped_array: Vec<f32> = vec![0.0, 3.0, 1.0, 7.0, 6.0, 2.0, 5.0];
    for i in 0..array.len() {
        min_heap.insert(i, array[i]);
    }
    for i in 0..array.len() {
        assert_eq!(min_heap.heap[i].prio, heaped_array[i])
    }
    heaped_array = vec![1.0, 3.0, 2.0, 7.0, 6.0, 5.0];
    let min = min_heap.delete_min();
    for i in 0..heaped_array.len() {
        assert_eq!(min_heap.heap[i].prio, heaped_array[i]);
        assert_eq!(min, 2)
    }
    heaped_array = vec![2.0, 3.0, 5.0, 7.0, 6.0, 6.0];
    min_heap.change_prio(5, 6.0);
    for i in 0..heaped_array.len() {
        assert_eq!(min_heap.heap[i].prio, heaped_array[i]);
    }
}

fn shortest_path(path : String){
    let graph = from_file(path).expect("Error converting file to graph");

    let mut now = Instant::now();

    println!("BFS started...");
    let bfst = graph.bfs(0);
    println!("BFS elapsed time => {}{} ms{}", color::Fg(color::Green), now.elapsed().as_millis(), color::Fg(color::Reset));
    bfst.print(0, graph.n_nodes() - 1);

    now = Instant::now();
    println!("Dijkstra started...");
    let spt_dij = graph.dijkstra(0);
    println!("Dijkstra elapsed time => {}{} ms{}", color::Fg(color::Green), now.elapsed().as_millis(), color::Fg(color::Reset));
    spt_dij.print(0, graph.n_nodes() - 1);

    now = Instant::now();
    println!("Bellman-Ford started");
    if let Some(spt_bf) = graph.bellman_ford(0) {
        println!("Bellman-Ford elapsed time => {} ms", now.elapsed().as_millis());
        spt_bf.print(0, graph.n_nodes() - 1);
    } else {
        println!("{}Bellman-Ford failed! It looks there are some negative cycles.{}", color::Fg(color::Red) ,color::Fg(color::Reset));
    }

    now = Instant::now();
    println!("Floyd-Warshall started...");
    let fw_matrix = graph.floyd_warshall();
    println!("Floyd-Warshall execution time => {}{} ms{}",color::Fg(color::Green), now.elapsed().as_millis(), color::Fg(color::Reset));
    fw_matrix.print(0, graph.n_nodes() - 1);
}

fn robot_travel(path : String, cell : (usize, usize)){
    let mut now = Instant::now();
    println!("Started to convert the file...");
    let graph = robot_graph_from_file(path, cell)
        .expect("Error converting file to robot grid");
    println!("File convertion time => {}{} ms{}",color::Fg(color::Green), now.elapsed().as_millis(), color::Fg(color::Reset));
    let (prevs, dists) = graph.bfs(0);
    robot_print_bfs(0, graph.n_nodes()-1, dists, &prevs);
}

fn main() {
    //shortest_path(String::from("/home/davide/Documenti/rust/algorithm/algorithms/src/graph100.in"))
    robot_travel(String::from("/home/davide/Documenti/rust/algorithm/algorithms/src/test4.in"),(1,1))
}

fn get_between<T, F>(first: &T, second: &T, mut func: F) -> T
    where F: FnMut(&T, &T) -> T
{
    func(first, second);
    func(first, second)
}