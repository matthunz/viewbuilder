use quadtree_rs::{
    area::{Area, AreaBuilder},
    point::Point,
    Quadtree,
};
use shipyard::EntityId;

pub struct Layout {
    quadtree: Quadtree<i64, EntityId>,
}

impl Layout {
    pub fn new(depth: usize) -> Self {
        Self {
            quadtree: Quadtree::new(depth),
        }
    }

    pub fn insert(&mut self, point: [f64; 2], size: [f64; 2], id: EntityId) {
        self.quadtree.insert(area(point, size), id);
    }

    pub fn query(&self, point: [f64; 2], size: [f64; 2]) -> impl Iterator<Item = EntityId> + '_ {
        self.quadtree
            .query(area(point, size))
            .map(|i| i.value_ref().clone())
    }
}

fn area(point: [f64; 2], size: [f64; 2]) -> Area<i64> {
    let point = point.map(to_rounded);
    let size = size.map(to_rounded);
    AreaBuilder::default()
        .anchor(Point {
            x: point[0],
            y: point[1],
        })
        .dimensions((size[0], size[1]))
        .build()
        .unwrap()
}

fn to_rounded(n: f64) -> i64 {
    (n * 10_000.).round() as _
}

fn from_rounded(i: i64) -> f64 {
    i as f64 / 10_000.
}
