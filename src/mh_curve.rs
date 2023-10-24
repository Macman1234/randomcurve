pub struct Point {
    pub x: usize,
    pub y: usize
}

pub struct MansfieldCurve {
    pub xsize: usize,
    pub ysize: usize,
    pub path: Vec<Point>,
}

fn OnEdge(p:Point,xsize:usize,ysize:usize) {
    if p.x == 0 || p.y == 0 || p.x == xsize || p.y == ysize
}

impl MansfieldCurve {
    pub fn new(xsize: usize, ysize: usize) -> MansfieldCurve{
        let mut curve = MansfieldCurve {
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

enum StyleKind {
    thin,
    arc,
    bold,
}

fn get_curve_char(curve: MansfieldCurve, idx: usize, style: StyleKind) {
    
}