use dioxus_native_core::prelude::*;
use dioxus_native_core_macro::partial_derive_state;
use quadtree_rs::{
    area::{Area, AreaBuilder},
    point::Point,
    Quadtree,
};
use shipyard::{Component, EntityId};
use slotmap::DefaultKey;
use std::sync::{Arc, Mutex};
use taffy::{
    style::{Dimension, Style},
    Taffy,
};

mod flex_direction;
pub use flex_direction::FlexDirection;

use crate::virtual_tree::DynAttribute;

#[derive(Clone, Default, Debug, Component)]
pub struct LayoutComponent {
    pub style: Style,
    pub key: Option<DefaultKey>,
}

#[partial_derive_state]
impl State<DynAttribute> for LayoutComponent {
    type ChildDependencies = (Self,);
    type ParentDependencies = ();
    type NodeDependencies = ();

    const NODE_MASK: NodeMaskBuilder<'static> =
        NodeMaskBuilder::new().with_attrs(AttributeMaskBuilder::Some(&[
            "flex_direction",
            "width",
            "height",
        ]));

    fn update<'a>(
        &mut self,
        node_view: NodeView<DynAttribute>,
        _: <Self::NodeDependencies as Dependancy>::ElementBorrowed<'a>,
        _: Option<<Self::ParentDependencies as Dependancy>::ElementBorrowed<'a>>,
        children: Vec<<Self::ChildDependencies as Dependancy>::ElementBorrowed<'a>>,
        context: &SendAnyMap,
    ) -> bool {
        let taffy: &Arc<Mutex<Taffy>> = context.get().unwrap();
        let mut taffy = taffy.lock().unwrap();

        let mut style = Style::default();
        for attr in node_view.attributes().into_iter().flatten() {
            if attr.attribute.name == "flex_direction" {
                let i = attr.value.as_int().unwrap();
                let n = u8::try_from(i).unwrap();
                let flex_direction: FlexDirection = n.try_into().unwrap();
                style.flex_direction = flex_direction.into();
            } else if attr.attribute.name == "width" {
                style.size.width = Dimension::Points(attr.value.as_float().unwrap() as _);
            } else if attr.attribute.name == "height" {
                style.size.height = Dimension::Points(attr.value.as_float().unwrap() as _);
            } else {
                todo!()
            }
        }

        let mut is_changed = self.style != style;

        let mut child_layout = vec![];
        for (child,) in children {
            child_layout.push(child.key.unwrap());
        }

        if let Some(key) = self.key {
            if taffy.children(key).unwrap() != child_layout {
                taffy.set_children(key, &child_layout).unwrap();
                is_changed = true;
            }

            if is_changed {
                taffy.set_style(key, style.clone()).unwrap();
            }
        } else {
            self.key = Some(
                taffy
                    .new_with_children(style.clone(), &child_layout)
                    .unwrap(),
            );
            is_changed = true;
        }

        if is_changed {
            self.style = style;
        }

        is_changed
    }
}

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
