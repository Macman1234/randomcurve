mod mh_curve;
mod text_curve_draw;
use text_curve_draw::{make_curve_string,StyleKind};
use crate::csv_curve_print::make_curve_csv_string;
mod csv_curve_print;
use crate::openscad_curve_generate::make_curve_openscad_file;
mod openscad_curve_generate;

use term_size;
use clap::{Parser, ValueEnum};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about=None)]
struct Args {
    /// dimensions of the curve
    #[arg(short, value_parser, value_delimiter = ',', default_values_t = vec![16,16])]
    dimensions: Vec<usize>,

    /// Number of Monte Carlo iterations to randomize the curve
    #[arg(short, long, default_value_t = 10000)]
    itercount: u32,

    /// Turns on wait till closed mode
    #[arg(short,long,action)]
    closed: bool,

    /// choose display output mode
    #[arg(value_enum, short, long, default_value_t = DisplayMode::Terminal)]
    mode: DisplayMode,

    /// Style of lines to draw in terminal mode
    #[arg(value_enum, short, long, default_value_t = StyleKind::Arc)]
    style: StyleKind,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum DisplayMode {
    /// One-shot as utf-8 box chars to the terminal
    Terminal,
    /// animated terminal utf-8 box chars
    TermAnim,
    /// CSV to stdout
    CSV,
    /// openSCAD file to stdout
    OpenSCAD,
}

fn main() {
    let args = Args::parse();

    match args.mode {
        DisplayMode::Terminal => {
            if args.dimensions.len() != 2 {panic!("only 2d for terminal")}
            let mut curve = mh_curve::MansfieldCurve::new(args.dimensions);
            curve.iterate(args.itercount);
            if args.closed { // if we want a closed curve
                while !curve.is_closed(){ // iterate it until the ends are neighbors
                    curve.iterate(1);
                }
            }
            println!("{}",make_curve_string(&curve, &args.style))
        },
        DisplayMode::TermAnim => {
            if args.dimensions.len() != 2 {panic!("only 2d for anim")}
            let mut curve = mh_curve::MansfieldCurve::new(args.dimensions); // initialize empty curve
            loop {
                if let Some((w, h)) = term_size::dimensions() { // get size of terminal
                    let xsize = w; // TODO: look into making smaller uint
                    let ysize = h;
                    if (curve.size[0],curve.size[1]) != (xsize,ysize) { // if it's changed from when the curve was last generated
                        curve = mh_curve::MansfieldCurve::new(vec![xsize,ysize]); // new curve
                    }
                    // TODO: make this not hardcoded
                    // TODO: maybe make this dependent on time rather than per-screen write? currently the rate of how fast it 
                    // can iterate the curve as well as how fast it can write to the screen. as such, iterations per display will
                    // show up as different rates depending on both CPU speed and terminal writeout speed. basing it on real time measurements
                    // would avoid this but would be harder to implement.
                    curve.iterate(10); // iterate 
                    print!("{}",make_curve_string(&curve, &args.style)); // generate string of curve and print it
                    for _ in 0..curve.size[1]-1 { // move cursor to top of generated curve so it can be overwritten
                        print!("\x1b[F"); // TODO : maybe implement this some other way?
                    }
                }
                else {
                    panic!("Unable to get term size");
                }  
            }
        },
        DisplayMode::CSV => {
            // CSV can be any dimension
            let mut curve = mh_curve::MansfieldCurve::new(args.dimensions);
            curve.iterate(args.itercount);
            if args.closed { // if we want a closed curve
                while !curve.is_closed(){ // iterate it until the ends are neighbors
                    curve.iterate(1);
                }
            }
            println!("{}",make_curve_csv_string(&curve))
        },
        DisplayMode::OpenSCAD => {
            if args.dimensions.len() != 2 && args.dimensions.len() != 3 {panic!("only 2d or 3d for openscad")}
            let mut curve = mh_curve::MansfieldCurve::new(args.dimensions);
            curve.iterate(args.itercount);
            if args.closed { // if we want a closed curve
                while !curve.is_closed(){ // iterate it until the ends are neighbors
                    curve.iterate(1);
                }
            }
            println!("{}",make_curve_openscad_file(&curve))
        },
    }
}