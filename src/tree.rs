use std::{collections::VecDeque, vec};

use rand::prelude::*;

use crate::point::{self, *};

pub(crate) type NodeI = usize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum ChildI {
    DownLeft = 0,
    DownRight = 1,
    UpLeft = 2,
    UpRight = 3,
}

#[derive(Debug, Clone)]
pub(crate) struct Square {
    mid: Point,
    rad: Coord,
}
impl Square {
    pub(crate) const UNIT: Self = Self {
        mid: Point::new(0.5, 0.5),
        rad: 0.5,
    };

    pub(crate) fn new(mid: Point, rad: Coord) -> Self {
        assert!(rad > 0.0);
        Self { mid, rad }
    }

    pub(crate) fn mid(&self) -> Point {
        self.mid
    }
    pub(crate) fn rad(&self) -> Coord {
        self.rad
    }

    pub(crate) fn x_lo(&self) -> Coord {
        self.mid.x() - self.rad
    }
    pub(crate) fn x_hi(&self) -> Coord {
        self.mid.x() + self.rad
    }
    pub(crate) fn y_lo(&self) -> Coord {
        self.mid.y() - self.rad
    }
    pub(crate) fn y_hi(&self) -> Coord {
        self.mid.y() + self.rad
    }

    pub(crate) fn down_left(&self) -> Point {
        Point::new(self.x_lo(), self.y_lo())
    }
    pub(crate) fn down_right(&self) -> Point {
        Point::new(self.x_hi(), self.y_lo())
    }
    pub(crate) fn up_left(&self) -> Point {
        Point::new(self.x_lo(), self.y_hi())
    }
    pub(crate) fn up_right(&self) -> Point {
        Point::new(self.x_hi(), self.y_hi())
    }

    pub(crate) fn up_mid(&self) -> Point {
        Point::new(self.mid.x(), self.y_hi())
    }
    pub(crate) fn down_mid(&self) -> Point {
        Point::new(self.mid.x(), self.y_lo())
    }
    pub(crate) fn left_mid(&self) -> Point {
        Point::new(self.x_lo(), self.mid.y())
    }
    pub(crate) fn right_mid(&self) -> Point {
        Point::new(self.x_hi(), self.mid.y())
    }

    pub(crate) fn sample(&self, rng: &mut impl rand::Rng) -> Point {
        Point::new(
            rng.random_range(self.x_lo()..self.x_hi()),
            rng.random_range(self.y_lo()..self.y_hi()),
        )
    }

    /// y-up:
    /// 2 3
    /// 0 1
    pub(crate) fn children(&self) -> Children<Square> {
        let half_rad = self.rad / 2.0;
        Children::<Square> {
            down_left: Square::new(
                Point::new(self.mid.x() - half_rad, self.mid.y() - half_rad),
                half_rad,
            ),
            down_right: Square::new(
                Point::new(self.mid.x() + half_rad, self.mid.y() - half_rad),
                half_rad,
            ),
            up_left: Square::new(
                Point::new(self.mid.x() - half_rad, self.mid.y() + half_rad),
                half_rad,
            ),
            up_right: Square::new(
                Point::new(self.mid.x() + half_rad, self.mid.y() + half_rad),
                half_rad,
            ),
        }
    }

    /// y-up:
    /// 2 3
    /// 0 1
    pub(crate) fn child_i_of_point(&self, point: Point) -> ChildI {
        match (point.x() < self.mid.x(), point.y() < self.mid.y()) {
            (true, true) => ChildI::DownLeft,
            (false, true) => ChildI::DownRight,
            (true, false) => ChildI::UpLeft,
            (false, false) => ChildI::UpRight,
        }
    }

    pub(crate) fn contains(&self, point: Point) -> bool {
        point.x() >= self.x_lo()
            && point.x() <= self.x_hi()
            && point.y() >= self.y_lo()
            && point.y() <= self.y_hi()
    }
}
impl Default for Square {
    fn default() -> Self {
        Self::UNIT
    }
}

/// basically a quadtree where every leaf is empty
#[derive(Debug, Clone)]
pub(crate) enum TreeShape {
    Leaf,
    Internal(Box<[TreeShape; 4]>),
}
impl TreeShape {
    pub(crate) fn from_random(leaf_prob: f64, max_depth: usize, rng: &mut impl Rng) -> Self {
        assert!((0.0..=1.0).contains(&leaf_prob));
        if max_depth == 0 || rng.random_bool(leaf_prob) {
            TreeShape::Leaf
        } else {
            TreeShape::Internal(Box::new([
                TreeShape::from_random(leaf_prob, max_depth - 1, rng),
                TreeShape::from_random(leaf_prob, max_depth - 1, rng),
                TreeShape::from_random(leaf_prob, max_depth - 1, rng),
                TreeShape::from_random(leaf_prob, max_depth - 1, rng),
            ]))
        }
    }
}

// #[derive(Debug, Clone)]
// struct TreeShape {
//     root: ShapeNode,
//     // domain: Square,
// }
// impl TreeShape {
//     fn children_domains(&self) -> impl Iterator<Item = (ShapeNode, Square)>
// }

#[derive(Debug, Clone)]
pub(crate) struct Children<T> {
    pub(crate) down_left: T,
    pub(crate) down_right: T,
    pub(crate) up_left: T,
    pub(crate) up_right: T,
}
impl<T> std::ops::Index<ChildI> for Children<T> {
    type Output = T;

    fn index(&self, index: ChildI) -> &Self::Output {
        match index {
            ChildI::DownLeft => &self.down_left,
            ChildI::DownRight => &self.down_right,
            ChildI::UpLeft => &self.up_left,
            ChildI::UpRight => &self.up_right,
        }
    }
}
// impl<T> From<Children<T>> for [T; 4] {
//     fn from(children: Children<T>) -> Self {
//         [
//             children.down_left,
//             children.down_right,
//             children.up_left,
//             children.up_right,
//         ]
//     }
// }
impl<T> IntoIterator for Children<T> {
    type Item = T;
    type IntoIter = std::array::IntoIter<T, 4>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIterator::into_iter([self.down_left, self.down_right, self.up_left, self.up_right])
    }
}

#[derive(Debug, Clone)]
pub(crate) enum QuadNode {
    Leaf(Option<PointI>),
    /// y-up:
    /// 2 3
    /// 0 1
    Internal(Children<NodeI>),
}
#[derive(Debug, Clone)]
pub(crate) struct QuadTree {
    domain: Square,
    points: Vec<Point>,
    /// root is at index 0
    /// probably should have parents have lower index than their children
    /// probably should have children are contiguous
    nodes: Vec<QuadNode>,
}
impl Default for QuadTree {
    fn default() -> Self {
        Self {
            domain: Square::default(),
            points: Vec::new(),
            nodes: vec![QuadNode::Leaf(None)],
        }
    }
}
impl QuadTree {
    // pub(crate) fn from_domain(domain: Square) -> Self {
    //     Self {
    //         domain,
    //         points: Vec::new(),
    //         nodes: Vec::new(),
    //     }
    // }

    pub(crate) fn points(&self) -> &[Point] {
        &self.points
    }
    pub(crate) fn nodes(&self) -> &[QuadNode] {
        &self.nodes
    }

    pub(crate) fn from_points(points: &[Point]) -> Self {
        let mut ret = Self::default();
        for &point in points {
            ret.insert(point);
        }
        ret
    }

    /// already have p_i in points
    /// but this will increase the len of nodes
    fn insert_into_node(&mut self, p_i: PointI, node_i: NodeI, domain: &Square) {
        assert!(domain.contains(self.points[p_i]));
        match self.nodes[node_i].clone() {
            QuadNode::Leaf(None) => {
                // insert into the empty leaf
                self.nodes[node_i] = QuadNode::Leaf(Some(p_i));
            }
            QuadNode::Leaf(Some(leaf_point_i)) => {
                // split the leaf, then insert both points
                let node_len = self.nodes.len();
                self.nodes
                    .extend_from_slice(&[const { QuadNode::Leaf(None) }; 4]);
                self.nodes[node_i] = QuadNode::Internal(Children {
                    down_left: node_len,
                    down_right: node_len + 1,
                    up_left: node_len + 2,
                    up_right: node_len + 3,
                });
                self.insert_into_node(leaf_point_i, node_i, domain);
                self.insert_into_node(p_i, node_i, domain);
            }
            QuadNode::Internal(children) => {
                let child_i = domain.child_i_of_point(self.points[p_i]);
                self.insert_into_node(p_i, children[child_i], &domain.children()[child_i]);
                // match children[child_i] {
                //     Some(child_node_i) => {
                //         self.insert_into_node(p_i, child_node_i, domain.children()[child_i]);
                //     }
                //     None => {
                //         let new_node_i = self.nodes.len();
                //         self.nodes.push(Node::Leaf(p_i));
                //         // TODO: this is kinda evil
                //         let Node::Internal(children) = &mut self.nodes[node_i] else {
                //             unreachable!()
                //         };
                //         children[child_i] = Some(new_node_i);
                //     }
                // }
            }
        }
    }

    // TODO: change the domain if needed
    pub(crate) fn insert(&mut self, p: Point) {
        let domain = self.domain.clone();
        let p_i = self.points.len();
        self.points.push(p);
        self.insert_into_node(p_i, 0, &domain);
        // if self.nodes.is_empty() {
        //     self.nodes.push(Node::Leaf(0));
        // } else {
        //     self.insert_into_node(self.points.len() - 1, 0, &self.domain);
        // }
    }

    // /// returns None if in leaf or TODO
    // pub(crate) fn child_of_point(&self, point: Point, node_i: NodeId) -> Option<NodeId> {
    //     match &self.nodes[node_i] {
    //         Node::Leaf(_) => None,
    //         Node::Internal(children) => match (
    //             point.x() < self.domain.mid.x(),
    //             point.y() < self.domain.mid.y(),
    //         ) {
    //             (true, true) => children[0],
    //             (true, false) => children[2],
    //             (false, true) => children[1],
    //             (false, false) => children[3],
    //         },
    //     }
    // }

    pub(crate) fn node_of_point(&self, point: Point) -> Option<NodeI> {
        let mut node_i = 0;
        let mut domain = self.domain.clone();
        if !domain.contains(point) {
            return None;
        }
        loop {
            match &self.nodes[node_i] {
                QuadNode::Leaf(_) => {
                    return Some(node_i);
                }
                QuadNode::Internal(children) => {
                    let child_i = domain.child_i_of_point(point);
                    node_i = children[child_i];
                    domain = domain.children()[child_i].clone();
                }
            }
        }
    }

    // pub(crate) fn domain_of_node(&self, node_i: NodeId) -> Square {
    //     assert!(node_i < self.nodes.len());
    //     let mut domain = self.domain.clone();
    //     let mut current_i = 0;
    //     if node_i == 0 {
    //         return domain;
    //     }
    //     loop {
    //         match &self.nodes[current_i] {
    //             Node::Leaf(_) => {
    //                 return None;
    //             }
    //             Node::Internal(children) => {
    //                 let mut found = false;
    //                 for (child_i, &child_node_i) in children.iter().enumerate() {
    //                     if child_node_i == node_i {
    //                         domain = domain.children()[child_i].clone();
    //                         return Some(domain);
    //                     }
    //                     if child_node_i > node_i {
    //                         current_i = child_node_i;
    //                         domain = domain.children()[child_i].clone();
    //                         found = true;
    //                         break;
    //                     }
    //                 }
    //                 if !found {
    //                     return None;
    //                 }
    //             }
    //         }
    //     }
    // }

    // pub(crate) fn nodes_of_point(&self, point: Point) -> impl Iterator<Item = NodeId> {
    //     // let mut node_i = 0;
    //     // loop {
    //     //     match &self.nodes[node_i] {
    //     //         Node::Leaf(_) => {
    //     //             yield node_i;
    //     //             return;
    //     //         }
    //     //         Node::Internal(children) => {
    //     //             node_i = match (
    //     //                 point.x() < self.domain.mid.x(),
    //     //                 point.y() < self.domain.mid.y(),
    //     //             ) {
    //     //                 (true, true) => children[0].unwrap(),
    //     //                 (true, false) => children[2].unwrap(),
    //     //                 (false, true) => children[1].unwrap(),
    //     //                 (false, false) => children[3].unwrap(),
    //     //             };
    //     //             yield node_i;
    //     //         }
    //     //     }
    //     // }
    //     // let mut node_i = 0;
    //     // (0..).map_while(move |_| match &self.nodes[node_i] {
    //     //     Node::Leaf(_) => None,
    //     //     Node::Internal(children) => {
    //     //         node_i = match (
    //     //             point.x() < self.domain.mid.x(),
    //     //             point.y() < self.domain.mid.y(),
    //     //         ) {
    //     //             (true, true) => children[0].unwrap(),
    //     //             (true, false) => children[2].unwrap(),
    //     //             (false, true) => children[1].unwrap(),
    //     //             (false, false) => children[3].unwrap(),
    //     //         };
    //     //         Some(node_i)
    //     //     }
    //     // })
    //     // let mut node_i = 0;
    //     // (0..).map_while(move |_| match self.child_of_point(point, node_i) {
    //     //     None => None,
    //     //     Some(child_i) => todo!(),
    //     // })
    //     todo!()
    // }
}
impl From<&TreeShape> for QuadTree {
    fn from(shape: &TreeShape) -> Self {
        let mut nodes = vec![QuadNode::Leaf(None)];
        let mut frontier = VecDeque::from([(0, shape)]);

        while let Some((node_i, shape)) = frontier.pop_front() {
            // make the node at node_i match shape
            assert!(node_i < nodes.len());
            assert!(matches!(nodes[node_i], QuadNode::Leaf(None)));
            match shape {
                TreeShape::Leaf => {}
                TreeShape::Internal(children_shapes) => {
                    let node_len = nodes.len();
                    nodes.extend_from_slice(&[const { QuadNode::Leaf(None) }; 4]);
                    nodes[node_i] = QuadNode::Internal(Children {
                        down_left: node_len,
                        down_right: node_len + 1,
                        up_left: node_len + 2,
                        up_right: node_len + 3,
                    });
                    for (i, child_shape) in children_shapes.iter().enumerate() {
                        frontier.push_back((node_len + i, child_shape));
                    }
                }
            }
        }

        Self {
            domain: Square::UNIT,
            points: Vec::new(),
            nodes,
        }
    }
}
impl From<FilledNode> for QuadNode {
    fn from(filled_node: FilledNode) -> Self {
        match filled_node {
            FilledNode::Leaf(p_i) => QuadNode::Leaf(Some(p_i)),
            FilledNode::Internal(children) => QuadNode::Internal(children),
        }
    }
}
impl From<FilledTree> for QuadTree {
    fn from(filled_tree: FilledTree) -> Self {
        Self {
            domain: filled_tree.domain,
            points: filled_tree.points,
            nodes: filled_tree.nodes.into_iter().map(QuadNode::from).collect(),
        }
    }
}

/// every internal node has four children
/// ie every leaf is filled
#[derive(Debug, Clone)]
pub(crate) enum FilledNode {
    Leaf(PointI),
    Internal(Children<NodeI>),
}
#[derive(Debug, Default, Clone)]
pub(crate) struct FilledTree {
    pub(crate) domain: Square,
    pub(crate) points: Vec<Point>,
    pub(crate) nodes: Vec<FilledNode>,
}
impl FilledTree {
    /// keep the same structure, but randomize where the points are in each leaf
    pub(crate) fn randomize(&mut self) {
        todo!()
    }

    // pub(crate) fn domain(&self) -> &Square {
    //     &self.domain
    // }
    // pub(crate) fn points(&self) -> &[Point] {
    //     &self.points
    // }
    // pub(crate) fn nodes(&self) -> &[FilledNode] {
    //     &self.nodes
    // }

    pub(crate) fn from_tree_shape_random_filled(shape: &TreeShape, rng: &mut impl Rng) -> Self {
        let mut ret: QuadTree = shape.into();

        let mut frontier = vec![(0, Square::UNIT)];
        while let Some((node_i, domain)) = frontier.pop() {
            match &ret.nodes[node_i] {
                QuadNode::Leaf(None) => {
                    let p_i = ret.points.len();
                    ret.points.push(domain.sample(rng));
                    ret.nodes[node_i] = QuadNode::Leaf(Some(p_i));
                }
                QuadNode::Leaf(Some(_)) => {
                    unreachable!()
                }
                QuadNode::Internal(children) => {
                    for (child_i, child_domain) in children
                        .clone()
                        .into_iter()
                        .zip(domain.children().into_iter())
                    {
                        frontier.push((child_i, child_domain));
                    }
                }
            }
        }

        ret.try_into().unwrap()
    }
}
impl TryFrom<QuadNode> for FilledNode {
    type Error = &'static str;

    fn try_from(node: QuadNode) -> Result<Self, Self::Error> {
        match node {
            QuadNode::Leaf(None) => Err("tree contains empty leaf"),
            QuadNode::Leaf(Some(p_i)) => Ok(FilledNode::Leaf(p_i)),
            QuadNode::Internal(children) => Ok(FilledNode::Internal(children)),
        }
    }
}
impl TryFrom<QuadTree> for FilledTree {
    type Error = &'static str;

    fn try_from(tree: QuadTree) -> Result<Self, Self::Error> {
        Ok(Self {
            domain: tree.domain,
            points: tree.points,
            nodes: tree
                .nodes
                .into_iter()
                .map(FilledNode::try_from)
                .collect::<Result<_, _>>()?,
        })
    }
}
