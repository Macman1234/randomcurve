use crate::mh_curve;
pub fn make_curve_csv_string(curve: &mh_curve::MansfieldCurve) -> String{
    let mut curve_string = String::new();
    curve_string.push_str("x,y,z,");
    for i in 0..curve.path.len() {
            for d in &curve.path[i].pos {
                curve_string.push_str(&d.to_string());
            }
            curve_string.push('\n');
        };
    curve_string
}
