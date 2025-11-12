use crate::{point::*, utils::*};

pub(crate) fn triangulate(points: &[Point]) -> Vec<(usize, usize, usize)> {
    let mut ret = Vec::new();
    for a_i in 0..points.len() {
        for b_i in a_i + 1..points.len() {
            for c_i in b_i + 1..points.len() {
                // let (a_i, b_i, c_i) = canonicalize(points, a_i, b_i, c_i);
                let a = points[a_i];
                let b = points[b_i];
                let c = points[c_i];
                // rust doesn't have no break
                if {
                    || {
                        for (p_i, &p) in points.iter().enumerate() {
                            if p_i == a_i || p_i == b_i || p_i == c_i {
                                continue;
                            }
                            if incircle(a, b, c, p) {
                                return false;
                            }
                        }
                        true
                    }
                }() {
                    ret.push((a_i, b_i, c_i));
                }
            }
        }
    }
    ret
}
