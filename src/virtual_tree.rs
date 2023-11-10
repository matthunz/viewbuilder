use crate::Tree;
use dioxus::core::Mutations;
use dioxus::prelude::VirtualDom;
use dioxus_native_core::exports::shipyard::Component;
use dioxus_native_core::node_ref::*;
use dioxus_native_core::prelude::*;
use dioxus_native_core_macro::partial_derive_state;

#[derive(Clone, Copy, Default, PartialEq, Eq, Debug)]
pub struct DynAttribute {}

impl FromAnyValue for DynAttribute {
    fn from_any_value(_value: &dyn std::any::Any) -> Self {
        todo!()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Default, Component)]
struct FlexDirectionAttr(DynAttribute);

#[partial_derive_state]
impl State<DynAttribute> for FlexDirectionAttr {
    // TextColor depends on the TextColor part of the parent
    type ParentDependencies = (Self,);

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
        let direction: DynAttribute = node_view
            .attributes()
            .and_then(|mut attrs| {
                attrs.next().map(|a| {
                    if a.attribute.name == "flex_direction" {
                        let direction: &DynAttribute = a.value.as_custom().unwrap();
                        direction.clone()
                    } else {
                        todo!()
                    }
                })
            })
            .unwrap_or_default();

        // check if the node contians a border attribute
        let new = Self(direction);
        // check if the member has changed
        let changed = new != *self;
        *self = new;
        changed
    }
}

pub struct VirtualTree {
    tree: Tree,
    vdom: VirtualDom,
    rdom: RealDom<DynAttribute>,
    state: DioxusState,
}

impl VirtualTree {
    pub fn new(app: dioxus::prelude::Component) -> Self {
        let mut vdom = VirtualDom::new(app);
        let mut tree = Tree::default();
        let mut rdom = RealDom::new([FlexDirectionAttr::to_type_erased()]);
        let mut state = DioxusState::create(&mut rdom);

        let mutations = vdom.rebuild();
        apply(&mut rdom, &mut tree, &mut state, mutations);

        Self {
            tree,
            vdom,
            state,
            rdom,
        }
    }

    pub async fn run(mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.vdom.wait_for_work().await;

        let mutations = self.vdom.render_immediate();
        apply(&mut self.rdom, &mut self.tree, &mut self.state, mutations);

        Ok(())
    }
}

fn apply(
    rdom: &mut RealDom<DynAttribute>,
    tree: &mut Tree,
    state: &mut DioxusState,
    mutations: Mutations,
) {
    state.apply_mutations(rdom, mutations);

    let ctx = SendAnyMap::new();
    let (updates, _) = rdom.update_state(ctx);

    for id in updates {
        let node = rdom.get(id).unwrap();
        let node_type = node.node_type();
        match &*node_type {
            NodeType::Text(text_node) => tree.insert_text_element(node.id(), &text_node.text),
            NodeType::Element(elem) => {
                tree.insert_element(node.id(), &elem.tag, &elem.attributes);
            }
            NodeType::Placeholder => todo!(),
        }
    }
}
