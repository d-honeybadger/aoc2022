#[path = "../lib.rs"] mod util;
use std::env;
use crate::util::util::read_input;
use std::collections::HashSet;


fn main() {
    let args: Vec<String> = env::args().collect();
    let (lines, part)= read_input(&args);

    match &*part {
        "1" => {
            let result = both_parts(&lines, 2);
            println!("result: {:?}\n", result); 
        }
        "2" => {
            let result = both_parts(&lines, 10);
            println!("result: {:?}\n", result); 
        }
        other => {
            eprintln!("no such part {}", other);
            std::process::exit(1)
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
struct Point(i32, i32);

#[derive(Debug)]
struct State {
    knots: Vec<Point>,
}

impl State {
    fn make_move(&mut self, direction: &str) {
        match direction {
            "R" => {
                self.knots[0] = Point(self.knots[0].0 + 1, self.knots[0].1);
            }
            "L" => {
                self.knots[0] = Point(self.knots[0].0 - 1, self.knots[0].1);
            }
            "U" => {
                self.knots[0] = Point(self.knots[0].0, self.knots[0].1 + 1);
            }
            "D" => {
                self.knots[0] = Point(self.knots[0].0, self.knots[0].1 - 1);
            }
            _ => {
                panic!("unexpected direction {}", direction);
            }
        }

        for i in 1..self.knots.len() {
            if self.knots[i].0 == self.knots[i-1].0 {
                // same column
                if (self.knots[i].1 - self.knots[i-1].1).abs() > 1 {
                    self.knots[i] = Point(self.knots[i].0, self.knots[i].1 + (self.knots[i-1].1 - self.knots[i].1).signum());
                }
            } else if self.knots[i].1 == self.knots[i-1].1 {
                // same row
                if (self.knots[i].0 - self.knots[i-1].0).abs() > 1 {
                    self.knots[i] = Point(self.knots[i].0 + (self.knots[i-1].0 - self.knots[i].0).signum(), self.knots[i].1);
                }
            } else {
                // diagonal
                if (self.knots[i].0 - self.knots[i-1].0).abs() > 1 || (self.knots[i].1 - self.knots[i-1].1).abs() > 1 {
                    self.knots[i] = Point(self.knots[i].0 + (self.knots[i-1].0 - self.knots[i].0).signum(), self.knots[i].1 + (self.knots[i-1].1 - self.knots[i].1).signum());
                }
            }
        }
    }
}

fn both_parts(lines: &Vec<String>, num_knots: usize) -> i32 {
    let mut visited: HashSet<Point> = HashSet::new();

    let mut s = State{
        knots: Vec::new(),
    };
    for _ in 0..num_knots {
        s.knots.push(Point(0,0))
    }
    visited.insert(s.knots[num_knots - 1].clone());

    for line in lines {
        let move_components: Vec<&str> = line.split(" ").collect();
        if move_components.len() != 2 {
            panic!("unexpected move {}", line);
        }

        let direction = move_components[0];
        let num_steps = move_components[1].parse().unwrap();

        for _ in 0..num_steps {
            s.make_move(direction);
            // println!("{:?}", s);
            visited.insert(s.knots[num_knots - 1].clone());
        }
    }

    return visited.len() as i32;
}
