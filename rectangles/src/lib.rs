use crate::Direction::{Down, Left, Right, Up};
use std::collections::HashSet;

enum Element {
    Vertex,
    HorizontalRib,
    VerticalRib,
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

// Sequence of rectangle tracing directions
const DIRECTIONS: [Direction; 4] = [Right, Down, Left, Up];

struct Board(Vec<Vec<Option<Element>>>);

impl Board {
    fn len(&self) -> (usize, usize) {
        (self.0[0].len(), self.0.len())
    }

    fn element(&self, location: Point) -> Option<&Element> {
        self.0[location.y][location.x].as_ref()
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Default)]
struct Point {
    x: usize,
    y: usize,
}

impl From<(usize, usize)> for Point {
    fn from((x, y): (usize, usize)) -> Self { Self { x, y } }
}

impl Point {
    fn new(x: usize, y: usize) -> Self { Self { x, y } }

    fn move_in_direction(&self, direction: Direction, board: &Board) -> Option<Point> {
        match direction {
            Left => {
                let x = self.x.checked_sub(1)?;
                Some(Point { x, ..*self })
            }
            Right => {
                if self.x + 1 < board.len().0 {
                    Some(Point { x: self.x + 1, ..*self })
                } else { None }
            }
            Up => {
                let y = self.y.checked_sub(1)?;
                Some(Point { y, ..*self })
            }
            Down => {
                if self.y + 1 < board.len().1 {
                    Some(Point { y: self.y + 1, ..*self })
                } else { None }
            }
        }
    }
}

pub fn count(lines: &[&str]) -> u32 {
    if lines.is_empty() || lines[0].is_empty() {
        return 0;
    }

    let board = lines
        .iter()
        .map(|&line| {
            line.chars()
                .map(|c| match c {
                    '-' => Some(Element::HorizontalRib),
                    '|' => Some(Element::VerticalRib),
                    '+' => Some(Element::Vertex),
                    _ => None,
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<Vec<_>>>();

    let board = Board(board);
    let mut rectangles = HashSet::new();

    for y in 0..board.len().1 {
        for x in 0..board.len().0 {
            // Scanning left-to-right and top-to-bottom
            if let Some(Element::Vertex) = board.element(Point::new(x, y)) {
                visit_vertex(&board, (x, y).into(), &mut rectangles);
            }
        }
    }

    rectangles.len() as u32
}

fn visit_vertex(board: &Board, location: Point, rectangles: &mut HashSet<[Point; 4]>) {
    // Start building a new rectangle from this position
    let mut rectangle = [Point::default(); 4];
    rectangle[0] = location;

    // Start walking in the first direction from our sequence (Right)
    if let Some(new_location) = location.move_in_direction(DIRECTIONS[0], board) {
        walk(board, new_location, 0, rectangle, rectangles);
    }
}

fn walk(
    board: &Board,
    location: Point,
    dir_idx: usize,
    current_rectangle: [Point; 4],
    rectangles: &mut HashSet<[Point; 4]>,
) {
    let direction = DIRECTIONS[dir_idx];

    // Select where to move next based on the current location
    // If we cannot proceed moving in the current direction - return
    match board.element(location) {
        None => (),
        Some(Element::HorizontalRib) => {
            if direction == Up || direction == Down {
                return;
            }

            // we can only move forward, if the board boundaries allow
            if let Some(new_location) = location.move_in_direction(direction, board) {
                walk(board, new_location, dir_idx, current_rectangle, rectangles);
            }
        }
        Some(Element::VerticalRib) => {
            if direction == Left || direction == Right {
                return;
            }

            // we can only move forward, if the board boundaries allow
            if let Some(new_location) = location.move_in_direction(direction, board) {
                walk(board, new_location, dir_idx, current_rectangle, rectangles);
            }
        }
        Some(Element::Vertex) => {
            // From here we can either proceed moving in the same direction (ignore the vertex)
            // or try to change direction (try the vertex as the rectangle's vertex).
            // First we try the vertex
            if dir_idx < 3 {
                // Try to use the current vertex as n-th vertex (not last)
                let next_dir_idx = dir_idx + 1;
                let next_dir = DIRECTIONS[next_dir_idx];

                if let Some(new_location) = location.move_in_direction(next_dir, board) {
                    let mut next_rectangle = current_rectangle;
                    next_rectangle[next_dir_idx] = location;

                    walk(board, new_location, next_dir_idx, next_rectangle, rectangles);
                }
            } else {
                // we've got a possible rectangle, check and add it
                if location == current_rectangle[0] {
                    try_add_rectangle(current_rectangle, rectangles);
                    return;
                }
            }

            // Try to ignore the vertex and move next in the same direction
            if let Some(new_location) = location.move_in_direction(direction, board) {
                walk(board, new_location, dir_idx, current_rectangle, rectangles);
            }
        }
    }
}

fn try_add_rectangle(rectangle: [Point; 4], rectangles: &mut HashSet<[Point; 4]>) -> bool {
    let [top_left, top_right, bottom_right, bottom_left] = rectangle;

    // check if the vertices form a valid orthogonal rectangle
    if top_left.y == top_right.y &&
        bottom_left.y == bottom_right.y &&
        top_left.x == bottom_left.x &&
        top_right.x == bottom_right.x
    {
        rectangles.insert(rectangle);
        true
    } else {
        false // not a valid rectangle
    }
}