use crate::layout::FlexDirection;
use crate::layout::LayoutComponent;
use crate::LayoutTree;
use crate::Tree;
use dioxus::core::Mutations;
use dioxus::prelude::VirtualDom;
use dioxus_native_core::node_ref::*;
use dioxus_native_core::prelude::*;
use dioxus_native_core_macro::partial_derive_state;
use quadtree_rs::Quadtree;
use shipyard::{Component, EntityId};
use std::any::Any;
use std::sync::Arc;
use std::sync::Mutex;
use taffy::prelude::Size;
use taffy::style::Style;
use taffy::style_helpers::TaffyMaxContent;
use taffy::Taffy;

#[derive(Clone, Default, PartialEq, Component)]
pub struct StyleComponent(pub Style);

#[derive(Clone, Default)]
pub struct DynAttribute(pub Option<Arc<dyn Any + Send + Sync>>);

impl PartialEq for DynAttribute {
    fn eq(&self, _other: &Self) -> bool {
        // TODO
        false
    }
}

impl FromAnyValue for DynAttribute {
    fn from_any_value(value: &dyn std::any::Any) -> Self {
        value.downcast_ref::<Self>().unwrap().clone()
    }
}

#[partial_derive_state]
impl State<DynAttribute> for StyleComponent {
    // TextColor depends on the TextColor part of the parent
    type ParentDependencies = ();

    type ChildDependencies = ();

    type NodeDependencies = ();

    // Border does not depended on any other member in the current node
    const NODE_MASK: NodeMaskBuilder<'static> =
        // Get access to the border attribute
        NodeMaskBuilder::new().with_attrs(AttributeMaskBuilder::Some(&["flex_direction"]));

    fn update<'a>(
        &mut self,
        node_view: NodeView<DynAttribute>,
        _node: <Self::NodeDependencies as Dependancy>::ElementBorrowed<'a>,
        _parent: Option<<Self::ParentDependencies as Dependancy>::ElementBorrowed<'a>>,
        _children: Vec<<Self::ChildDependencies as Dependancy>::ElementBorrowed<'a>>,
        _context: &SendAnyMap,
    ) -> bool {
        let mut style = Style::default();

        for attr in node_view.attributes().into_iter().flatten() {
            if attr.attribute.name == "flex_direction" {
                let i = attr.value.as_int().unwrap();
                let n = u8::try_from(i).unwrap();
                let flex_direction: FlexDirection = n.try_into().unwrap();
                style.flex_direction = flex_direction.into();
            } else {
                todo!()
            }
        }

        let new = Self(style);
        let changed = new != *self;
        *self = new;
        changed
    }
}

pub struct VirtualTree {
    pub tree: Arc<Mutex<Tree>>,
    vdom: VirtualDom,
    rdom: RealDom<DynAttribute>,
    state: DioxusState,
}

impl VirtualTree {
    pub fn new(app: dioxus::prelude::Component, tree: Arc<Mutex<Tree>>) -> Self {
        let mut vdom = VirtualDom::new(app);
        let mut rdom = RealDom::new([
            StyleComponent::to_type_erased(),
            LayoutComponent::to_type_erased(),
        ]);
        let mut state = DioxusState::create(&mut rdom);

        let mutations = vdom.rebuild();
        let taffy = tree.lock().unwrap().taffy.clone();
        apply(
            &mut rdom,
            &mut tree.lock().unwrap(),
            &mut state,
            mutations,
            taffy,
        );

        Self {
            tree,
            vdom,
            state,
            rdom,
        }
    }

    pub async fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.vdom.wait_for_work().await;

        let mutations = self.vdom.render_immediate();
        let taffy = self.tree.lock().unwrap().taffy.clone();
        apply(
            &mut self.rdom,
            &mut self.tree.lock().unwrap(),
            &mut self.state,
            mutations,
            taffy,
        );

        Ok(())
    }
}

fn apply(
    rdom: &mut RealDom<DynAttribute>,
    tree: &mut Tree,
    state: &mut DioxusState,
    mutations: Mutations,
    taffy: Arc<Mutex<Taffy>>,
) {
    state.apply_mutations(rdom, mutations);

    let mut cx = SendAnyMap::new();
    cx.insert(taffy.clone());

    let (_, changes) = rdom.update_state(cx);
    for (id, mask) in changes.iter() {
        let node = rdom.get(*id).unwrap();
        if tree.slots.contains_key(id) {
            tree.update(id.clone(), node, mask.clone(), &taffy);
        } else {
            tree.insert(node, &taffy);
        }
    }

    // Compute the new layout
    let mut taffy_ref = taffy.lock().unwrap();
    let root_layout_key = rdom
        .get(rdom.root_id())
        .unwrap()
        .get::<LayoutComponent>()
        .unwrap()
        .key
        .unwrap();
    taffy::compute_layout(&mut taffy_ref, root_layout_key, Size::MAX_CONTENT).unwrap();

    // TODO
    tree.layout.quadtree = Quadtree::new(20);

    // Convert the new relative layouts to global layouts
    rdom.traverse_depth_first(|node| {
        let layout = node.get::<LayoutComponent>().unwrap();
        let layout_key = layout.key.unwrap();

        let mut layout = taffy_ref.layout(layout_key).unwrap().clone();
        if let Some(parent_id) = node.parent_id() {
            let parent_layout = tree.slots[&parent_id].layout;
            layout.location.x += parent_layout.location.x;
            layout.location.y += parent_layout.location.y;
        }
        tree.slots.get_mut(&node.id()).unwrap().layout = layout;

        tree.layout.insert(
            [layout.location.x as _, layout.location.y as _],
            [layout.size.width as _, layout.size.height as _],
            node.id(),
        );
    });
}
