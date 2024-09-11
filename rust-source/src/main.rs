mod mh_curve;
mod text_curve_draw;
use text_curve_draw::{make_curve_string,StyleKind};

use term_size;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about=None)]
struct Args {
    /// Horizontal size of the curve
    #[arg(short, default_value_t = 16)]
    xsize: i32,

    /// Vertical size of the curve
    #[arg(short, default_value_t = 16)]
    ysize: i32,

    /// Number of Monte Carlo iterations to randomize the curve
    #[arg(short, long, default_value_t = 10000)]
    itercount: u32,

    /// Style of lines to draw
    #[arg(value_enum,short, long, default_value_t = StyleKind::Arc)]
    style: StyleKind,

    /// Turns on terminal animation mode
    #[arg(short,long,action)]
    animation: bool,

}

fn main() {

    let args = Args::parse();

    // If animation mode is on, continuously iterate curve and display to terminal.
    // Generate new curve if terminal size is changed
    if args.animation{
        let mut curve = mh_curve::MansfieldCurve::new(0,0); // initialize empty curve
        loop {
            if let Some((w, h)) = term_size::dimensions() { // get size of terminal
                let xsize: i32 = w.try_into().unwrap(); // cast to i32 TODO: make xsize type usize
                let ysize: i32 = h.try_into().unwrap();
                if (curve.xsize,curve.ysize) != (xsize,ysize) { // if it's changed from when the curve was last generated
                    curve = mh_curve::MansfieldCurve::new(xsize,ysize); // new curve
                }
                // TODO: make this not hardcoded
                // TODO: maybe make this dependent on time rather than per-screen write? currently the rate of how fast it 
                // can iterate the curve as well as how fast it can write to the screen. as such, iterations per display will
                // show up as different rates depending on both CPU speed and terminal writeout speed. basing it on real time measurements
                // would avoid this but would be harder to implement.
                curve.iterate(10); // iterate 
                print!("{}",make_curve_string(&curve, &args.style)); // generate string of curve and print it
                for _ in 0..curve.ysize-1 { // move cursor to top of generated curve so it can be overwritten
                    print!("\x1b[F"); // TODO : maybe implement this some other way?
                }
            }
            else {
                panic!("Unable to get term size");
            }  
        }
    }

    // if not in animation mode, just generate a curve, iterate it by itercount and print it
    else {
        let mut curve = mh_curve::MansfieldCurve::new(args.xsize,args.ysize);
        curve.iterate(args.itercount);

        println!("{}",make_curve_string(&curve, &args.style));
    }
}