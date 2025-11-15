use std::ops::{Index, IndexMut};

use eframe::egui::{
    accesskit::Node,
    ahash::{HashSet, HashSetExt},
};

use crate::{point::*, tree::*};

// #[derive(Debug, Clone)]
// pub(crate) enum AlgNodeParent {
//     Leaf {
//         p_i: PointI,
//         // neighbors: Neighbors,
//         /// (parent_i, child_i in parent)
//         parent_child: (NodeI, ChildI),
//     },
//     Internal {
//         children: Children<NodeI>,
//         parent_child: Option<(NodeI, ChildI)>,
//     },
// }
// #[derive(Debug, Clone)]
// pub(crate) struct AlgTreeBuilder {
//     domain: Square,
//     points: Vec<Point>,
//     /// root is at index 0
//     /// probably should have parents have lower index than their children
//     /// probably should have children are contiguous
//     nodes: Vec<AlgNodeParent>,
// }
// // impl From<FilledNode> for AlgNodeBuilder {
// //     fn from(node: FilledNode) -> Self {
// //         match node {
// //             FilledNode::Leaf(p_i) => AlgNodeBuilder::Leaf {
// //                 p_i,
// //                 neighbors: Default::default(),
// //             },
// //             FilledNode::Internal(children) => AlgNodeBuilder::Internal(children),
// //         }
// //     }
// // }
// impl From<FilledTree> for AlgTreeBuilder {
//     fn from(tree: FilledTree) -> Self {
//         // let mut ret = Self {
//         //     domain: tree.domain,
//         //     points: tree.points,
//         //     nodes: tree.nodes.into_iter().map(AlgNodeBuilder::from).collect(),
//         // };

//         // get parent info
//         {
//             let mut frontier = VecDeque::new();
//         }

//         todo!()
//     }
// }

#[derive(Debug, Clone, Copy)]
enum NeighborI {
    Left = 0b00,
    Up = 0b01,
    Right = 0b10,
    Down = 0b11,
}
impl NeighborI {
    const ALL: [NeighborI; 4] = [
        NeighborI::Left,
        NeighborI::Up,
        NeighborI::Right,
        NeighborI::Down,
    ];

    const fn opposite(self) -> Self {
        match self {
            NeighborI::Left => NeighborI::Right,
            NeighborI::Up => NeighborI::Down,
            NeighborI::Right => NeighborI::Left,
            NeighborI::Down => NeighborI::Up,
        }
    }

    const fn clockwise(self) -> Self {
        match self {
            NeighborI::Left => NeighborI::Up,
            NeighborI::Up => NeighborI::Right,
            NeighborI::Right => NeighborI::Down,
            NeighborI::Down => NeighborI::Left,
        }
    }

    const fn clockwise_n(self, n: usize) -> Self {
        match n {
            0 => self,
            1 => self.clockwise(),
            2 => self.clockwise().clockwise(),
            3 => self.clockwise().clockwise().clockwise(),
            _ => panic!("n must be 0..3"),
        }
    }
}
/// a neighbor must be a leaf at an equal or lower depth than us
/// so bigger leaves can't see smaller leaves
/// this allows us to not have a vec of neighbors,
/// and is still correct because the smaller leaves will see the bigger leaf,
/// so it doesn't matter than the bigger leaf can't see them.
// TODO: maybe if it's at a equal depth, it must also be right or down
// #[derive(Debug, Default, Clone)]
// pub(crate) struct Neighbors {
//     left: Option<NodeI>,
//     up: Option<NodeI>,
//     down: Option<NodeI>,
//     right: Option<NodeI>,
// }
// impl IntoIterator for Neighbors {
//     type Item = NodeI;
//     type IntoIter = std::iter::Flatten<std::array::IntoIter<Option<NodeI>, 4>>;

//     fn into_iter(self) -> Self::IntoIter {
//         IntoIterator::into_iter(
//             [self.left, self.up, self.down, self.right]
//                 .into_iter()
//                 .flatten(),
//         )
//     }
// }
#[derive(Debug, Default, Clone)]
pub(crate) struct Neighbors {
    left: Vec<NodeI>,
    up: Vec<NodeI>,
    down: Vec<NodeI>,
    right: Vec<NodeI>,
}
// impl IntoIterator for Neighbors {
//     type Item = NodeI;
//     type IntoIter = std::iter::Flatten<std::array::IntoIter<Option<NodeI>, 4>>;

//     fn into_iter(self) -> Self::IntoIter {
//         IntoIterator::into_iter(
//             [self.left, self.up, self.down, self.right]
//                 .into_iter()
//                 .flatten(),
//         )
//     }
// }
impl IntoIterator for Neighbors {
    type Item = NodeI;
    type IntoIter = std::iter::Flatten<std::array::IntoIter<std::vec::Vec<usize>, 4>>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIterator::into_iter(
            [self.left, self.up, self.down, self.right]
                .into_iter()
                .flatten(),
        )
    }
}
impl Index<NeighborI> for Neighbors {
    type Output = Vec<NodeI>;

    fn index(&self, index: NeighborI) -> &Self::Output {
        match index {
            NeighborI::Left => &self.left,
            NeighborI::Up => &self.up,
            NeighborI::Right => &self.right,
            NeighborI::Down => &self.down,
        }
    }
}
impl IndexMut<NeighborI> for Neighbors {
    fn index_mut(&mut self, index: NeighborI) -> &mut Self::Output {
        match index {
            NeighborI::Left => &mut self.left,
            NeighborI::Up => &mut self.up,
            NeighborI::Right => &mut self.right,
            NeighborI::Down => &mut self.down,
        }
    }
}

// TODO: optimization: don't have enum AlgNode, have SOA instead
#[derive(Debug, Clone)]
pub(crate) enum AlgNode {
    Leaf { p_i: PointI, neighbors: Neighbors },
    Internal(Children<NodeI>),
}
#[derive(Debug, Clone)]
pub(crate) struct AlgTree {
    pub(crate) domain: Square,
    pub(crate) points: Vec<Point>,
    /// root is at index 0
    /// probably should have parents have lower index than their children
    /// probably should have children are contiguous
    pub(crate) nodes: Vec<AlgNode>,
}
// impl From<AlgNodeParent> for AlgNode {
//     fn from(value: AlgNodeParent) -> Self {
//         todo!()
//     }
// }
// impl From<AlgTreeBuilder> for AlgTree {
//     fn from(value: AlgTreeBuilder) -> Self {
//         todo!()
//     }
// }
impl AlgNode {
    fn as_internal(&self) -> &Children<NodeI> {
        match self {
            AlgNode::Internal(children) => children,
            _ => panic!("not an internal node"),
        }
    }
}
impl From<FilledNode> for AlgNode {
    fn from(node: FilledNode) -> Self {
        match node {
            FilledNode::Leaf(p_i) => AlgNode::Leaf {
                p_i,
                neighbors: Default::default(),
            },
            FilledNode::Internal(children) => AlgNode::Internal(children),
        }
    }
}
impl From<FilledTree> for AlgTree {
    fn from(filled_tree: FilledTree) -> Self {
        let mut tree = Self {
            domain: filled_tree.domain,
            points: filled_tree.points,
            nodes: filled_tree.nodes.into_iter().map(AlgNode::from).collect(),
        };

        // get parent info
        // None iff root
        let parent_child: Vec<Option<(NodeI, ChildI)>> = {
            let mut ret = vec![None; tree.nodes.len()];
            let mut frontier = vec![(0usize, None)];
            while let Some((node_i, parent_info)) = frontier.pop() {
                match &tree.nodes[node_i] {
                    AlgNode::Leaf { .. } => {
                        ret[node_i] = parent_info;
                    }
                    AlgNode::Internal(children) => {
                        ret[node_i] = parent_info;
                        frontier.push((children.down_right, Some((node_i, ChildI::DownRight))));
                        frontier.push((children.down_left, Some((node_i, ChildI::DownLeft))));
                        frontier.push((children.up_right, Some((node_i, ChildI::UpRight))));
                        frontier.push((children.up_left, Some((node_i, ChildI::UpLeft))));
                    }
                }
            }
            assert!(ret[0].is_none());
            ret[1..].iter().for_each(|pc| assert!(pc.is_some()));
            ret
        };

        // fn children_along_edge(tree: &AlgTree, direction: NeighborI, node_i: NodeI) -> Vec<NodeI> {
        //     match &tree.nodes[node_i] {
        //         AlgNode::Leaf { .. } => vec![node_i],
        //         AlgNode::Internal(children) => match direction {
        //             NeighborI::Left => {
        //                 let mut result = vec![];
        //                 result.extend(children_along_edge(tree, direction, children.up_left));
        //                 result.extend(children_along_edge(tree, direction, children.down_left));
        //                 result
        //             },
        //             NeighborI::Up => {
        //                 let mut result = vec![];
        //                 result.extend(children_along_edge(tree, direction, children.up_left));
        //                 result.extend(children_along_edge(tree, direction, children.up_right));
        //                 result
        //             },
        //             NeighborI::Right => {
        //                 let mut result = vec![];
        //                 result.extend(children_along_edge(tree, direction, children.up_right));
        //                 result.extend(children_along_edge(tree, direction, children.down_right));
        //                 result
        //             },
        //             NeighborI::Down => {
        //                 let mut result = vec![];
        //                 result.extend(children_along_edge(tree, direction, children.down_left));
        //                 result.extend(children_along_edge(tree, direction, children.down_right));
        //                 result
        //             },
        //         },
        //     }
        // }

        // populate neighbors
        let neighbor_of_node = |direction: NeighborI, node_i: NodeI| -> Option<NodeI> {
            // let neighbors_of_node = |direction: NeighborI, node_i: NodeI| -> Vec<NodeI> {
            assert!(matches!(tree.nodes[node_i], AlgNode::Leaf { .. }));
            // ur dl dr, dr ul
            // go up, storing the child_i taken
            // our parent has child in the correct direction of us, go down to that child
            // go down, mirroring the path we took up along direction
            let mut node_i = node_i;
            // TODO: the path up only needs 1 bit per step, not 2
            let mut path_up: Vec<ChildI> = Vec::new();
            loop {
                let Some((parent_i, child_i)) = parent_child[node_i] else {
                    return None;
                    // return Vec::new();
                };

                match (direction, child_i) {
                    // (NeighborI::Left, ChildI::DownRight) | (NeighborI::Left, ChildI::UpRight) => {
                    //     node_i = parent_i;
                    //     break;
                    // }
                    // (NeighborI::Up, ChildI::DownLeft) | (NeighborI::Up, ChildI::DownRight) => {
                    //     break;
                    // }
                    // (NeighborI::Right, ChildI::DownLeft) | (NeighborI::Right, ChildI::UpLeft) => {
                    //     break;
                    // }
                    // (NeighborI::Down, ChildI::UpLeft) | (NeighborI::Down, ChildI::UpRight) => {
                    //     break;
                    // }
                    (NeighborI::Left, ChildI::DownRight) => {
                        node_i = tree.nodes[parent_i].as_internal().down_left;
                        break;
                    }
                    (NeighborI::Left, ChildI::UpRight) => {
                        node_i = tree.nodes[parent_i].as_internal().up_left;
                        break;
                    }
                    (NeighborI::Up, ChildI::DownLeft) => {
                        node_i = tree.nodes[parent_i].as_internal().up_left;
                        break;
                    }
                    (NeighborI::Up, ChildI::DownRight) => {
                        node_i = tree.nodes[parent_i].as_internal().up_right;
                        break;
                    }
                    (NeighborI::Right, ChildI::DownLeft) => {
                        node_i = tree.nodes[parent_i].as_internal().down_right;
                        break;
                    }
                    (NeighborI::Right, ChildI::UpLeft) => {
                        node_i = tree.nodes[parent_i].as_internal().up_right;
                        break;
                    }
                    (NeighborI::Down, ChildI::UpLeft) => {
                        node_i = tree.nodes[parent_i].as_internal().down_left;
                        break;
                    }
                    (NeighborI::Down, ChildI::UpRight) => {
                        node_i = tree.nodes[parent_i].as_internal().down_right;
                        break;
                    }
                    _ => {
                        path_up.push(child_i);
                        node_i = parent_i;
                    }
                }
            }
            while let Some(child_i) = path_up.pop() {
                match &tree.nodes[node_i] {
                    AlgNode::Leaf { .. } => return Some(node_i),
                    AlgNode::Internal(children) => match (direction, child_i) {
                        (NeighborI::Left, ChildI::DownLeft) => {
                            node_i = children.down_right;
                        }
                        (NeighborI::Left, ChildI::UpLeft) => {
                            node_i = children.up_right;
                        }
                        (NeighborI::Up, ChildI::UpLeft) => {
                            node_i = children.down_left;
                        }
                        (NeighborI::Up, ChildI::UpRight) => {
                            node_i = children.down_right;
                        }
                        (NeighborI::Right, ChildI::DownRight) => {
                            node_i = children.down_left;
                        }
                        (NeighborI::Right, ChildI::UpRight) => {
                            node_i = children.up_left;
                        }
                        (NeighborI::Down, ChildI::DownLeft) => {
                            node_i = children.up_left;
                        }
                        (NeighborI::Down, ChildI::DownRight) => {
                            node_i = children.up_right;
                        }
                        // (NeighborI::Left, ChildI::DownRight) => {}
                        // (NeighborI::Left, ChildI::UpRight) => {}
                        // (NeighborI::Up, ChildI::DownLeft) => {}
                        // (NeighborI::Up, ChildI::DownRight) => {}
                        // (NeighborI::Right, ChildI::DownLeft) => {}
                        // (NeighborI::Right, ChildI::UpLeft) => {}
                        // (NeighborI::Down, ChildI::UpLeft) => {}
                        // (NeighborI::Down, ChildI::UpRight) => {}
                        _ => unreachable!(),
                    },
                }
            }
            match &tree.nodes[node_i] {
                AlgNode::Leaf { .. } => Some(node_i),
                AlgNode::Internal(_) => None,
            }
        };

        // TODO: bad
        // let mut new_nodes = tree.nodes.clone();
        // for (node_i, node) in new_nodes.iter_mut().enumerate() {
        //     if let AlgNode::Leaf { neighbors, .. } = node {
        //         neighbors.left = neighbor_of_node(NeighborI::Left, node_i as NodeI);
        //         neighbors.up = neighbor_of_node(NeighborI::Up, node_i as NodeI);
        //         neighbors.right = neighbor_of_node(NeighborI::Right, node_i as NodeI);
        //         neighbors.down = neighbor_of_node(NeighborI::Down, node_i as NodeI);
        //     }
        // }
        // tree.nodes = new_nodes;
        // tree

        // TODO: this is very slow

        // build neighbor map
        let neighbor_map = {
            let mut neighbor_map: Vec<Neighbors> = vec![Default::default(); tree.nodes.len()];
            for node_i in 0..tree.nodes.len() {
                let AlgNode::Leaf { .. } = &tree.nodes[node_i] else {
                    continue;
                };
                for direction in NeighborI::ALL {
                    let Some(neighbor_i) = neighbor_of_node(direction, node_i) else {
                        continue;
                    };
                    neighbor_map[node_i][direction].push(neighbor_i);
                    neighbor_map[neighbor_i][direction.opposite()].push(node_i);
                }
            }
            neighbor_map
        };

        // dedup
        let neighbor_map: Vec<Neighbors> = {
            neighbor_map
                .into_iter()
                .map(|neighbors| Neighbors {
                    left: Vec::from_iter(HashSet::from_iter(neighbors.left)),
                    up: Vec::from_iter(HashSet::from_iter(neighbors.up)),
                    down: Vec::from_iter(HashSet::from_iter(neighbors.down)),
                    right: Vec::from_iter(HashSet::from_iter(neighbors.right)),
                })
                .collect()
        };

        // assign
        for (node_i, new_neighbors) in neighbor_map.iter().enumerate() {
            let AlgNode::Leaf {
                neighbors: ref mut old_neighbors,
                ..
            } = tree.nodes[node_i]
            else {
                continue;
            };
            *old_neighbors = new_neighbors.clone();
        }
        tree
    }
}
impl AlgTree {
    pub(crate) fn neighbors(&self, node_i: NodeI) -> impl Iterator<Item = NodeI> {
        let neighbors = match &self.nodes[node_i] {
            AlgNode::Leaf { neighbors, .. } => neighbors,
            _ => panic!("not a leaf node"),
        };
        neighbors.clone().into_iter()
        // [
        //     neighbors.left,
        //     neighbors.up,
        //     neighbors.right,
        //     neighbors.down,
        // ]
        // .into_iter()
        // .flatten()
    }

    // pub(crate) fn neighbors2(&self, node_i: NodeI) -> HashSet<NodeI> {
    //     let neighbors = match &self.nodes[node_i] {
    //         AlgNode::Leaf { neighbors, .. } => neighbors.clone().into_iter().collect::<Vec<_>>(),
    //         _ => panic!("not a leaf node"),
    //     };
    //     let mut ret = HashSet::new();
    //     for neighbor_i in neighbors {
    //         ret.insert(neighbor_i);
    //         for neighbor2_i in self.neighbors(neighbor_i) {
    //             ret.insert(neighbor2_i);
    //         }
    //     }
    //     ret.remove(&node_i);
    //     ret
    // }

    // pub(crate) fn neighbors2(&self, node_i: NodeI) -> HashSet<NodeI> {
    //     let neighbors = match &self.nodes[node_i] {
    //         AlgNode::Leaf { neighbors, .. } => neighbors.clone().into_iter().collect::<Vec<_>>(),
    //         _ => panic!("not a leaf node"),
    //     };

    //     // TODO: this is very very slow
    //     fn collect_nodes(tree: &AlgTree, path: &[NeighborI], node_i: NodeI) -> Vec<NodeI> {
    //         if path.is_empty() {
    //             return vec![node_i];
    //         }
    //         let AlgNode::Leaf { neighbors, .. } = &tree.nodes[node_i] else {
    //             panic!("not a leaf node");
    //         };
    //         let mut ret = Vec::new();
    //         for neighbor_i in &neighbors[path[0]] {
    //             ret.extend(collect_nodes(tree, &path[1..], *neighbor_i));
    //         }
    //         ret
    //     }
    //     let mut ret = HashSet::new();
    //     let paths: [&[NeighborI]; _] = {
    //         use NeighborI as NbI;
    //         [
    //             &[NbI::Left],
    //             &[NbI::Left, NbI::Left],
    //             &[NbI::Left, NbI::Right],
    //             &[NbI::Left, NbI::Up],
    //             &[NbI::Left, NbI::Down],
    //             &[NbI::Left, NbI::Left, NbI::Up],
    //             &[NbI::Left, NbI::Left, NbI::Down],
    //             &[NbI::Left, NbI::Up, NbI::Left],
    //             &[NbI::Left, NbI::Down, NbI::Left],
    //             &[NbI::Left, NbI::Up, NbI::Right],
    //             &[NbI::Left, NbI::Down, NbI::Right],
    //             &[NbI::Left, NbI::Left, NbI::Up, NbI::Up],
    //             &[NbI::Left, NbI::Left, NbI::Down, NbI::Down],
    //         ]
    //     };
    //     for cw in 0..4 {
    //         for path in paths {
    //             ret.extend(collect_nodes(
    //                 self,
    //                 &path
    //                     .iter()
    //                     .map(|direction| direction.clockwise_n(cw))
    //                     .collect::<Vec<_>>(),
    //                 node_i,
    //             ));
    //         }
    //     }
    //     ret
    // }
    pub(crate) fn neighbors_diag(&self, node_i: NodeI) -> HashSet<NodeI> {
        assert!(matches!(self.nodes[node_i], AlgNode::Leaf { .. }));
        fn collect_nodes(tree: &AlgTree, path: &[NeighborI], node_i: NodeI) -> Vec<NodeI> {
            if path.is_empty() {
                return vec![node_i];
            }
            let AlgNode::Leaf { neighbors, .. } = &tree.nodes[node_i] else {
                panic!("not a leaf node");
            };
            let mut ret = Vec::new();
            for neighbor_i in &neighbors[path[0]] {
                ret.extend(collect_nodes(tree, &path[1..], *neighbor_i));
            }
            ret
        }
        let mut ret = HashSet::new();
        let paths: [&[NeighborI]; _] = {
            use NeighborI as NbI;
            [
                &[NbI::Left],
                // &[NbI::Left, NbI::Right],
                &[NbI::Left, NbI::Up],
                &[NbI::Left, NbI::Down],
            ]
        };
        for cw in 0..4 {
            for path in paths {
                ret.extend(collect_nodes(
                    self,
                    &path
                        .iter()
                        .map(|direction| direction.clockwise_n(cw))
                        .collect::<Vec<_>>(),
                    node_i,
                ));
            }
        }
        ret
    }

    pub(crate) fn neighbors2(&self, node_i: NodeI) -> HashSet<NodeI> {
        assert!(matches!(self.nodes[node_i], AlgNode::Leaf { .. }));
        let mut ret = HashSet::new();
        // for neighbor_i in self.neighbors_diag(node_i) {
        // // for neighbor_i in self.neighbors(node_i) {
        //     ret.insert(neighbor_i);
        //     for neighbor2_i in self.neighbors_diag(neighbor_i) {
        //         ret.insert(neighbor2_i);
        //     }
        // }
        // for neighbor_i in self.neighbors_diag(node_i) {
        for neighbor_i in self.neighbors(node_i) {
            ret.insert(neighbor_i);
            for neighbor2_i in self.neighbors(neighbor_i) {
                ret.insert(neighbor2_i);
                for neighbor3_i in self.neighbors(neighbor2_i) {
                    ret.insert(neighbor3_i);
                }
            }
        }
        ret.remove(&node_i);
        ret
    }
}

pub(crate) fn triangulate(points: &[Point]) -> Result<Vec<(usize, usize, usize)>, &'static str> {
    // let tree = AlgTree::from(AlgTreeBuilder::from(FilledTree::try_from(
    //     QuadTree::from_points(points),
    // )?));

    todo!()
}
