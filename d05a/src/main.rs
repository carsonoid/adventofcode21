use std::env;
use std::fmt;
use std::fs;

#[derive(Clone, PartialEq, Copy, Debug)]
struct Segment {
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
}

impl fmt::Display for Segment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Customize so only `x` and `y` are denoted.
        write!(f, "{},{} -> {},{}", self.x1, self.y1, self.x2, self.y2)
    }
}

impl Segment {
    pub fn parse(s: &str) -> Segment {
        let parts: Vec<&str> = s.split(" -> ").collect();
        let p1: Vec<i32> = parts[0]
            .split(",")
            .map(|x| x.parse().expect("must be int"))
            .collect();
        let p2: Vec<i32> = parts[1]
            .split(",")
            .map(|x| x.parse().expect("must be int"))
            .collect();

        Segment {
            x1: p1[0],
            y1: p1[1],
            x2: p2[0],
            y2: p2[1],
        }
    }
    pub fn is_vertical(&self) -> bool {
        self.x1 == self.x2
    }
    pub fn is_horizontal(&self) -> bool {
        self.y1 == self.y2
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("filename arg is required");
        return;
    }

    let filename = &args[1];

    println!("Reading file: {}", filename);

    let contents = fs::read_to_string(filename).expect("error reading file");

    let segments: Vec<Segment> = contents
        .split("\n")
        .filter(|&s| !s.is_empty())
        .map(Segment::parse)
        .collect();
    println!("{:?}", segments);

    let mut grid: Vec<Vec<i32>> = vec![vec![0; 1000]; 1000];
    for seg in segments {
        if seg.is_horizontal() {
            let y = seg.y1 as usize;
            if seg.x1 <= seg.x2 {
                let mut x = seg.x1 as usize;
                let x2 = seg.x2 as usize;
                while x <= x2 {
                    grid[y][x] += 1;
                    x += 1;
                }
            } else {
                let mut x = seg.x1 as usize;
                let x2 = seg.x2 as usize;
                while x >= x2 {
                    grid[y][x] += 1;
                    x -= 1;
                }
            }
        }

        if seg.is_vertical() {
            let x = seg.x1 as usize;
            if seg.y1 <= seg.y2 {
                let mut y = seg.y1 as usize;
                let y2 = seg.y2 as usize;
                while y <= y2 {
                    grid[y][x] += 1;
                    y += 1;
                }
            } else {
                let mut y = seg.y1 as usize;
                let y2 = seg.y2 as usize;
                while y >= y2 {
                    grid[y][x] += 1;
                    y -= 1;
                }
            }
        }
    }

    let mut count = 0;
    for (y, row) in grid.iter().enumerate() {
        println!("{:?}", row);
        for elem in row {
            if *elem >= 2 {
                count += 1;
            }
        }
    }

    println!("intersections: {}", count);
}
