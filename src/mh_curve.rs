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

// possible TODO: somehow hash each position so lookups of index
// based on position are faster. might actually be more overhead than lookup for each
// neighbor...
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
        // fun animations, please investigate whether this poses
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
    fn point_valid(&self, p: &Point) -> bool { 
        p.x >= 0 && p.y >= 0 && p.x < self.xsize && p.y < self.ysize // a point is in the curve if it's not negative and less than the bounds
    }
    fn is_neighbor(&self, p0: &Point, p1: &Point) -> bool {
        let xdist = (p0.x - p1.x).abs();
        let ydist = (p0.y - p1.y).abs();
        xdist + ydist == 1
    }
    pub fn is_closed(&self) -> bool {
        self.is_neighbor(&self.path[0],&self.path[self.path.len()-1])
    }
    pub fn iterate(&mut self,itercount:u32) {
        let mut rng = rand::thread_rng();
        for _ in 0..itercount {
            let endpoints = [0,self.path.len()-1];
            let i1: usize = *endpoints.iter().choose(&mut rng).unwrap(); // choose which endpoint to use
            let p1 = &self.path[i1]; // get position of endpoint

            // construct vec of neighbors and prune to valid ones
            let mut neighbors = vec![
                Point{x: p1.x+1,y: p1.y},
                Point{x: p1.x-1, y: p1.y},
                Point{x: p1.x,y: p1.y+1},
                Point{x: p1.x,y: p1.y-1}
            ];
            neighbors.retain(|p0| self.point_valid(p0));
            // TODO: improve efficiency by pruning out already-connected neighbor.

            let p2 = neighbors.iter().choose(&mut rng).unwrap(); // choose random neighbor position
            let i2 = self.path.iter().position(|p0| p0 == p2).unwrap(); //

            if i1 == 0 {
                self.path[i1..i2].reverse()
            } else {
                self.path[i2+1..i1+1].reverse()
            }
        }
    }
}