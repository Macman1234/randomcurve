use rand::seq::IteratorRandom;
use std::fmt;

pub struct Point {
    pub x: i32,
    pub y: i32
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,"({},{})", self.x, self.y)
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

pub struct MansfieldCurve {
    pub xsize: i32,
    pub ysize: i32,
    pub path: Vec<Point>,
}
/*
enum Edge {
    Top,
    TopRight,
    Right,
    BottomRight,
    Bottom,
    BottomLeft,
    Left,
    TopLeft,
    None
}

fn check_edge(p: Point, xsize: i32, ysize: i32) -> Edge {
    if p.x == 0 {
        if p.y == 0 {Edge::TopLeft}
        else if p.y == ysize {Edge::BottomLeft}
        else {Edge::Left}
    }
    else if p.x == xsize {
        if p.y == 0 {Edge::TopRight}
        else if p.y == ysize {Edge::BottomRight}
        else {Edge::Right}
    }
    else {Edge::None}
}
*/

impl MansfieldCurve {
    pub fn new(xsize: i32, ysize: i32) -> MansfieldCurve{

        // Construct empty struct
        let mut curve = MansfieldCurve {
            xsize,
            ysize,
            path: Vec::new()
        };

        // Fill path with snaking pattern
        // If using this for something more rigorous than
        // fun animations, investigate whether this poses
        // any problems with density/energy
        for n in 0..(xsize*ysize) {
            let y = n / xsize;
            let x = if y % 2 == 0 {
                n % xsize
            } else {
                xsize - (n % xsize) -1
            };
            curve.path.push(Point{x,y})
        }
        curve
    }
    fn point_valid(&self,p: &Point) -> bool {
        p.x >= 0 && p.y >= 0 && p.x < self.xsize && p.y < self.ysize
    }
    pub fn iterate(&mut self,itercount:u32) {
        let mut rng = rand::thread_rng();
        for _ in 0..itercount {
            // choose endpoint
            let endpoints = [0,self.path.len()-1];
            let i1: usize = *endpoints.iter().choose(&mut rng).unwrap();
            let p1 = &self.path[i1];

            // construct vec of neighbors and prune to valid ones
            let mut neighbors = vec![Point{x: p1.x+1,y: p1.y},
            Point{x: p1.x-1, y: p1.y},Point{x: p1.x,y: p1.y+1},Point{x: p1.x,y: p1.y-1}];
            neighbors.retain(|p0| self.point_valid(p0));

            let p2 = neighbors.iter().choose(&mut rng).unwrap();
            let i2 = self.path.iter().position(|p0| p0 == p2).unwrap();

            if i1 == 0 {
                self.path[i1..i2].reverse()
            } else {
                self.path[i2+1..i1+1].reverse()
            }
        }
    }
}