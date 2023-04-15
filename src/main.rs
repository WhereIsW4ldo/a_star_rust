fn main() {
    let mut grid: Vec<Vec<Field>> = init_grid(2, 2);
    let mut linked_list: Vec<Node> = vec![];
    let mut open: Vec<&Field> = vec![];
    let mut closed: Vec<&Field> = vec![];

    // set start Field
    grid[0][0].set_start();
    linked_list.push(Node { prev: 0, next: vec![], field: &grid[0][0] });

    // set end Field
    let end: (i32, i32) = (1, 1);
    grid[end.0 as usize][end.1 as usize].set_end();

    // calculate all h costs
    set_all_h(&mut grid, end);

    // add start node to open list
    open.push(&grid[0][0]);
    grid[0][0].state = State::Open;

    dbg!(&grid);
}

fn init_grid(size_x: i32, size_y: i32) -> Vec<Vec<Field>> {
    vec![vec![Field::new(Tile::Ground); size_x as usize]; size_y as usize]
}

fn set_all_h(grid: &mut Vec<Vec<Field>>, (x, y): (i32, i32)) {
    for (i, vector) in grid.into_iter().enumerate(){
        for (j, field) in vector.into_iter().enumerate() {
            let x = (x - i as i32).abs();
            let y = (y - j as i32).abs();
            field.set_h(x*x + y*y);
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
struct Node<'a> {
    prev: usize,
    next: Vec<usize>,
    field: &'a Field
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
    h: i32
}

impl Field {
    pub fn new(tile: Tile) -> Field {
        Field {
            state: State::None,
            tile,
            g: 0,
            h: 0
        }
    }

    pub fn set_start(&mut self) {
        self.tile = Tile::Start;
    }

    pub fn set_end(&mut self) {
        self.tile = Tile::End;
    }

    pub fn reset(&mut self) {
        self.tile = Tile::Ground;
    }

    pub fn set_g(&mut self, g: i32) {
        self.g = g;
    }

    pub fn set_h(&mut self, h: i32) {
        self.h = h;
    }

    pub fn cal_f(&self) -> i32{
        self.g + self.h
    }
}

impl std::fmt::Display for Field {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Field: state: {}, tile: {}, g: {}, h: {}", self.state, self.tile, self.g, self.h)
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
        f1.reset();
        assert_eq!(Tile::Ground, f1.tile);
    }

    #[test]
    fn test_start_values() {
        let f1 = Field::new(Tile::Ground);
        assert_eq!(0 as i32, f1.g);
        assert_eq!(0 as i32, f1.h);
    }

    #[test]
    fn test_h() {
        let mut f1 = Field::new(Tile::Ground);
        f1.set_g(10);
        f1.set_h(-2);
        assert_eq!(8 as i32, f1.cal_f());
        f1.set_g(2);
        f1.set_h(-2);
        assert_eq!(0 as i32, f1.cal_f());
    }
}
