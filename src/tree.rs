use std::{collections::VecDeque, vec};

use rand::prelude::*;

use crate::point::{self, *};

type NodeId = usize;

#[derive(Debug, Clone)]
struct Square {
    mid: Point,
    rad: Coord,
}
impl Square {
    const UNIT: Self = Self {
        mid: Point::new(0.5, 0.5),
        rad: 0.5,
    };

    fn new(mid: Point, rad: Coord) -> Self {
        assert!(rad > 0.0);
        Self { mid, rad }
    }

    fn x_lo(&self) -> Coord {
        self.mid.x() - self.rad
    }
    fn x_hi(&self) -> Coord {
        self.mid.x() + self.rad
    }
    fn y_lo(&self) -> Coord {
        self.mid.y() - self.rad
    }
    fn y_hi(&self) -> Coord {
        self.mid.y() + self.rad
    }

    fn sample(&self, rng: &mut impl rand::Rng) -> Point {
        Point::new(
            rng.random_range(self.x_lo()..self.x_hi()),
            rng.random_range(self.y_lo()..self.y_hi()),
        )
    }

    /// y-up:
    /// 2 3
    /// 0 1
    fn children(&self) -> [Self; 4] {
        let half_rad = self.rad / 2.0;
        [
            Square::new(
                Point::new(self.mid.x() - half_rad, self.mid.y() - half_rad),
                half_rad,
            ),
            Square::new(
                Point::new(self.mid.x() + half_rad, self.mid.y() - half_rad),
                half_rad,
            ),
            Square::new(
                Point::new(self.mid.x() - half_rad, self.mid.y() + half_rad),
                half_rad,
            ),
            Square::new(
                Point::new(self.mid.x() + half_rad, self.mid.y() + half_rad),
                half_rad,
            ),
        ]
    }

    /// y-up:
    /// 2 3
    /// 0 1
    fn child_i_of_point(&self, point: Point) -> usize {
        match (point.x() < self.mid.x(), point.y() < self.mid.y()) {
            (true, true) => 0,
            (false, true) => 1,
            (true, false) => 2,
            (false, false) => 3,
        }
    }

    fn contains(&self, point: Point) -> bool {
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
enum TreeShape {
    Leaf,
    Internal(Box<[TreeShape; 4]>),
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
enum Node {
    Leaf(Option<PointId>),
    /// y-up:
    /// 2 3
    /// 0 1
    Internal([NodeId; 4]),
}
#[derive(Debug, Clone)]
pub(crate) struct Tree {
    domain: Square,
    points: Vec<Point>,
    /// root is at index 0
    /// probably should have parents have lower index than their children
    /// probably should have children are contiguous
    nodes: Vec<Node>,
}
impl Default for Tree {
    fn default() -> Self {
        Self {
            domain: Square::default(),
            points: Vec::new(),
            nodes: vec![Node::Leaf(None)],
        }
    }
}
impl Tree {
    // pub(crate) fn from_domain(domain: Square) -> Self {
    //     Self {
    //         domain,
    //         points: Vec::new(),
    //         nodes: Vec::new(),
    //     }
    // }

    pub(crate) fn from_points(points: Vec<Point>) -> Self {
        let mut ret = Self::default();
        for point in points {
            ret.insert(point);
        }
        ret
    }

    /// already have p_i in points
    /// but this will increase the len of nodes
    fn insert_into_node(&mut self, p_i: PointId, node_i: NodeId, domain: &Square) {
        assert!(domain.contains(self.points[p_i]));
        match self.nodes[node_i] {
            Node::Leaf(None) => {
                // insert into the empty leaf
                self.nodes[node_i] = Node::Leaf(Some(p_i));
            }
            Node::Leaf(Some(leaf_point_i)) => {
                // split the leaf, then insert both points
                let node_len = self.nodes.len();
                self.nodes
                    .extend_from_slice(&[const { Node::Leaf(None) }; 4]);
                self.nodes[node_i] =
                    Node::Internal([node_len, node_len + 1, node_len + 2, node_len + 3]);
                self.insert_into_node(leaf_point_i, node_i, domain);
                self.insert_into_node(p_i, node_i, domain);
            }
            Node::Internal(children) => {
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
impl From<&TreeShape> for Tree {
    fn from(shape: &TreeShape) -> Self {
        let mut nodes = vec![Node::Leaf(None)];
        let mut frontier = VecDeque::from([(0, shape)]);

        while let Some((node_i, shape)) = frontier.pop_front() {
            // make the node at node_i match shape
            assert!(node_i < nodes.len());
            assert!(matches!(nodes[node_i], Node::Leaf(None)));
            match shape {
                TreeShape::Leaf => {}
                TreeShape::Internal(children_shapes) => {
                    let node_len = nodes.len();
                    nodes.extend_from_slice(&[const { Node::Leaf(None) }; 4]);
                    nodes[node_i] =
                        Node::Internal([node_len, node_len + 1, node_len + 2, node_len + 3]);
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

/// every internal node has four children
/// ie every leaf is filled
#[derive(Debug, Clone)]
enum FilledNode {
    Leaf(PointId),
    Internal([NodeId; 4]),
}
#[derive(Debug, Default, Clone)]
pub(crate) struct FilledTree {
    domain: Square,
    points: Vec<Point>,
    nodes: Vec<FilledNode>,
}
impl FilledTree {
    /// keep the same structure, but randomize where the points are in each leaf
    pub(crate) fn randomize(&mut self) {
        todo!()
    }

    pub(crate) fn from_tree_shape_random_filled(shape: &TreeShape, rng: &mut impl Rng) -> Self {
        let mut ret: Tree = shape.into();

        let mut frontier = vec![(0, Square::UNIT)];
        while let Some((node_i, domain)) = frontier.pop() {
            match &ret.nodes[node_i] {
                Node::Leaf(None) => {
                    let p_i = ret.points.len();
                    ret.points.push(domain.sample(rng));
                    ret.nodes[node_i] = Node::Leaf(Some(p_i));
                }
                Node::Leaf(Some(_)) => {
                    unreachable!()
                }
                Node::Internal(children) => {
                    for (&child_i, child_domain) in children.iter().zip(domain.children()) {
                        frontier.push((child_i, child_domain));
                    }
                }
            }
        }

        ret.try_into().unwrap()
    }
}
impl TryFrom<Node> for FilledNode {
    type Error = &'static str;

    fn try_from(node: Node) -> Result<Self, Self::Error> {
        match node {
            Node::Leaf(None) => Err("tree contains empty leaf"),
            Node::Leaf(Some(p_i)) => Ok(FilledNode::Leaf(p_i)),
            Node::Internal(children) => Ok(FilledNode::Internal(children)),
        }
    }
}
impl TryFrom<Tree> for FilledTree {
    type Error = &'static str;

    fn try_from(tree: Tree) -> Result<Self, Self::Error> {
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
