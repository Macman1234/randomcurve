use crate::mh_curve;
use clap::ValueEnum;

#[derive(Copy, Clone, Debug,PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum StyleKind {
    Light,
    Heavy,
    Arc,
}

pub fn get_curve_char(curve: &mh_curve::MansfieldCurve, idx: usize, style: &StyleKind) -> char {
    let current_point = &curve.path[idx];
    let mut x_diff_after = 0;
    let mut y_diff_after = 0;
    let mut x_diff_before = 0;
    let mut y_diff_before = 0;
    if idx != curve.path.len() - 1 {
        let later_neighbor = &curve.path[idx + 1];
        x_diff_after = current_point.pos[0] as i32 - later_neighbor.pos[0] as i32;
        y_diff_after = current_point.pos[1] as i32 - later_neighbor.pos[1] as i32;
    }
    else if curve.is_closed() {
        let later_neighbor = &curve.path[0];
        x_diff_after = current_point.pos[0] as i32 - later_neighbor.pos[0] as i32;
        y_diff_after = current_point.pos[1] as i32 - later_neighbor.pos[1] as i32;
    }
    if idx != 0 {
        let before_neighbor = &curve.path[idx - 1];
        x_diff_before = current_point.pos[0] as i32 - before_neighbor.pos[0] as i32;
        y_diff_before = current_point.pos[1] as i32 - before_neighbor.pos[1] as i32;
    }
    else if curve.is_closed() {
        let before_neighbor = &curve.path[curve.path.len() - 1];
        x_diff_before = current_point.pos[0] as i32 - before_neighbor.pos[0] as i32;
        y_diff_before = current_point.pos[1] as i32 - before_neighbor.pos[1] as i32;
    }
    
    match (x_diff_after, y_diff_after, x_diff_before, y_diff_before) {
        (1, 0, 0, -1) | (0, -1, 1, 0) => match style {
            StyleKind::Light => '┐',
            StyleKind::Arc => '╮',
            StyleKind::Heavy => '┓',
        },
        (1, 0, 0, 1) | (0, 1, 1, 0) => match style {
            StyleKind::Light => '┘',
            StyleKind::Arc => '╯',
            StyleKind::Heavy => '┛',
        },
        (0, 1, -1, 0) | (-1, 0, 0, 1) => match style {
            StyleKind::Light => '└',
            StyleKind::Arc => '╰',
            StyleKind::Heavy => '┗',
        },
        (0, -1, -1, 0) | (-1, 0, 0, -1) => match style {
            StyleKind::Light => '┌',
            StyleKind::Arc => '╭',
            StyleKind::Heavy => '┏',
        },
        (1, 0, -1, 0) | (-1, 0, 1, 0) => match style {
            StyleKind::Light => '─',
            StyleKind::Arc => '─',
            StyleKind::Heavy => '━',
        },
        (0, 1, 0, -1) | (0, -1, 0, 1) => match style {
            StyleKind::Light => '│',
            StyleKind::Arc => '│',
            StyleKind::Heavy => '┃',
        },
        (0, 1, 0, 0) | (0, 0, 0, 1) => match style {
            StyleKind::Light => '╵',
            StyleKind::Arc => '╵',
            StyleKind::Heavy => '╹',
        },
        (0, -1, 0, 0) | (0, 0, 0, -1) => match style {
            StyleKind::Light => '╷',
            StyleKind::Arc => '╷',
            StyleKind::Heavy => '╻',
        },
        (1, 0, 0, 0) | (0, 0, 1, 0) => match style {
            StyleKind::Light => '╴',
            StyleKind::Arc => '╴',
            StyleKind::Heavy => '╸',
        },
        (-1, 0, 0, 0) | (0, 0, -1, 0) => match style {
            StyleKind::Light => '╶',
            StyleKind::Arc => '╶',
            StyleKind::Heavy => '╺',
        },
        _ => '?',
    }
}
pub fn make_curve_string(curve: &mh_curve::MansfieldCurve, style: &StyleKind) -> String {
    let mut curve_string = String::new();
    if curve.dim != 2 {panic!("can only make text of 2d curve")}
    for y in 0..curve.size[1] {
        for x in 0..curve.size[0] {
            let p = mh_curve::Point { pos: vec![x,y] };
            let i = curve.path.iter().position(|p1| p1 == &p);
            match i {
                Some(i) => String::push(&mut curve_string, get_curve_char(curve, i, style)),
                None => (),
            }
        }
        if y < curve.size[1]-1 {
            String::push(&mut curve_string, '\n')
        }
    }
    curve_string
}
