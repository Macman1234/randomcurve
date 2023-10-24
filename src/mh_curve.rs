pub struct Point {
    pub x: usize,
    pub y: usize
}

pub struct Mansfieldcurve {
    pub xsize: usize,
    pub ysize: usize,
    pub path: Vec<Point>,
}

impl Mansfieldcurve {
    pub fn new(xsize: usize, ysize: usize) -> Mansfieldcurve{
        let mut curve = Mansfieldcurve {
            xsize,
            ysize,
            path: Vec::new()
        };
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
}