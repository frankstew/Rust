use shared;
use std::collections::HashSet;
use std::fmt;

fn main() {
    let mut grid = Grid {
        points: parse_input("./input.txt"),
        ..Default::default()
    };
    grid.find_basins();
    let sum = grid.get_largest_basins_prod();
    //println!("Three largest basins are: ");
    //println!("{:?}", large_basins[0].points);
    //println!("{:?}", large_basins[1].points);
    //println!("{:?}", large_basins[2].points);
    //println!("with a prod of sizes equal to {}", sum);
    println!("Three largest basins have a product of sizes equal to: {}", sum);

}

//storing grid as ["153534920587493", "185839432038429", ....], index by
//grid[i].chars().nth(j).unwrap()
fn parse_input(filename: &str) -> Vec<String> {
    shared::get_lines(filename)
}

#[derive(Hash, Debug)]
struct Point {
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

struct Basin {
    points: Vec<Point>
}

impl Basin {
    fn new() -> Basin {
        Basin {
            points: Vec::new()
        }
    }
}

impl Default for Basin {
    fn default() -> Basin {
        Basin {
            points: Vec::new()
        }
    }
}

struct Grid {
    points: Vec<String>,
    minima: Vec<Point>,
    basins: Vec<Basin>
}

impl Default for Grid {
    fn default() -> Grid {
        Grid {
            points: Vec::new(),
            minima: Vec::new(),
            basins: Vec::new()
        }
    }
}

fn find_min_index(basins: &mut Vec<&Basin>, b: &Basin) -> usize {
    let mut min_index = 0;
    let mut min = basins[0].points.len();
    for index in 0..basins.len() {
        if basins[index].points.len() < min {
            min_index = index;
            min = basins[index].points.len();
        }
    }
    min_index
}

impl Grid {
    fn get_point(&self, row: usize, col: usize) -> i32 {
        let base: u32 = 10;
        self.points[row].chars().nth(col).unwrap().to_digit(base).unwrap() as i32
    }

    fn get_largest_basins_prod(&self) -> usize {
        let b1 = Basin::new();
        let b2 = Basin::new();
        let b3 = Basin::new();
        let mut large_basins = vec![&b1, &b2, &b3];
        for b in &self.basins {
            let i = find_min_index(&mut large_basins, &b);
            if b.points.len() > large_basins[i].points.len() {
                large_basins[i] = b;
            }
        }
        return large_basins[0].points.len() * large_basins[1].points.len() * large_basins[2].points.len();
    }

    fn find_basins(&mut self) {
        let mut prev_searched_points: HashSet<Point> = HashSet::new(); 
        for row in 0..self.points.len() {
            for col in 0..self.points[row].len() {
                
                let p = Point { x: row, y: col }; 
                if self.get_point(p.x, p.y) == 9 || prev_searched_points.contains(&p){
                    continue;
                }
                let basin = self.map_basin(row, col, &mut prev_searched_points);
                //println!("{}", basin.points.len());
                self.basins.push(basin);        
            }
        }
    }
    

    fn map_basin(&self, row: usize, col: usize, prev_searched_points: &mut HashSet<Point>) -> Basin {
        let mut basin = Basin::new(); 
        let mut pts_to_search: Vec<Point> = vec![Point {x: row, y: col }];
        while pts_to_search.len() > 0 {
            let point = pts_to_search.pop().unwrap();
            if self.get_point(point.x, point.y) == 9 || prev_searched_points.contains(&point) {
                continue;
            }
            basin.points.push(point.clone());
            prev_searched_points.insert(point.clone());
            let adj = self.get_adjacent_points(point.x, point.y);
            for p in adj {
                // skip if value is 9 or already been looked at
                if self.get_point(p.x, p.y) == 9 || prev_searched_points.contains(&p) {
                    continue;
                }
                pts_to_search.push(p); 
            }
        }
        basin
    }

    fn get_adjacent_points(&self, row: usize, col: usize) -> Vec<Point> {
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
                // bottom right 
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

    fn is_minimum(&self, row: usize, col: usize) -> bool {
        let height = self.get_point(row, col);
        let mut is_min = true;
        for p in self.get_adjacent_points(row, col) {
            is_min = is_min && height < self.get_point(p.x, p.y);
        }
        is_min
    }

    fn find_minima(&mut self) {
        let mut minima = Vec::new();
        for row in 0..self.points.len() {
            for col in 0..self.points[row].len() {
                if self.is_minimum(row, col) {
                    minima.push(Point {
                        x: row,
                        y: col
                    });
                }
            }
        }
        self.minima = minima;
    }

    fn calc_risk(&self) -> i32 {
        let mut risk = 0;
        for p in &self.minima {
            if self.is_minimum(p.x, p.y) {
                risk += self.get_point(p.x, p.y) + 1; 
            }
        }
        risk
    }
}
