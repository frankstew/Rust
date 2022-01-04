use shared;









fn main() {
   // let lines = shared::get_lines("./input.txt");
    //println!("{}", lines.len());
    let p1 = Point {
        x: 20, 
        y: 30
    };
    let p2 = Point {
        x: 40,
        y: 50
    };
    let l = Line {
        start: p1,
        end: p2
    };
    println!("{}", l.end.x);
    parse_vert_and_horiz_lines("./input.txt");
    
}

struct Point {
    x: i32,
    y: i32
}

struct Line {
    start: Point,
    end: Point
}

//return Vec<Line> or something? [Line1, Line2, ...]
fn parse_vert_and_horiz_lines(filename: &str) {
    let lines = shared::get_lines(filename);
    let mut line_vec: Vec<Vec<&str>> = Vec::new();
    for line in lines.iter() {
        line_vec.push(line.split("->").collect());
    }
    println!("{}", line_vec[0][0]);
}
