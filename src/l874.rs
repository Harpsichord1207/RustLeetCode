use std::collections::HashSet;

#[allow(dead_code)]
pub struct Solution;

#[derive(Copy, Clone, Eq, PartialEq, Hash, Default)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    fn from_vec(v: Vec<i32>) -> Option<Self> {
        if let (Some(&x), Some(&y)) = (v.get(0), v.get(1)) {
            Some(Self { x,  y })
        } else {
            None
        }
    }

    fn move_points(&self, direction: Direction, step: i32, obstacles: &HashSet<Self>) -> Self {
        let (mut x, mut y) = (self.x, self.y);
        for _ in 0..step {
            let p = Point::new(x, y);
            match direction {
                Direction::North => y += 1,
                Direction::South => y -= 1,
                Direction::East => x += 1,
                Direction::West => x -= 1,
            }
            if obstacles.contains(&Point::new(x, y)) {
                return p;
            }
        }
        Point::new(x, y)
    }

    fn distance(&self) -> i32 {
        self.x * self.x + self.y * self.y
    }
}


#[derive(Clone, Copy)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn turn_right(direction: Self) -> Self {
        return match direction {
            Self::North => Self::East,
            Self::South => Self::West,
            Self::East => Self::South,
            Self::West => Self::North,
        }
    }

    fn turn_left(direction: Self) -> Self {
        return match direction {
            Self::North => Self::West,
            Self::South => Self::East,
            Self::East => Self::North,
            Self::West => Self::South,
        }
    }
}

impl Solution {
    #[allow(dead_code)]
    pub fn robot_sim(commands: Vec<i32>, obstacles: Vec<Vec<i32>>) -> i32 {
        let obstacles = obstacles
            .into_iter()
            .map(|v| Point::from_vec(v).unwrap())
            .collect::<HashSet<Point>>();
        let mut position = Point::default();
        let mut direction = Direction::North;

        let mut result = 0;
        for command in commands {
            match command {
                -1 => direction = Direction::turn_right(direction),
                -2 => direction = Direction::turn_left(direction),
                s => position = position.move_points(direction, s, &obstacles),
            }
            println!("Current at ({}, {}), distance: {}", position.x, position.y, position.distance());
            result = result.max(position.distance());
        }
        result
    }
}