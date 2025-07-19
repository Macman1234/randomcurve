use rand::seq::IteratorRandom;
use std::fmt;

pub struct Point {
    pub pos: Vec<usize>
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,"{:?}", self.pos)
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.pos == other.pos
    }
}

pub struct MansfieldCurve {
    pub size: Vec<usize>,
    pub dim: usize,
    pub path: Vec<Point>,
}

// possible TODO: somehow hash each position so lookups of index
// based on position are faster. might actually be more overhead than lookup for each
// neighbor...
impl MansfieldCurve {
    pub fn new(size: Vec<usize>) -> MansfieldCurve{

        // Construct empty struct
        let mut curve = MansfieldCurve {
            size: size.clone(), //why rust why. why can't i take the length without taking ownership
            dim: size.len(),
            path: Vec::new()
        };

        // Fill path with snaking pattern
        // If using this for something more rigorous than
        // fun animations, please investigate whether this poses
        // any problems with density/energy
        for n in 0..(size.iter().product()) {
            let mut pos: Vec<usize> = Vec::new();
            for d in 0..curve.dim {
                // i think divisor is based on product off all next dimensions???
                let nextdimprodsize = if d == curve.dim-1 { 1 } else {size[d+1..curve.dim].iter().product() };
                // direction is based on parity of previous dimension values, summed???
                let prevdimsumpos = if d == 0 { 1 } else {pos[0..d].iter().sum() }; 
                let p = if prevdimsumpos % 2 == 0 {
                    (n / nextdimprodsize) % size[d]
                } else {
                    size[d] - ((n / nextdimprodsize) % size[d]) - 1
                };
                pos.push(p);
            }

            /*let y = n / xsize;
            let x = if y % 2 == 0 {
                n % xsize
            } else {
                xsize - (n % xsize) -1
            };*/
            curve.path.push(Point{pos})
        }
        curve
    }
    pub fn is_neighbor(&self, p0: &Point, p1: &Point) -> bool {
        let distances: isize = (0..p0.pos.len()).map(|n| (p0.pos[n] as isize - p1.pos[n] as isize).abs()).sum();
        distances == 1
    }
    /*fn point_valid(&self, p: &Point) -> bool { 
        p.x >= 0 && p.y >= 0 && p.x < self.xsize && p.y < self.ysize // a point is in the curve if it's not negative and less than the bounds
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
            let i2 = self.path.iter().position(|p0| p0 == p2).unwrap(); // get index of chosen neighbor

            if i1 == 0 {
                self.path[i1..i2].reverse()
            } else {
                self.path[i2+1..i1+1].reverse()
            }
        }
    }*/
}