use std::{vec};


use serde::{Deserialize, Serialize};

pub fn execute(grid_size: (usize, usize), start: (usize, usize), end: (usize, usize), walls: Vec<(usize, usize)>) -> Data {

    let mut grid: Vec<Vec<Field>> = init_grid(grid_size);
    let mut linked_list: Vec<Node> = vec![];
    let mut open: Vec<(usize, usize)> = vec![];
    let mut closed: Vec<(usize, usize)> = vec![];

    // set some wall fields
    for wall in walls {
        grid[wall.0][wall.1].tile = Tile::Wall;
        // println!("\n\n\n\nwall: {:?}\n\n\n\n", grid[wall.0][wall.1]);
    }

    // set start Field
    grid[start.0][start.1].set_start();
    linked_list.push(Node {
        prev: start,
        field: start,
    });

    // set end Field
    grid[end.0 as usize][end.1 as usize].set_end();

    // calculate all h costs
    set_all_h(&mut grid, end);

    // add start node to open list
    open.push(start);
    grid[start.0][start.1].state = State::Open;

    let data = calculate(open, &mut closed, &mut grid, &mut linked_list, start, end);
    
    return data;
}

fn init_grid((size_x, size_y): (usize, usize)) -> Vec<Vec<Field>> {
    vec![vec![Field::new(Tile::Ground); size_x]; size_y]
}

fn set_all_h(grid: &mut Vec<Vec<Field>>, (x, y): (usize, usize)) {
    for (i, vector) in grid.into_iter().enumerate() {
        for (j, field) in vector.into_iter().enumerate() {
            let x = (x as i32 - i as i32).abs();
            let y = (y as i32 - j as i32).abs();
            field.set_h(x * x + y * y);
        }
    }
}

fn calculate_f(grid: &mut Vec<Vec<Field>>) {
    for vector in grid {
        for field in vector {
            field.cal_f();
        }
    }
}

fn get_adjacents((y, x): (usize, usize), len_x: usize, len_y: usize) -> Vec<(usize, usize)> {
    let mut adjacents: Vec<(usize, usize)> = vec![];

    if y > 0 {
        adjacents.push((y - 1, x));
    }
    if y < len_y - 1 {
        adjacents.push((y + 1, x));
    }
    if x > 0 {
        adjacents.push((y, x - 1));
    }
    if x < len_x - 1 {
        adjacents.push((y, x + 1));
    }

    adjacents
}

#[derive(Debug, Clone)]
#[derive(Serialize, Deserialize)]
pub struct Data{
    start: (usize, usize),
    end: (usize, usize),
    open: Vec<(usize, usize)>,
    closed: Vec<(usize, usize)>,
    path: Vec<(usize, usize)>,
}

impl std::fmt::Display for Data {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "start: {:?}, \nend: {:?}, \nopen: {:?}, \nclosed: {:?}, \npath: {:?}", self.start, self.end, self.open, self.closed, self.path)
    }
}

fn calculate(mut open: Vec<(usize, usize)>, closed: &mut Vec<(usize, usize)>, grid: &mut Vec<Vec<Field>>, linked_list: &mut Vec<Node>, start: (usize, usize), end: (usize, usize)) -> Data{
    loop {
        calculate_f(grid);
            // look for lowest F-cost square in open list
        if open.is_empty() {
            return Data {
                start,
                end,
                open,
                closed: closed.clone(),
                path: vec![],
            };
        }
        let lowest = open.iter().min_by(|a, b| {
            grid[a.0][a.1].f.cmp(&grid[b.0][b.1].f)
        }).unwrap();
        // println!("open: {:?}", open);
        // println!("lowest: ({}, {}) {:?}", lowest.0, lowest.1, grid[lowest.0][lowest.1]);
        let current_square: (usize, usize) = *lowest;

        if current_square.0 == end.0 as usize && current_square.1 == end.1 as usize {
            return Data {
                start,
                end,
                open,
                closed: closed.clone(),
                path: backtrack(linked_list, start, end),
            };
        }

        // switch current_square to closed list
        closed.push(current_square);
        grid[current_square.0][current_square.1].state = State::Closed;
        let index = open
            .iter()
            .position(|x| x.0 == current_square.0 && x.1 == current_square.1)
            .unwrap();
        open.remove(index);

        // for each of the 4 adjacent to this current square
        let adj = get_adjacents(current_square, grid.len(), grid[0].len());
        // println!("adj: {:?}", adj);
        for adjacent in &adj {
            //If it is not walkable or if it is on the closed list, ignore it. Otherwise do the following.
            // println!("grid[adjacent.0][adjacent.1].tile: {:?}", grid[adjacent.0][adjacent.1].tile);
            if closed.contains(adjacent) || grid[adjacent.0][adjacent.1].tile == Tile::Wall {
                continue;
            }

            //If it isn’t on the open list, add it to the open list. Make the current square the parent of this square. Record the F, G, and H costs of the square.
            if !open.contains(adjacent) {
                grid[adjacent.0][adjacent.1].state = State::Open;
                open.push(*adjacent);
                let last_g = grid[current_square.0][current_square.1].g;
                grid[adjacent.0][adjacent.1].set_g(last_g + 1);
                linked_list.push(Node {
                    prev: current_square,
                    field: *adjacent,
                });
            }
            //If it is on the open list already, check to see if this path to that square is better, using G cost as the measure. A lower G cost means that this is a better path. If so, change the parent of the square to the current square, and recalculate the G and F scores of the square. If you are keeping your open list sorted by F score, you may need to resort the list to account for the change.
            else {
                if grid[adjacent.0][adjacent.1].g > grid[current_square.0][current_square.1].g + 1 {
                    linked_list.push(Node {
                        prev: current_square,
                        field: *adjacent,
                    });
                }
            }

            calculate_f(grid);
        }
    }
}

fn backtrack(linked_list: &Vec<Node>, start: (usize, usize), end: (usize, usize)) -> Vec<(usize, usize)>{
    // println!("linked_list: {:?}", linked_list);
    // println!("start: {:?}", start);
    // println!("end: {:?}", end);

    if let None = linked_list.iter().position(|node| node.field.0 == end.0 && node.field.1 == end.1) {
        return vec![(linked_list[0].field.0, linked_list[0].field.1)]
    }
    let index_end = linked_list.iter().position(|node| node.field.0 == end.0 && node.field.1 == end.1).unwrap();

    let mut prev = &linked_list[index_end];
    let mut path: Vec<(usize, usize)> = vec![];

    loop {
        // println!("{:?}", prev.field);
        if prev.field.0 == start.0 && prev.field.1 == start.1 {
            break;
        }
        match linked_list.iter().position(|node| node.field == prev.prev) {
            Some(index) => {
                prev = &linked_list[index];
                path.push(prev.field);
            },
            None => break,
        }
    }
    path
}

#[derive(Debug, PartialEq, Clone)]
struct Node {
    prev: (usize, usize),
    field: (usize, usize),
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Tile {
    Wall,
    Ground,
    Start,
    End,
}

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Tile::Wall => write!(f, "Wall"),
            Tile::Ground => write!(f, "Ground"),
            Tile::Start => write!(f, "Start"),
            Tile::End => write!(f, "End"),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum State {
    Open,
    Closed,
    None,
}

impl std::fmt::Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            State::Open => write!(f, "Open"),
            State::Closed => write!(f, "Closed"),
            State::None => write!(f, "None"),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct Field {
    state: State,
    tile: Tile,
    g: i32,
    h: i32,
    f: i32,
}

impl Field {
    pub fn new(tile: Tile) -> Field {
        Field {
            state: State::None,
            tile,
            g: 0,
            h: 0,
            f: 0,
        }
    }

    pub fn set_start(&mut self) {
        self.tile = Tile::Start;
    }

    pub fn set_end(&mut self) {
        self.tile = Tile::End;
    }

    pub fn set_g(&mut self, g: i32) {
        self.g = g;
    }

    pub fn set_h(&mut self, h: i32) {
        self.h = h;
    }

    pub fn cal_f(&mut self) -> i32 {
        self.f = self.g + self.h;
        return self.f;
    }
}

impl std::fmt::Display for Field {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Field: state: {}, tile: {}, g: {}, h: {}, f: {}",
            self.state, self.tile, self.g, self.h, self.f
        )
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_init() {
        let f1 = Field::new(Tile::Ground);
        let f2 = Field::new(Tile::Wall);
        assert_eq!(State::None, f1.state);
        assert_eq!(Tile::Ground, f1.tile);
        assert_eq!(Tile::Wall, f2.tile);
    }

    #[test]
    fn test_set_start() {
        let mut f1 = Field::new(Tile::Ground);
        f1.set_start();
        assert_eq!(Tile::Start, f1.tile);
    }

    #[test]
    fn test_set_end() {
        let mut f1 = Field::new(Tile::Ground);
        f1.set_end();
        assert_eq!(Tile::End, f1.tile);
    }

    #[test]
    fn test_reset() {
        let mut f1 = Field::new(Tile::Ground);
        f1.set_end();
        assert_eq!(Tile::End, f1.tile);
    }

    #[test]
    fn test_start_values() {
        let f1 = Field::new(Tile::Ground);
        assert_eq!(0 as i32, f1.g);
        assert_eq!(0 as i32, f1.h);
    }

    #[test]
    fn test_values() {
        let mut f1 = Field::new(Tile::Ground);
        f1.set_g(10);
        f1.set_h(-2);
        assert_eq!(8 as i32, f1.cal_f());
        f1.set_g(2);
        f1.set_h(-2);
        assert_eq!(0 as i32, f1.cal_f());
    }
}
