// use crate::{point::*, tree::NodeId};

// #[derive(Debug, Clone)]
// /// a direction node must be a leaf at an equal or lower depth than us
// /// TODO: maybe if it's at a equal depth, it must also be right or down
// pub(crate) enum AlgNode {
//     Leaf {
//         p_i: PointId,
//         up: Option<NodeId>,
//         down: Option<NodeId>,
//         left: Option<NodeId>,
//         right: Option<NodeId>,
//     },
//     /// 2 3
//     /// 0 1
//     Internal([NodeId; 4]),
// }
// #[derive(Debug, Clone)]
// pub(crate) struct Tree {
//     domain: Square,
//     points: Vec<Point>,
//     /// root is at index 0
//     /// probably should have parents have lower index than their children
//     /// probably should have children are contiguous
//     nodes: Vec<Node>,
// }



// pub(crate) fn triangulate(points: &[Point]) -> Vec<(usize, usize, usize)> {}