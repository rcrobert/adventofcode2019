use std::io;
use std::io::{BufReader, BufRead};
use std::iter::Iterator;
use std::cmp::{Eq, Ordering};


/// Represents direction on a compass.
#[derive(Copy, Clone, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

/// Represents a range of values from [lower, upper).
#[derive(Copy, Clone)]
struct Interval {
    lower: i64,
    upper: i64,
}

/// Represents a point in 2 dimensions.
#[derive(Copy, Clone, Debug, Eq)]
struct Point {
    x: i64,
    y: i64,
}

/// Represents an intersection of two `Wires`.
#[derive(Debug, Eq)]
struct Intersection {
    /// The sum of the distances along the two wires to reach this intersection.
    distance: u64,
    point: Point,
}

/// Represents an edge of a `Wire`: an origin, direction, and magnitude.
#[derive(Copy, Clone, Debug)]
struct Edge {
    direction: Direction,
    magnitude: i64,
    origin: Point,
}

struct Wire {
    edges: Vec<Edge>,
}

fn main() {
    let stdin = io::stdin();
    let buf = BufReader::new(stdin);

    let mut wires = Vec::<Wire>::with_capacity(2);
    for line in buf.lines() {
        wires.push(match line {
            Ok(line) => Wire::from_string(&line),
            Err(err) => panic!("Failed to read line: {:?}", err),
        });
    }

    let wire_0 = &wires[0];
    let wire_1 = &wires[1];

    let mut intersections = wire_0.get_intersections(wire_1);
    intersections.sort();

    println!("Closest intersection is: {:?} which is {} units away", intersections[0], intersections[0].distance);
}

impl Point {
    /// Positions are colinear on a compass, not on any 2 dimensional line.
    fn colinear(&self, r: &Point) -> bool {
        self.x == r.x || self.y == r.y
    }

    /// Distance from the origin, uses Manhattan distance.
    fn distance_from_origin(&self) -> u64 {
        self.x.abs() as u64 + self.y.abs() as u64
    }

    /// Distance from the other Point, uses Manhattan distance.
    fn distance_from(&self, other: &Self) -> u64 {
        let x_distance = (self.x - other.x).abs() as u64;
        let y_distance = (self.y - other.y).abs() as u64;
        x_distance + y_distance
    }
}

impl Ord for Point {
    fn cmp(&self, other: &Self) -> Ordering {
        self.distance_from_origin().cmp(&other.distance_from_origin())
    }
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self == other
    }
}

impl Intersection {
    fn new(distance: u64, point: Point) -> Self {
        Self {
            distance,
            point,
        }
    }
}

impl Ord for Intersection {
    fn cmp(&self, other: &Self) -> Ordering {
        self.distance.cmp(&other.distance)
    }
}

impl PartialOrd for Intersection {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Intersection {
    fn eq(&self, other: &Self) -> bool {
        self == other
    }
}

impl Interval {
    fn new(lower: i64, upper: i64) -> Self {
        assert!(lower < upper);
        Self {
            lower,
            upper,
        }
    }

    fn contains(&self, value: i64) -> bool {
        self.lower <= value && value < self.upper
    }

    fn overlaps(&self, other: &Self) -> bool {
        self.contains(other.lower) || other.contains(self.lower)
    }
}

impl Edge {
    fn get_endpoint(&self) -> Point {
        match self.direction {
            Direction::Up => Point {x: self.origin.x, y: self.origin.y + self.magnitude as i64},
            Direction::Down => Point {x: self.origin.x, y: self.origin.y - self.magnitude as i64},
            Direction::Left => Point {x: self.origin.x - self.magnitude as i64, y: self.origin.y},
            Direction::Right => Point {x: self.origin.x + self.magnitude as i64, y: self.origin.y},
        }
    }

    /// Transforms this `Edge` to a directionless `Interval`.
    fn as_interval(&self) -> Interval {
        let endpoint = self.get_endpoint();
        match self.direction {
            Direction::Up => Interval::new(self.origin.y, endpoint.y + 1),
            Direction::Down => Interval::new(endpoint.y - 1, self.origin.y),
            Direction::Left => Interval::new(endpoint.x - 1, self.origin.x),
            Direction::Right => Interval::new(self.origin.x, endpoint.x + 1),
        }
    }

    fn parallel(&self, other: &Self) -> bool {
        match self.direction {
            Direction::Up | Direction::Down => {
                match other.direction {
                    Direction::Up | Direction::Down => true,
                    Direction::Left | Direction::Right => false,
                }
            },
            Direction::Left | Direction::Right => {
                match other.direction {
                    Direction::Up | Direction::Down => false,
                    Direction::Left | Direction::Right => true,
                }
            },
        }
    }

    fn colinear(&self, other: &Self) -> bool {
        self.origin.colinear(&other.origin)
    }

    fn is_overlapping(&self, other: &Self) -> bool {
        if !(self.colinear(other) && self.parallel(other)) {
            false
        } else {
            let my_interval = self.as_interval();
            let other_interval = other.as_interval();

            my_interval.overlaps(&other_interval)
        }
    }

    fn is_crossing(&self, other: &Self) -> bool {
        if self.parallel(other) {
            return false;
        }

        let my_interval = self.as_interval();
        let other_interval = other.as_interval();

        match self.direction {
            Direction::Up | Direction::Down => {
                // If we are between their origin and endpoint wrt X
                // If we are surrounding their origin and endpoint wrt Y
                other_interval.contains(self.origin.x) && my_interval.contains(other.origin.y)
            },
            Direction::Left | Direction::Right => {
                // If we are between their origin and endpoint wrt Y
                // If we are surrounding their origin and endpoint wrt X
                other_interval.contains(self.origin.y) && my_interval.contains(other.origin.x)
            },
        }
    }

    fn get_intersection(&self, other: &Self) -> Option<Point> {
        if !self.is_crossing(other) {
            return None;
        }

        match self.direction {
            Direction::Up | Direction::Down => Some(Point { x: self.origin.x, y: other.origin.y, }),
            Direction::Left | Direction::Right => Some(Point { x: other.origin.x, y: self.origin.y, }),
        }
    }
}

impl Wire {
    fn from_string(string: &String) -> Self {
        let mut current_position = Point {
            x: 0,
            y: 0,
        };
        let mut edges = Vec::<Edge>::new();
        for edge_str in string.trim().split(",") {
            let edge = Wire::create_edge(edge_str, &current_position);
            current_position = edge.get_endpoint();
            edges.push(edge);
        }

        Wire {
            edges: edges,
        }
    }

    fn create_edge(vector_str: &str, current_pos: &Point) -> Edge {
        let direction = &vector_str[0..1];
        let direction = match direction {
            "U" => Direction::Up,
            "D" => Direction::Down,
            "L" => Direction::Left,
            "R" => Direction::Right,
            _ => panic!("Unrecognized direction!"),
        };

        let magnitude = &vector_str[1..];
        let magnitude: i64 = magnitude.parse().expect("Failed to parse magnitude");

        Edge {
            direction: direction,
            magnitude: magnitude,
            origin: *current_pos,
        }
    }

    fn iter(&self) -> WireIter {
        WireIter {
            data: &self,
            index: 0,
        }
    }

    fn get_intersections(&self, other: &Self) -> Vec<Intersection> {
        let mut result = Vec::<Intersection>::new();

        let mut my_distance: u64 = 0;

        for edge in self.iter() {
            let mut other_distance: u64 = 0;
            for other_edge in other.iter() {
                assert!(!edge.is_overlapping(&other_edge));
                match edge.get_intersection(&other_edge) {
                    None => (),
                    Some(intersection) => {
                        // Find the partial distance from these edges
                        let mut my_partial_distance = intersection.distance_from(&edge.origin);
                        my_partial_distance += my_distance;
                        let mut other_partial_distance = intersection.distance_from(&other_edge.origin);
                        other_partial_distance += other_distance;

                        let intersection = Intersection::new(my_partial_distance + other_partial_distance,
                                                             intersection);
                        result.push(intersection);
                    },
                }

                other_distance = other_distance + other_edge.magnitude as u64;
            }

            my_distance = my_distance + edge.magnitude as u64;
        }

        result
    }
}

struct WireIter<'a> {
    data: &'a Wire,
    index: usize,
}

impl<'a> Iterator for WireIter<'a> {
    type Item = Edge;
    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.data.edges.len() {
            None
        } else {
            self.index += 1;
            Some(self.data.edges[self.index - 1])
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_crossing_midsection() {
        let base_edge = Edge {
            direction: Direction::Right,
            magnitude: 10,
            origin: Point { x: 0, y: 0 },
        };

        let crossing_edge = Edge {
            direction: Direction::Up,
            magnitude: 10,
            origin: Point { x: 5, y: -5 },
        };

        assert!(crossing_edge.is_crossing(&base_edge));
        assert!(base_edge.is_crossing(&crossing_edge));
    }

    #[test]
    fn test_crossing_at_endpoints() {
        let base_edge = Edge {
            direction: Direction::Right,
            magnitude: 10,
            origin: Point { x: 0, y: 0 },
        };

        let crossing_edge = Edge {
            direction: Direction::Up,
            magnitude: 10,
            origin: Point { x: 0, y: -5 },
        };

        assert!(crossing_edge.is_crossing(&base_edge));
        assert!(base_edge.is_crossing(&crossing_edge));
    }

    #[test]
    fn test_parallel_not_crossing() {
        let base_edge = Edge {
            direction: Direction::Right,
            magnitude: 2,
            origin: Point { x: 0, y: 0 },
        };

        let crossing_edge = Edge {
            direction: Direction::Left,
            magnitude: 5,
            origin: Point { x: 2, y: 0 },
        };

        assert!(!crossing_edge.is_crossing(&base_edge));
        assert!(!base_edge.is_crossing(&crossing_edge));
    }
}
