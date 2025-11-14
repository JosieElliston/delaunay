// use crate::{point::*, tree::*};

// #[derive(Debug, Default, Clone)]
// struct Neighbors {
//     up: Option<NodeId>,
//     down: Option<NodeId>,
//     left: Option<NodeId>,
//     right: Option<NodeId>,
// }
// /// a neighbor must be a leaf at an equal or lower depth than us
// /// so bigger leaves can't see smaller leaves
// /// this allows us to not have a vec of neighbors,
// /// and is still correct because the smaller leaves will see the bigger leaf,
// /// so it doesn't matter than the bigger leaf can't see them.
// /// TODO: maybe if it's at a equal depth, it must also be right or down
// /// TODO: optimization: don't have enum AlgNode, have SOA instead
// #[derive(Debug, Clone)]
// pub(crate) enum AlgNode {
//     Leaf {
//         p_i: PointId,
//         neighbors: Neighbors,
//     },
//     /// 2 3
//     /// 0 1
//     Internal([NodeId; 4]),
// }
// #[derive(Debug, Clone)]
// pub(crate) struct AlgTree {
//     domain: Square,
//     points: Vec<Point>,
//     /// root is at index 0
//     /// probably should have parents have lower index than their children
//     /// probably should have children are contiguous
//     nodes: Vec<AlgNode>,
// }
// impl From<FilledNode> for AlgNode {
//     fn from(node: FilledNode) -> Self {
//         match node {
//             FilledNode::Leaf(p_i) => AlgNode::Leaf {
//                 p_i,
//                 neighbors: Default::default(),
//             },
//             FilledNode::Internal(children) => AlgNode::Internal(children),
//         }
//     }
// }
// impl From<FilledTree> for AlgTree {
//     fn from(tree: FilledTree) -> Self {
//         let mut ret = Self {
//             domain: tree.domain,
//             points: tree.points,
//             nodes: tree.nodes.into_iter().map(AlgNode::from).collect(),
//         };

//         fn neighbors_of_node(tree: &AlgTree, node_id: NodeId) -> Neighbors {
//             assert!(matches!(tree.nodes[node_id], AlgNode::Leaf { .. }));

//             // 
//         }

//         todo!()
//     }
// }

// pub(crate) fn triangulate(points: &[Point]) -> Result<Vec<(usize, usize, usize)>, &'static str> {
//     let tree = AlgTree::from(FilledTree::try_from(Tree::from_points(points))?);

//     todo!()
// }
