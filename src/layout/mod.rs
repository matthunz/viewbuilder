use crate::virtual_tree::DynAttribute;
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
use taffy::{style::Style, Taffy};

mod dimension;
pub use dimension::{Dimension, IntoDimension};

mod flex_direction;
pub use flex_direction::FlexDirection;

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
        for attr_view in node_view.attributes().into_iter().flatten() {
            match &*attr_view.attribute.name {
                "flex_direction" => {
                    let i = attr_view.value.as_int().unwrap();
                    let n = u8::try_from(i).unwrap();
                    let flex_direction: FlexDirection = n.try_into().unwrap();
                    style.flex_direction = flex_direction.into();
                }
                "width" => {
                    style.size.width = Dimension::from_value(attr_view.value).into_taffy(2.);
                }
                "height" => {
                    style.size.height = Dimension::from_value(attr_view.value).into_taffy(2.);
                }
                _ => unimplemented!(),
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

pub struct LayoutTree {
    pub(crate) quadtree: Quadtree<i64, EntityId>,
    taffy: Taffy,
}

impl LayoutTree {
    pub fn new(depth: usize) -> Self {
        Self {
            quadtree: Quadtree::new(depth),
            taffy: Taffy::new(),
        }
    }

    pub fn insert(&mut self, point: [f64; 2], size: [f64; 2], id: EntityId) {
        self.quadtree.insert(area(point, size), id);
    }

    pub fn query(&self, point: [f64; 2]) -> impl Iterator<Item = EntityId> + '_ {
        let point = point.map(to_rounded);
        let area = AreaBuilder::default()
            .anchor(Point {
                x: point[0],
                y: point[1],
            })
            .build()
            .unwrap();
        self.quadtree.query(area).map(|i| i.value_ref().clone())
    }
}

fn area(point: [f64; 2], size: [f64; 2]) -> Area<i64> {
    let point = point.map(to_rounded);
    let size = size.map(|n| to_rounded(n).max(1));
  
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
    (n * 100.).round() as _
}

/*
fn from_rounded(i: i64) -> f64 {
    i as f64 / 10_000.
}
*/
