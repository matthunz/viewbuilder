#![allow(non_snake_case)]

use dioxus::prelude::VirtualDom;
use dioxus_native_core::exports::shipyard::Component;
use dioxus_native_core::node::OwnedAttributeDiscription;
use dioxus_native_core::node::OwnedAttributeValue;
use dioxus_native_core::node_ref::*;
use dioxus_native_core::prelude::*;

use dioxus_native_core_macro::partial_derive_state;
use skia_safe::Typeface;

use std::borrow::Cow;
use std::collections::HashMap;
use std::hash::BuildHasherDefault;

use crate::element::Element;
use crate::factory::ViewFactory;
use crate::text_factory::TextElementFactory;
use crate::Factory;
use crate::TextFactory;

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
    elements: HashMap<Cow<'static, str>, Box<dyn Factory>>,
    text_factory: Box<dyn TextFactory>,
}

impl Default for VirtualTree {
    fn default() -> Self {
        let mut me = Self {
            elements: Default::default(),
            text_factory: Box::new(TextElementFactory {}),
        };
        me.insert_element("view", ViewFactory {});
        me.insert_element("Root", ViewFactory {});
        me
    }
}

impl VirtualTree {
    pub fn insert_element(
        &mut self,
        tag: impl Into<Cow<'static, str>>,
        element: impl Factory + 'static,
    ) {
        self.elements.insert(tag.into(), Box::new(element));
    }

    pub fn create_element(
        &mut self,
        tag: &str,
        attrs: &HashMap<
            OwnedAttributeDiscription,
            OwnedAttributeValue<DynAttribute>,
            BuildHasherDefault<rustc_hash::FxHasher>,
        >,
    ) -> Option<Box<dyn Element>> {
        self.elements
            .get_mut(tag)
            .map(|elem| elem.from_attrs(attrs))
    }

    pub fn create_text_element(&mut self, text: &str) -> Box<dyn Element> {
        self.text_factory.create_text(text)
    }

    pub fn run(
        mut self,
        app: dioxus::prelude::Component,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // create the vdom, the real_dom, and the binding layer between them
        let mut vdom = VirtualDom::new(app);
        let mut rdom: RealDom<DynAttribute> = RealDom::new([FlexDirectionAttr::to_type_erased()]);
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
                        let _indent = " ".repeat(node.height() as usize);
                        let _border = *node.get::<FlexDirectionAttr>().unwrap();

                        let _id = node.id();

                        let node_type = node.node_type();

                        let typeface = Typeface::new("monospace", Default::default()).unwrap();
                        let _font = skia_safe::Font::new(typeface, Some(100.));
                        let _elem = match &*node_type {
                            NodeType::Text(text_node) => self.create_text_element(&text_node.text),
                            NodeType::Element(elem) => {
                                dbg!(&elem.tag);
                                self.create_element(&elem.tag, &elem.attributes).unwrap()
                            }
                            NodeType::Placeholder => todo!(),
                        };
                    });
                }
            })
    }
}
