use crate::{point::*, utils::incircle};

pub(crate) fn triangulate(points: &[Point]) -> Vec<(usize, usize, usize)> {
    let mut ret = Vec::new();
    for i in 0..points.len() {
        for j in i + 1..points.len() {
            for k in j + 1..points.len() {
                let a = points[i];
                let b = points[j];
                let c = points[k];
                // rust doesn't have no break
                if {
                    || {
                        for &p in points {
                            if p == a || p == b || p == c {
                                continue;
                            }
                            if incircle(a, b, c, p) {
                                return false;
                            }
                        }
                        true
                    }
                }() {
                    ret.push((i, j, k));
                }
            }
        }
    }
    ret
}
