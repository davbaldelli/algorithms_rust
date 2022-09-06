use crate::graphs::{bfs, dijkstra};
use crate::heap::MinHeap;
use crate::robot::{robot_print_bfs, robot_print_dijkstra};
use std::time::Instant;

mod graphs;
mod heap;
mod robot;
mod sort;

#[test]
fn test_insertion_sort() {
    let mut array = vec![2, 3, -1, 7, 6, 9, 5];
    sort::insertion_sort(&mut array);
    assert_eq!(array, vec![-1, 2, 3, 5, 6, 7, 9])
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

#[test]
fn test_shortest_path() {
    let cell_size: (usize, usize) = (3, 3);
    let graph = match robot::graph_from_file(
        String::from("/home/davide/Documenti/rust/algorithm/algorithms/src/test4.in"),
        cell_size,
    ) {
        Ok(graph) => graph,
        Err(e) => panic!("{}", e.to_string()),
    };
    let mut now = Instant::now();
    println!("Dijkstra started...");
    let _ = dijkstra(&graph, 0);
    println!("Dijkstra elapsed time => {} ms", now.elapsed().as_millis());
    now = Instant::now();
    println!("BFS started...");
    let _ = bfs(&graph, 0);
    println!("BFS elapsed time => {} ms", now.elapsed().as_millis());
}

#[test]
fn test_read_graph(){
    let graph = match graphs::from_file(String::from("/home/davide/Documenti/rust/algorithm/algorithms/src/graph10.in")){
        Ok(graph) => graph,
        Err(e) => panic!("{}", e.to_string()),
    };
    graph.print();
}

fn main() {
    let cell_size: (usize, usize) = (3, 3);
    let graph = match robot::graph_from_file(
        String::from("/home/davide/Documenti/rust/algorithm/algorithms/src/test4.in"),
        cell_size,
    ) {
        Ok(graph) => graph,
        Err(e) => panic!("{}", e.to_string()),
    };
    let mut now = Instant::now();
    println!("Dijkstra started...");
    let _ = dijkstra(&graph, 0);
    println!("Dijkstra elapsed time => {} ms", now.elapsed().as_millis());
    now = Instant::now();
    println!("BFS started...");
    let _ = bfs(&graph, 0);
    println!("BFS elapsed time => {} ms", now.elapsed().as_millis());
    //robot_print_bfs(0, graph.n_nodes - 1, result_bfs.0, result_bfs.1, &result_bfs.2);
}
