use spade::*;

use crate::point::*;

pub(crate) fn triangulate(points: &[Point]) -> Vec<(usize, usize, usize)> {
    DelaunayTriangulation::<Point2<Coord>>::bulk_load_stable(
        points.iter().cloned().map(Point2::from).collect(),
    )
    .unwrap()
    .inner_faces()
    .map(|face| face.vertices().map(|v| v.index()).into())
    .collect::<Vec<(PointI, PointI, PointI)>>()
}
