use crate::mh_curve;
pub fn print_curve_debug_string(curve: &mh_curve::MansfieldCurve) {
    for i in 0..curve.path.len() {
            print!("{} ",i);
            if i > 0 {
                print!{"{} ", curve.is_neighbor(&curve.path[i],&curve.path[i-1])};
            }
            println!("{}", curve.path[i]);
        }
}
