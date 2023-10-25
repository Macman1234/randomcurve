mod mh_curve;

mod text_curve_draw;
use text_curve_draw::{make_curve_string,StyleKind};

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut curve = mh_curve::MansfieldCurve::new(32,32);

    curve.iterate(1000000);

    println!("{}",make_curve_string(&curve, &StyleKind::Arc));
}