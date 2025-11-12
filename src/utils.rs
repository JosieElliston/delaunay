use crate::point::*;

pub(crate) fn lerp_f32(lo: f32, hi: f32, t: f32) -> f32 {
    assert!(lo < hi);
    // assert!((0.0..=1.0).contains(&t));
    lo + (hi - lo) * t
}

pub(crate) fn inv_lerp_f32(lo: f32, hi: f32, x: f32) -> f32 {
    assert!(lo < hi);
    // assert!((lo..=hi).contains(&x));
    (x - lo) / (hi - lo)
}

pub(crate) fn lerp_f64(lo: f64, hi: f64, t: f64) -> f64 {
    assert!(lo < hi);
    // assert!((0.0..=1.0).contains(&t));
    lo + (hi - lo) * t
}

pub(crate) fn inv_lerp_f64(lo: f64, hi: f64, x: f64) -> f64 {
    assert!(lo < hi);
    // assert!((lo..=hi).contains(&x));
    (x - lo) / (hi - lo)
}

pub(crate) fn lerp_coord(lo: Coord, hi: Coord, t: Coord) -> Coord {
    assert!(lo < hi);
    // assert!((0.0..=1.0).contains(&t));
    lo + (hi - lo) * t
}

pub(crate) fn inv_lerp_coord(lo: Coord, hi: Coord, x: Coord) -> Coord {
    assert!(lo < hi);
    // assert!((lo..=hi).contains(&x));
    (x - lo) / (hi - lo)
}

pub(crate) fn canonicalize_edge(a_i: usize, b_i: usize) -> (usize, usize) {
    assert!(a_i != b_i);
    if a_i < b_i { (a_i, b_i) } else { (b_i, a_i) }
}

pub(crate) fn is_ccw(a: Point, b: Point, c: Point) -> bool {
    let det = (b.x() - a.x()) * (c.y() - a.y()) - (c.x() - a.x()) * (b.y() - a.y());
    assert_ne!(det, 0.0, "collinear");
    det > 0.0
}

/// the min index is at 0 and the the triangle is ccw
pub(crate) fn canonicalize_tri(
    points: &[Point],
    a_i: usize,
    b_i: usize,
    c_i: usize,
) -> (usize, usize, usize) {
    assert!(a_i != b_i && b_i != c_i && c_i != a_i);

    let (a_i, b_i, c_i) = if a_i < b_i && a_i < c_i {
        (a_i, b_i, c_i)
    } else if b_i < a_i && b_i < c_i {
        (b_i, a_i, c_i)
    } else {
        (c_i, a_i, b_i)
    };

    if is_ccw(points[a_i], points[b_i], points[c_i]) {
        (a_i, b_i, c_i)
    } else {
        (a_i, c_i, b_i)
    }
}

/// returns whether the point is in the triangle
/// probably should be ccw
pub(crate) fn intriangle_ccw(a: Point, b: Point, c: Point, p: Point) -> bool {
    let abp = (b.x() - a.x()) * (p.y() - a.y()) - (b.y() - a.y()) * (p.x() - a.x());
    let bcp = (c.x() - b.x()) * (p.y() - b.y()) - (c.y() - b.y()) * (p.x() - b.x());
    let cap = (a.x() - c.x()) * (p.y() - c.y()) - (a.y() - c.y()) * (p.x() - c.x());

    (abp >= 0.0 && bcp >= 0.0 && cap >= 0.0) || (abp <= 0.0 && bcp <= 0.0 && cap <= 0.0)
}

pub(crate) fn intriangle(a: Point, b: Point, c: Point, p: Point) -> bool {
    if is_ccw(a, b, c) {
        intriangle_ccw(a, b, c, p)
    } else {
        !intriangle_ccw(a, b, c, p)
    }
}

// TODO: return Ordering instead of bool
// TODO: https://www.cs.cmu.edu/afs/cs/project/quake/public/code/predicates.c
pub(crate) fn incircle(a: Point, b: Point, c: Point, p: Point) -> bool {
    if is_ccw(a, b, c) {
        incircle_ccw(a, b, c, p)
    } else {
        !incircle_ccw(a, b, c, p)
    }
}

pub(crate) fn incircle_ccw(a: Point, b: Point, c: Point, p: Point) -> bool {
    let ax = a.x() - p.x();
    let ay = a.y() - p.y();
    let bx = b.x() - p.x();
    let by = b.y() - p.y();
    let cx = c.x() - p.x();
    let cy = c.y() - p.y();

    let det = (ax * ax + ay * ay) * (bx * cy - cx * by) - (bx * bx + by * by) * (ax * cy - cx * ay)
        + (cx * cx + cy * cy) * (ax * by - bx * ay);

    det > 0.0
}
