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
        } else if seg.is_vertical() {
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
        } else {
            // handle diags
            println!("diag line {}", seg);

            // up to the right
            if seg.x2 > seg.x1 && seg.y2 < seg.y1 {
                // println!("up right");
                let mut x = seg.x1 as usize;
                let mut y = seg.y1 as usize;
                let x2 = seg.x2 as usize;
                while x <= x2 {
                    grid[y][x] += 1;
                    x += 1;
                    y -= 1;
                }
            }
            if seg.x2 > seg.x1 && seg.y2 > seg.y1 {
                // println!("down right");
                let mut x = seg.x1 as usize;
                let mut y = seg.y1 as usize;
                let x2 = seg.x2 as usize;
                while x <= x2 {
                    // println!("INCR ({},{})", x, y);
                    grid[y][x] += 1;
                    x += 1;
                    y += 1;
                }
            }
            if seg.x2 < seg.x1 && seg.y2 < seg.y1 {
                // println!("up left");
                let mut x = seg.x1 as usize;
                let mut y = seg.y1 as usize;
                let x2 = seg.x2 as usize;
                while x >= x2 {
                    // println!("INCR ({},{})", x, y);
                    grid[y][x] += 1;
                    if x == 0 || y == 0 {
                        break;
                    }
                    x -= 1;
                    y -= 1;
                }
            }
            if seg.x2 < seg.x1 && seg.y2 > seg.y1 {
                // println!("down left");
                let mut x = seg.x1 as usize;
                let mut y = seg.y1 as usize;
                let x2 = seg.x2 as usize;
                while x >= x2 {
                    // println!("{} >= {}", x, y);
                    // println!("INCR ({},{})", x, y);
                    grid[y][x] += 1;
                    y += 1;
                    if x == 0 {
                        break;
                    }
                    x -= 1;
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

// adapted from https://stackoverflow.com/a/1968345
fn check_intersect(s1: &Segment, s2: &Segment) -> bool {
    let s1_x = s1.x2 - s1.x1;
    let s1_y = s1.y2 - s1.y1;
    let s2_x = s2.x2 - s2.x1;
    let s2_y = s2.y2 - s2.y1;

    let s_div = -s2_x * s1_y + s1_x * s2_y;
    if s_div == 0 {
        return false;
    }
    let s = (-s1_y * (s1.x1 - s2.x1) + s1_x * (s1.y1 - s2.y1)) / s_div;
    let t = (s2_x * (s1.y1 - s2.y1) - s2_y * (s1.x1 - s2.x1)) / (-s2_x * s1_y + s1_x * s2_y);

    if s >= 0 && s <= 1 && t >= 0 && t <= 1 {
        // Collision detected
        let x = s1.x1 + (t * s1_x);
        let y = s1.y1 + (t * s1_y);
        // println!("intersect at ({}, {})", x, y);
        return true;
    }

    false
}
