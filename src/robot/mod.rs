use std::fs;
use std::io::{BufRead, BufReader, Error};
use std::time::Instant;

use crate::graphs::{Edge, Graph};
use crate::graphs::GraphType::GraphUndirected;

pub fn robot_print_bfs(
    src: usize,
    dst: usize,
    pred: Vec<Option<usize>>,
    dist: Vec<i32>,
    prev_edge: &Vec<Option<&Edge>>,
) {
    if dist[dst] > 0 {
        println!("{}", dist[dst]);
        robot_print_path(src, dst, pred, prev_edge);
        println!();
    } else {
        println!("{}", -1);
    }
}

pub fn robot_print_dijkstra(
    src: usize,
    dst: usize,
    pred: Vec<Option<usize>>,
    dist: Vec<usize>,
    prev_edge: &Vec<Option<&Edge>>,
) {
    if dist[dst] != (usize::MAX - (1000 * 1000)) {
        println!("{}", dist[dst]);
        robot_print_path(src, dst, pred, prev_edge);
        println!();
    } else {
        println!("{}", -1);
    }
}

fn robot_print_path(
    src: usize,
    dst: usize,
    pred: Vec<Option<usize>>,
    prev_edge: &Vec<Option<&Edge>>,
) {
    if src != dst {
        match pred[dst] {
            Some(prev_dst) => {
                robot_print_path(src, prev_dst, pred, prev_edge);
                print!("{}", prev_edge[dst].unwrap().direction.value());
            }
            None => println!("-1"),
        }
    }
}

pub fn graph_from_file(path: String, cell_size: (usize, usize)) -> Result<Graph, Error> {
    let result = read_grid_from_file(path)?;
    let grid = result.0;
    let rows = result.1;
    let cols = result.2;
    let mut graph = Graph::new(
        (rows - (cell_size.0 - 1)) * (cols - (cell_size.1 - 1)),
        GraphUndirected,
    );
    for i in 0..(rows - (cell_size.0 - 1)) {
        for j in 0..(cols - (cell_size.1 - 1)) {
            if is_a_node(&grid, i, j, '*', cell_size) {
                if !is_last_col(j, cols, cell_size)
                    && !has_obstacles_on_right(&grid, i, j, '*', cell_size)
                {
                    let vertices = get_src_dst(i, j, cols, cell_size, false);
                    graph.add_edge(vertices.0, vertices.1, 1, false);
                }

                if !is_last_row(i, rows, cell_size)
                    && !has_obstacles_below(&grid, i, j, '*', cell_size)
                {
                    let vertices = get_src_dst(i, j, cols, cell_size, true);
                    graph.add_edge(vertices.0, vertices.1, 1, true);
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
            x = size.unwrap().0;
            y = size.unwrap().1;
        } else {
            match read_grid_file_line(&line, y)? {
                Some(vec) => grid.push(vec),
                //This error occurs when a line is not properly formatted
                None => {
                    return Err(Error::new(
                        std::io::ErrorKind::InvalidInput,
                        "Incorrect line format",
                    ));
                }
            }
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

fn read_first_line(line: Result<String, Error>) -> Result<Option<(usize, usize)>, Error> {
    let parsed: Option<(usize, usize)> = match line {
        Ok(line) => match sscanf::scanf!(line, "{} {}", usize, usize) {
            Ok(parsed) => Some(parsed),
            //This error occurs when the first line is wrongly formatted
            Err(_) => {
                return Err(Error::new(
                    std::io::ErrorKind::InvalidInput,
                    "First line wrongly formatted",
                ));
            }
        },
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

fn read_grid_file_line(
    line: &Result<String, Error>,
    columns: usize,
) -> Result<Option<Vec<char>>, Error> {
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
    Ok(Some(vector))
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
