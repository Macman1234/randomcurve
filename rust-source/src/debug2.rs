mod mh_curve;
mod text_curve_draw;
use text_curve_draw::{make_curve_string,StyleKind};
fn main() {
    let mut curve = mh_curve::MansfieldCurve::new(9,9);
    
    for i in 0..curve.path.len() {
        print!("{} ",i);
        println!("{}",curve.path[i])
    }
    
    println!("{}",make_curve_string(&curve, &StyleKind::Arc));
    curve.iterate(100);
    for i in 0..curve.path.len() {
        print!("{} ",i);
        println!("{}",curve.path[i])
    }
    println!("{}",make_curve_string(&curve, &StyleKind::Arc));
}
