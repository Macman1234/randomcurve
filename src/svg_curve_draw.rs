use crate::mh_curve;
pub fn make_curve_path_file(curve: &mh_curve::MansfieldCurve, scale: i32) -> String {
    let path_string = make_curve_path(curve, scale);
    let file_string = format!("<svg height=\"{0}\" width=\"{1}\" xmlns=\"http://www.w3.org/2000/svg\">
    {path_string}
</svg>",(curve.ysize+2)*scale,(curve.ysize+2)*scale);
    file_string
}

fn make_curve_path(curve: &mh_curve::MansfieldCurve, scale: i32) -> String {
    let mut path_string: String = "<path d=\"".to_string();
    let mut first: bool = true;
    for p in curve.path.iter() {
        if first { 
            let cmd = format!("M{0} {1}", (p.x+1)*scale,(p.y+1)*scale);
            first = false;
            path_string = [path_string, cmd].join(" ");
        }
        else {
            let cmd = format!("L{0} {1}", (p.x+1)*scale,(p.y+1)*scale);
            path_string = [path_string, cmd].join(" ");
        }
    };
    if curve.is_closed() {
        let cmd = format!("L{0} {1}", (curve.path[0].x+1)*scale,(curve.path[0].y+1)*scale);
        path_string = [path_string, cmd].join(" ");
    }
    let end = format!("\" style=\"fill:none;stroke:black;stroke-width:{0};stroke-linecap:square\" />",scale/2);
    path_string = [path_string, end].join("");
    path_string
}