#![allow(non_snake_case)]
use crate::element::TextElement;
use crate::element::ViewElement;
use crate::layout::FlexDirection;
use dioxus::prelude::*;
use dioxus_native_core::exports::shipyard::Component;
use dioxus_native_core::node_ref::*;
use dioxus_native_core::prelude::*;
use dioxus_native_core::real_dom::ViewEntry;
use dioxus_native_core_macro::partial_derive_state;
use skia_safe::Typeface;

impl FromAnyValue for FlexDirection {
    fn from_any_value(value: &dyn std::any::Any) -> Self {
        value.downcast_ref::<Self>().unwrap().clone()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Default, Component)]
struct FlexDirectionAttr(FlexDirection);

#[partial_derive_state]
impl State<FlexDirection> for FlexDirectionAttr {
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
        node_view: NodeView<FlexDirection>,
        _node: <Self::NodeDependencies as Dependancy>::ElementBorrowed<'a>,
        _parent: Option<<Self::ParentDependencies as Dependancy>::ElementBorrowed<'a>>,
        _children: Vec<<Self::ChildDependencies as Dependancy>::ElementBorrowed<'a>>,
        _context: &SendAnyMap,
    ) -> bool {
        let direction: FlexDirection = node_view
            .attributes()
            .and_then(|mut attrs| {
                attrs.next().map(|a| {
                    if a.attribute.name == "flex_direction" {
                        let direction: &FlexDirection = a.value.as_custom().unwrap();
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

pub fn run(app: dioxus::prelude::Component) -> Result<(), Box<dyn std::error::Error>> {
    // create the vdom, the real_dom, and the binding layer between them
    let mut vdom = VirtualDom::new(app);
    let mut rdom: RealDom<FlexDirection> = RealDom::new([FlexDirectionAttr::to_type_erased()]);
    let mut dioxus_intigration_state = DioxusState::create(&mut rdom);

    let mutations = vdom.rebuild();
    // update the structure of the real_dom tree
    dioxus_intigration_state.apply_mutations(&mut rdom, mutations);
    let ctx = SendAnyMap::new();
    // set the font size to 3.3
    // ctx.insert(FontSize(3.3));
    // update the State for nodes in the real_dom tree
    let _to_rerender = rdom.update_state(ctx);

    // we need to run the vdom in a async runtime
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()?
        .block_on(async {
            loop {
                // wait for the vdom to update
                vdom.wait_for_work().await;

                // get the mutations from the vdom
                let mutations = vdom.render_immediate();

                // update the structure of the real_dom tree
                dioxus_intigration_state.apply_mutations(&mut rdom, mutations);

                // update the state of the real_dom tree
                let ctx = SendAnyMap::new();
                // set the font size to 3.3
                // ctx.insert(FontSize(3.3));
                let _to_rerender = rdom.update_state(ctx);

                // render...
                rdom.traverse_depth_first_advanced(true, |node| {
                    let indent = " ".repeat(node.height() as usize);
                    let border = *node.get::<FlexDirectionAttr>().unwrap();

                    let id = node.id();

                    let node_type = node.node_type();

                    let typeface = Typeface::new("monospace", Default::default()).unwrap();
                    let font = skia_safe::Font::new(typeface, Some(100.));
                    let elem = match &*node_type {
                        NodeType::Text(text_node) => {
                            Box::new(TextElement::new(text_node.text.clone(), &font))
                        }
                        NodeType::Element(_) => todo!(),
                        NodeType::Placeholder => todo!(),
                    };
                });
            }
        })
}
