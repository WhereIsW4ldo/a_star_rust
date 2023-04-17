use std::{vec};

pub mod a_star;

fn main() {

    let grid_size: (usize, usize) = (10, 10);
    let start: (usize, usize) = (0, 0);
    let end: (usize, usize) = (9, 9);
    let walls: Vec<(usize, usize)> = vec![(1, 1), (2, 2), (3, 3), (4, 4), (5, 5)];

    println!("{:?}", a_star::execute(grid_size, start, end, walls));

}
