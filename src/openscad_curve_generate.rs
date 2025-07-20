use crate::mh_curve;

pub fn make_curve_openscad_file(curve: &mh_curve::MansfieldCurve) -> String {
    let header_string = "// Transpose of matrix A (swap rows and columns)
function transpose(A) = [for (j = [0:len(A[0])-1]) [for(i = [0:len(A)-1]) A[i][j]]];

//  Cylinder of radius r from P to Q
module cyl_between(P, Q, r){
    v = Q - P;    // vector from P to Q
    L = norm(v);  // height of the cylnder = dist(P, Q) 
    c = v / L;    // unit vector: direction from P to Q    
    is_c_vertical = ( 1 - abs(c * [0, 0, 1]) < 1e-6); //is c parallel to z axis?
    u = is_c_vertical ? [1, 0, 0] : cross([0, 0, 1], c); // normal to c and Z axis
    a = u / norm(u); // unit vector normal to c and the Z axis
    b = cross(c, a); // unit vector normal to a and b
    // [a, b, c] is an orthonormal basis, i.e. the rotation matrix; P is the translation
    MT = [a, b, c, P]; // the transformation matrix
    M = transpose(MT); // OpenSCAD wants vectors in columns, so we need to transpose
    multmatrix(M)
        translate([-r/2,-r/2,0])
            cube([r,r,L+r/2]);
}
";
    let mut curve_string: String = String::new();
    curve_string.push_str(header_string);
    curve_string.push_str("rad = 0.5;\n");
    for i in 1..curve.path.len() {
            let prevpos = &curve.path[i-1].pos;
            let pos = &curve.path[i].pos;
            if curve.dim == 3 {
                curve_string.push_str(&format!("cyl_between([{},{},{}],[{},{},{}],rad);",prevpos[0],prevpos[1],prevpos[2],pos[0],pos[1],pos[2]));
            } else if curve.dim == 2 {
                curve_string.push_str(&format!("cyl_between([{},{},{}],[{},{},{}],rad);",prevpos[0],prevpos[1],0,pos[0],pos[1],0));
            } else { panic!("2 or 3 dimensional for openscad")}
            curve_string.push('\n');
        };
    curve_string
}