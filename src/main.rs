mod mh_curve;
fn main() {
    let curve = mh_curve::Mansfieldcurve::new(6,6);
    for p in curve.path {
        println!("{},{}",p.x,p.y);
    }
}
