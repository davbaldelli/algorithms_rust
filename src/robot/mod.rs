use std::fs;
use std::io::{BufRead, BufReader, Error};
use crate::Edge;

use crate::graphs::{Graph};
use crate::graphs::GraphType::GraphUndirected;

#[allow(dead_code)]
#[derive(Clone)]
pub struct RobotEdge{
    src: usize,
    dst: usize,
    weight: f32,
    direction : char
}

impl Edge for RobotEdge{
    fn new(src: usize, dst: usize, weight: f32) -> Self {
        RobotEdge{src, dst, weight, direction : 'N'}
    }

    fn source(&self) -> usize {
        self.src
    }

    fn set_source(&mut self, source: usize) {
        self.src = source
    }

    fn destination(&self) -> usize {
        self.dst
    }

    fn set_destination(&mut self, destination: usize) {
        self.dst = destination
    }

    fn weight(&self) -> f32 {
        self.weight
    }
}

#[allow(dead_code)]
impl RobotEdge {
    pub fn direction(&self) -> char{
        self.direction
    }

    pub fn set_direction(&mut self, dir : char){
        self.direction = dir;
    }
}

#[allow(dead_code)]
pub fn robot_print_bfs(
    src: usize,
    dst: usize,
    dist: Vec<i32>,
    prev_edge: &Vec<Option<&RobotEdge>>,
) {
    if dist[dst] > 0 {
        println!("{}", dist[dst]);
        robot_print_path(src, dst, prev_edge);
        println!();
    } else {
        println!("{}", -1);
    }
}

#[allow(dead_code)]
pub fn robot_print_dijkstra(
    src: usize,
    dst: usize,
    prev_edge: &Vec<Option<&RobotEdge>>,
    dist: Vec<usize>,
) {
    if dist[dst] != (usize::MAX - (1000 * 1000)) {
        println!("{}", dist[dst]);
        robot_print_path(src, dst, prev_edge);
        println!();
    } else {
        println!("{}", -1);
    }
}

fn robot_print_path(
    src: usize,
    dst: usize,
    prev_edge: &Vec<Option<&RobotEdge>>,
) {
    if src != dst {
        match prev_edge[dst] {
            Some(edge) => {
                robot_print_path(src, edge.source(), prev_edge);
                match prev_edge[dst] {
                    Some(edge) => {
                        print!("{}", edge.direction());
                    },
                    None =>()
                }

            }
            None => println!("-1"),
        }
    }
}

#[allow(dead_code)]
pub fn robot_graph_from_file(path: String, cell_size: (usize, usize)) -> Result<Graph<RobotEdge>, Error> {
    let (grid, rows, cols) = read_grid_from_file(path)?;
    let mut graph: Graph<RobotEdge> = Graph::new(
        (rows - (cell_size.0 - 1)) * (cols - (cell_size.1 - 1)),
        GraphUndirected,
    );
    let mut cell_is_node: Vec<Option<bool>> = vec![None; graph.n_nodes()];

    cell_is_node[0] = Some(is_a_node(&grid, 0, 0, '*', cell_size));
    for i in 0..(rows - (cell_size.0 - 1)) {
        for j in 0..(cols - (cell_size.1 - 1)) {
            let src = get_src_dst(i, j, cols, cell_size, false).0;
            if cell_is_node[src].is_none() {
                cell_is_node[src] = Some(is_a_node(&grid, i, j, '*', cell_size));
            }
            if cell_is_node[src].unwrap() {
                if !is_last_col(j, cols, cell_size) {
                    let (src, dst) = get_src_dst(i, j, cols, cell_size, false);
                    if !has_obstacles_on_right(&grid, i, j, '*', cell_size) {
                        graph.create_edge(src, dst, 1.0);
                        let src_len = graph.edges[src].len();
                        graph.edges[src][src_len-1].set_direction('E');
                        let dst_len = graph.edges[dst].len();
                        graph.edges[dst][dst_len-1].set_direction('W');
                        cell_is_node[dst] = Some(true);
                    } else {
                        cell_is_node[dst] = Some(false);
                    }
                }
                if !is_last_row(i, rows, cell_size) {
                    let (src, dst) = get_src_dst(i, j, cols, cell_size, true);
                    if !has_obstacles_below(&grid, i, j, '*', cell_size) {
                        graph.create_edge(src, dst, 1.0);
                        let src_len = graph.edges[src].len();
                        graph.edges[src][src_len-1].set_direction('S');
                        let dst_len = graph.edges[dst].len();
                        graph.edges[dst][dst_len-1].set_direction('N');
                        cell_is_node[dst] = Some(true);
                        cell_is_node[dst] = Some(true);
                    } else {
                        cell_is_node[dst] = Some(false);
                    }
                }
            }
        }
    }
    Ok(graph)
}

fn read_grid_from_file(path: String) -> Result<(Vec<Vec<char>>, usize, usize), Error> {
    let buff_reader = BufReader::new(fs::File::open(path)?);
    let mut i: i32 = -1;
    let mut x: usize = 0;
    let mut y: usize = 0;
    let mut grid: Vec<Vec<char>> = Vec::new();
    for line in buff_reader.lines() {
        if i == -1 {
            let size = read_first_line(line)?;
            x = size.0;
            y = size.1;
        } else {
            grid.push(read_grid_file_line(&line, y)?)
        }
        i += 1;
    }
    if i != x as i32 {
        return Err(Error::new(
            std::io::ErrorKind::InvalidInput,
            "Lines prompted not matching grid size",
        ));
    }
    Ok((grid, x, y))
}

fn read_first_line(line: Result<String, Error>) -> Result<(usize, usize), Error> {
    let parsed: (usize, usize) = match line {
        Ok(line) => sscanf::scanf!(line, "{} {}", usize, usize).expect("First line wrongly formatted"),
        //This error occurs when there is any first line
        Err(_) => {
            return Err(Error::new(
                std::io::ErrorKind::InvalidInput,
                "First line missing",
            ));
        }
    };
    Ok(parsed)
}

fn read_grid_file_line(line: &Result<String, Error>, columns: usize, ) -> Result<Vec<char>, Error> {
    let mut vector = Vec::new();
    match line {
        Ok(line) => {
            for val in line.chars(){
                vector.push(val);
            }
            if vector.len() != columns {
                return Err(Error::new(
                    std::io::ErrorKind::InvalidInput,
                    "The line contains a wrong number of columns",
                ));
            }
        },
        Err(_) => {
            return Err(Error::new(
                std::io::ErrorKind::InvalidInput,
                "Error reading the line",
            ));
        }
    }
    Ok(vector)
}

fn get_src_dst(
    x: usize,
    y: usize,
    cols: usize,
    cell_size: (usize, usize),
    vertical: bool,
) -> (usize, usize) {
    return if vertical {
        let src = (x * (cols - (cell_size.1 - 1))) + y;
        let dst = ((x + 1) * (cols - (cell_size.1 - 1))) + y;
        (src, dst)
    } else {
        let src = (x * (cols - (cell_size.1 - 1))) + y;
        let dst = (x * (cols - (cell_size.1 - 1))) + (y + 1);
        (src, dst)
    };
}

fn is_a_node(
    grid: &Vec<Vec<char>>,
    x: usize,
    y: usize,
    wall: char,
    cell_size: (usize, usize),
) -> bool {
    for i in x..(x + cell_size.0) {
        for j in y..(y + cell_size.1) {
            if grid[i][j] == wall {
                return false;
            }
        }
    }
    true
}

fn has_obstacles_on_right(
    grid: &Vec<Vec<char>>,
    x: usize,
    y: usize,
    wall: char,
    cell_size: (usize, usize),
) -> bool {
    for i in 0..cell_size.0 {
        if grid[x + i][y + cell_size.1] == wall {
            return true;
        }
    }
    return false;
}

fn has_obstacles_below(
    grid: &Vec<Vec<char>>,
    x: usize,
    y: usize,
    wall: char,
    cell_size: (usize, usize),
) -> bool {
    for i in 0..cell_size.1 {
        if grid[x + cell_size.0][y + i] == wall {
            return true;
        }
    }
    return false;
}

fn is_last_col(y: usize, cols: usize, cell_size: (usize, usize)) -> bool {
    return y >= (cols - cell_size.1);
}

fn is_last_row(x: usize, rows: usize, cell_size: (usize, usize)) -> bool {
    return x >= (rows - cell_size.0);
}
