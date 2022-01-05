use shared;
use std::collections::HashMap;









fn main() {
   // let lines = shared::get_lines("./input.txt");
    //println!("{}", lines.len());
    println!("Number of points where 2 or more lines overlap: {}", count_overlapping_points(count_points_hit(parse_vert_and_horiz_lines("./input.txt"))));
    //let line = Line {
    //    start: Point {
    //        x: 12,
    //        y: 523
    //    },
    //    end: Point {
    //        x: 2,
    //        y: 524
    //    }
    //};
    //let ps = find_points_in_line(line);
    //for p in ps {
    //    println!("({}, {})", p.x, p.y);
    //}
    
}

#[derive(Hash)]
#[derive(Eq)]
struct Point {
    x: i32,
    y: i32
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y 
    }
}




struct Line {
    start: Point,
    end: Point
}

//return Vec<Line> or something? [Line1, Line2, ...]
fn parse_vert_and_horiz_lines(filename: &str) -> Vec<Line> {
    let file_lines = shared::get_lines(filename);
    let mut line_vec: Vec<Line> = Vec::new();
    for file_line in file_lines.iter() {
        let str_points: Vec<&str> = file_line.split("->").map(|s| s.trim()).map(|s| s.trim_end()).collect();
        
        //println!("{} {} {}", str_points[0], str_points[1], str_points.len());
        let p1: Vec<i32> = str_points[0].split(",").map(|n| n.parse::<i32>().unwrap()).collect();
        let p2: Vec<i32> = str_points[1].split(",").map(|n| n.parse::<i32>().unwrap()).collect();
        
        let point1 = Point {
            x: p1[0],
            y: p1[1]
        };
        let point2 = Point {
            x: p2[0],
            y: p2[1]
        };
        let line = Line{
            start: point1,
            end: point2
        };
        if is_horiz_or_vert(&line) {
            line_vec.push(line);
        }
    }
    line_vec
}


fn is_horiz_or_vert(line: &Line) -> bool {
    is_horiz(line) || is_vert(line)
}

fn is_horiz(line: &Line) -> bool {
    line.start.y == line.end.y
}


fn is_vert(line: &Line) -> bool {
    line.start.x == line.end.x
}

fn find_points_in_line(line: &Line) -> Vec<Point> {
    let mut line_points: Vec<Point> = Vec::new();
    if is_horiz(&line) {
        // loop through x
        let y = line.start.y;
        let mut range = vec![line.start.x, line.end.x];
        range.sort();
        for x in range[0]..(range[1] + 1) {
            line_points.push(Point {
                x: x,
                y: y
            });
        }
    }
    else if is_vert(&line) {
        // loop through y
        let x = line.start.x;
        let mut range = vec![line.start.y, line.end.y];
        range.sort();
        for y in range[0]..(range[1] + 1) {
            line_points.push(Point {
                x: x,
                y: y
            });
        }
    }
    line_points
}

fn count_points_hit(lines: Vec<Line>) -> HashMap<Point, i32> {
    let mut points_hit_counter = HashMap::new();
    for line in lines.iter() {
        for point in find_points_in_line(line) {
                *points_hit_counter.entry(point).or_insert(0) += 1;
        }
    }
    //println!("{}", points_hit_counter.len());
    //for (k, v) in &points_hit_counter {
    //    println!("({}, {}): {}", k.x, k.y, v);
    //}
    points_hit_counter
}

fn count_overlapping_points(points_count: HashMap<Point, i32>) -> i32 {
    let mut cnt = 0;
    for (_point, hit_num) in points_count {
        if hit_num >= 2 {
            cnt += 1;
        }
    }
    cnt
}
