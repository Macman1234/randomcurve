mod mh_curve;
mod text_curve_draw;
use text_curve_draw::{make_curve_string,StyleKind};

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about=None)]
struct Args {
    /// Horizontal size of the curve
    #[arg(short, default_value_t = 32)]
    xsize: i32,

    /// Vertical size of the curve
    #[arg(short, default_value_t = 32)]
    ysize: i32,

    /// Number of Monte Carlo iterations to randomize the curve
    #[arg(short, long, default_value_t = 100000)]
    itercount: u32,

    /// Style of lines to draw in CLI mode
    #[arg(value_enum,short, long, default_value_t = StyleKind::Arc)]
    style: StyleKind,

}

fn main() {

    let args = Args::parse();

    let mut curve = mh_curve::MansfieldCurve::new(args.xsize,args.ysize);

    curve.iterate(args.itercount);

    println!("{}",make_curve_string(&curve, &args.style));
}