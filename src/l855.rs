use std::ops::Div;

pub struct ExamRoom {
    capacity: i32,
    pub occupied: Vec<i32>,
}

impl ExamRoom {

    pub fn new(n: i32) -> Self {
        Self {
            capacity: n,
            occupied: Vec::new(),
        }
    }

    fn seat_and_return(&mut self, seat: i32) -> i32 {
        self.occupied.push(seat);
        self.occupied.sort_unstable();
        seat
    }

    pub fn seat(&mut self) -> i32 {
        return match self.occupied.len() {
            0 => self.seat_and_return(0),
            1 => {
                if self.occupied[0] <= (self.capacity - 1).div(2) {
                    self.seat_and_return(self.capacity - 1)
                } else {
                    self.seat_and_return(0)
                }
            }
            _ => {
                let mut min_distance = i32::MIN;
                let mut target_p = self.capacity;
                for pair in self.occupied.windows(2) {
                    let p1 = pair[0];
                    let p2 = pair[1];
                    let p = (p1 + p2).div(2);
                    if p > p1 && p - p1 > min_distance {
                        min_distance = p - p1;
                        target_p = p;
                    }
                }
                if !self.occupied.contains(&0) {
                    if self.occupied[0] >= min_distance {
                        target_p = 0;
                    }
                } else if !self.occupied.contains(&(self.capacity-1)) {
                    if self.capacity - 1 - self.occupied.last().unwrap() > min_distance {
                        target_p = self.capacity - 1;
                    }
                }
                self.seat_and_return(target_p)
            }
        };
    }

    pub fn leave(&mut self, p: i32) {
        self.occupied.retain(|&i| i != p )
    }
}
