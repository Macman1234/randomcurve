mod mh_curve;
mod text_curve_draw;

fn main() {
    let curve = mh_curve::MansfieldCurve::new(6,6);
    for i in 0..curve.path.len() {
        print!("{} ",i);
        print!("{} ",text_curve_draw::get_curve_char(&curve,i,text_curve_draw::StyleKind::Arc));
        println!("{} {}",curve.path[i].x,curve.path[i].y)
    }
}
