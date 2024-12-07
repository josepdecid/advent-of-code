use std::collections::HashSet;

type Maze = Vec<Vec<char>>;
type Position = (usize, usize);

#[derive(Copy, Clone, Eq, Hash, PartialEq)]
enum Direction { Up, Down, Left, Right, Stop }
impl Direction { 
    fn delta(&self) -> (i32, i32) {
        match self {
            Direction::Up => (-1, 0),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
            Direction::Right => (0, 1),
            Direction::Stop => (0, 0),
        }
    }
}

fn read_data() -> Maze  {
    let bytes = include_bytes!("input.txt");
    let data = String::from_utf8_lossy(bytes);
    
    return data
        .lines()
        .map(|line| line.chars().collect())
        .collect();
}

fn find_initial_position(maze: &Maze) -> Position {
    for x in 0..maze.len() {
        for y in 0..maze[0].len() {
            if maze[x][y] == '^' {
                return (x, y);
            }
        }
    }

    return (0, 0);
}

fn get_next_position_if_not_out_of_bounds(current: &Position, direction: &Direction, upper_boundary: &Position) -> Option<Position> {
    let next = (current.0 as i32 + direction.delta().0, current.1 as i32 + direction.delta().1);
    if 
        next.0 < 0 || next.0 >= upper_boundary.0 as i32 ||
        next.1 < 0 || next.1 >= upper_boundary.1 as i32
    {
        return None;
    } 

    return Some((next.0 as usize, next.1 as usize));
}

fn next_direction(direction: &Direction) -> Direction {
    return match direction {
        Direction::Up => Direction::Right,
        Direction::Right => Direction::Down,
        Direction::Down => Direction::Left,
        Direction::Left => Direction::Up,
        _ => Direction::Stop,
    };
}

fn maze_has_loop(maze: &Maze, initial_position: Position) -> bool {
    let shape = (maze.len(), maze[0].len());
    let mut direction: Direction = Direction::Up;
    let mut current_position = initial_position; 

    let mut directions_lookup = vec![vec![HashSet::<Direction>::new(); shape.0]; shape.1];
    directions_lookup[current_position.0 as usize][current_position.1 as usize].insert(direction);

    while let Some(next_position) = get_next_position_if_not_out_of_bounds(&current_position, &direction, &shape) { 
        if maze[next_position.0][next_position.1] == '#' {
            direction = next_direction(&direction);
            continue;
        }
        
        if directions_lookup[next_position.0][next_position.1].contains(&direction) {
            return true;
        }
            
        current_position = next_position;
        directions_lookup[current_position.0][current_position.1].insert(direction);   
    }

    return false;
}

fn count_new_objects_that_cause_loop(maze: &mut Maze) -> u32 {
    let mut result: u32 = 0;
    let initial_position = find_initial_position(maze);

    for x in 0..maze.len() {
        for y in 0..maze[0].len() {
            if maze[x][y] == '.' {
                maze[x][y] = '#';
                if maze_has_loop(maze, initial_position) { result += 1; }
                maze[x][y] = '.';
            }
        }
    }

    return result;
}

fn main() {
    let mut maze = read_data();
    let result = count_new_objects_that_cause_loop(&mut maze);
    println!("Result: {}", result);  // 1655 
}
