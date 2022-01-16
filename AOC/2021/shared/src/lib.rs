use std::fs;
use std::string::ToString;
use std::fmt;

pub fn print_stuff() {
    println!("Stuff");
}

pub fn get_int_lines(filename: &str) -> Vec<i32> {
    let contents = fs::read_to_string(filename.to_string())
        .expect("Woops");
    // filter to remove empty lines (notr perfect)
    contents.split("\n").filter(|&elem| !is_empty_or_newline(elem)).map(|s| s.trim_end().parse::<i32>().unwrap()).collect()

    //OR 

    //let lines: Vec<String> = contents.split("\n").map(|s| s.to_string()).collect();
    // + filter
    //lines
}

pub fn get_lines(filename: &str) -> Vec<String> {
    let contents = fs::read_to_string(filename.to_string()).expect("Woops");
    contents.split("\n").filter(|&elem| !is_empty_or_newline(elem)).map(|s| s.trim_end().to_string()).collect()
}

pub fn is_empty_or_newline(elem: &str) -> bool {
    elem == "" || elem == "\n" || elem == "\r\n" || elem.as_bytes() == &[13] || elem.to_string().is_empty()
}


pub fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}



pub fn print_vec<T: ToString>(v: &Vec<T>) {
    let mut vec_str = String::from("[");
    for i in v.iter() {
        let mut append_str = i.to_string();
        append_str.push_str(", ");
        vec_str.push_str(&append_str);
    }
    vec_str = vec_str[0..vec_str.len() - 2].to_string();
    vec_str.push_str("]");
    println!("{}", vec_str);
}


#[derive(Hash, Debug)]
pub struct Point {
    x: usize,
    y: usize
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self. x == other.x && self.y == other.y
    }
}

impl Eq for Point {}

impl Point {
    fn clone(&self) -> Point {
        Point { x: self.x, y: self.y }
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

pub struct Grid {
    pub points: Vec<Vec<i32>>,
}

impl Default for Grid {
    fn default() -> Grid {
        Grid {
            points: Vec::new(),
        }
    }
}

impl Grid {
    pub fn get_point_val(&self, row: usize, col: usize) -> i32 {
        self.points[row][col]
    }

    pub fn get_adjacent_points_inc_diag(&self, row: usize, col: usize) -> Vec<Point> {
        let mut adj_pts = Vec::new();
        if row == 0 {
            // top
            if col == 0 {
                // top left 
                adj_pts.push(Point { x: row, y: col + 1 });
                adj_pts.push(Point { x: row + 1, y: col });
                adj_pts.push(Point { x: row + 1, y: col + 1 });
            } else if col == self.points[0].len() - 1 {
                // top right
                adj_pts.push(Point { x: row, y: col - 1 });
                adj_pts.push(Point { x: row + 1, y: col });
                adj_pts.push(Point { x: row + 1, y: col - 1 });
            } else {
                // top middle
                adj_pts.push(Point { x: row, y: col + 1 });
                adj_pts.push(Point { x: row, y: col - 1 });
                adj_pts.push(Point { x: row + 1, y: col });
                adj_pts.push(Point { x: row + 1, y: col - 1 });
                adj_pts.push(Point { x: row + 1, y: col + 1 });
            }
        } else if row == self.points.len() - 1 {
            // bottom
            if col == 0 {
                // bottom left 
                adj_pts.push(Point { x: row, y: col + 1 });
                adj_pts.push(Point { x: row - 1, y: col });
                adj_pts.push(Point { x: row - 1, y: col + 1 });
            } else if col == self.points[0].len() - 1 {
                // bottom right
                adj_pts.push(Point { x: row, y: col - 1 });
                adj_pts.push(Point { x: row - 1, y: col });
                adj_pts.push(Point { x: row - 1, y: col - 1 });
            } else {
                // bottom middle
                adj_pts.push(Point { x: row, y: col - 1 });
                adj_pts.push(Point { x: row, y: col + 1 });
                adj_pts.push(Point { x: row - 1, y: col });
                adj_pts.push(Point { x: row - 1, y: col - 1 });
                adj_pts.push(Point { x: row - 1, y: col + 1 });
            }
        } else {
            // middle  
            if col == 0 {
                // middle left 
                adj_pts.push(Point { x: row, y: col + 1 });
                adj_pts.push(Point { x: row + 1, y: col });
                adj_pts.push(Point { x: row - 1, y: col });
                adj_pts.push(Point { x: row - 1, y: col + 1 });
                adj_pts.push(Point { x: row + 1, y: col + 1 });
            } else if col == self.points[0].len() - 1 {
                // middle right
                adj_pts.push(Point { x: row, y: col - 1 });
                adj_pts.push(Point { x: row + 1, y: col });
                adj_pts.push(Point { x: row - 1, y: col });
                adj_pts.push(Point { x: row - 1, y: col - 1 });
                adj_pts.push(Point { x: row + 1, y: col - 1 });
            } else {
                // true middle
                adj_pts.push(Point { x: row, y: col + 1 });
                adj_pts.push(Point { x: row, y: col - 1 });
                adj_pts.push(Point { x: row - 1, y: col });
                adj_pts.push(Point { x: row + 1, y: col });
                adj_pts.push(Point { x: row + 1, y: col + 1 });
                adj_pts.push(Point { x: row + 1, y: col - 1 });
                adj_pts.push(Point { x: row - 1, y: col + 1 });
                adj_pts.push(Point { x: row - 1, y: col - 1 });
            }
        }
        adj_pts
    }

    pub fn get_adjacent_points(&self, row: usize, col: usize) -> Vec<Point> {
        let mut adj_pts = Vec::new();
        if row == 0 {
            // top
            if col == 0 {
                // top right 
                adj_pts.push(Point { x: row, y: col + 1 });
                adj_pts.push(Point { x: row + 1, y: col });
            } else if col == self.points[0].len() - 1 {
                // top right
                adj_pts.push(Point { x: row, y: col - 1 });
                adj_pts.push(Point { x: row + 1, y: col });
            } else {
                // top middle
                adj_pts.push(Point { x: row, y: col + 1 });
                adj_pts.push(Point { x: row, y: col - 1 });
                adj_pts.push(Point { x: row + 1, y: col });
            }
        } else if row == self.points.len() - 1 {
            // bottom
            if col == 0 {
                // bottom left 
                adj_pts.push(Point { x: row, y: col + 1 });
                adj_pts.push(Point { x: row - 1, y: col });
            } else if col == self.points[0].len() - 1 {
                // bottom right
                adj_pts.push(Point { x: row, y: col - 1 });
                adj_pts.push(Point { x: row - 1, y: col });
            } else {
                // bottom middle
                adj_pts.push(Point { x: row, y: col - 1 });
                adj_pts.push(Point { x: row, y: col + 1 });
                adj_pts.push(Point { x: row - 1, y: col });
            }
        } else {
            // middle  
            if col == 0 {
                // middle left 
                adj_pts.push(Point { x: row, y: col + 1 });
                adj_pts.push(Point { x: row + 1, y: col });
                adj_pts.push(Point { x: row - 1, y: col });
            } else if col == self.points[0].len() - 1 {
                // middle right
                adj_pts.push(Point { x: row, y: col - 1 });
                adj_pts.push(Point { x: row + 1, y: col });
                adj_pts.push(Point { x: row - 1, y: col });
            } else {
                // true middle
                adj_pts.push(Point { x: row, y: col + 1 });
                adj_pts.push(Point { x: row, y: col - 1 });
                adj_pts.push(Point { x: row - 1, y: col });
                adj_pts.push(Point { x: row + 1, y: col });
            }
        }
        adj_pts
    }
}
