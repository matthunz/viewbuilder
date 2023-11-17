use dioxus::core::BorrowedAttributeValue;

use super::VirtualElement;
use crate::{any_element::AnyElement, element::Text, virtual_tree::VirtualNode, Element};

pub struct VirtualText {}

impl VirtualElement for VirtualText {
    fn from_vnode(&self, node: &VirtualNode) -> Box<dyn Element> {
        if let VirtualNode::Element {
            tag: _,
            attrs,
            children,
        } = node
        {
            let mut text = Text::builder();

            for child in children {
                if let VirtualNode::Text(s) = child {
                    text.content(s.clone());
                }
            }

            for _attr in attrs {}

            Box::new(text.build())
        } else {
            todo!()
        }
    }

    fn set_attribute(
        &self,
        name: &str,
        value: BorrowedAttributeValue,
        element: &mut dyn AnyElement,
    ) {
        let text: &mut Text = element.as_any_mut().downcast_mut().unwrap();
        match name {
            "font_size" => {
                let font_size = if let BorrowedAttributeValue::Float(n) = value {
                    n
                } else {
                    todo!()
                };
                text.set_font_size(font_size as _)
            }
            _ => {}
        }
    }

    fn set_handler(
        &self,
        name: &str,
        mut handler: Box<dyn FnMut() + Send>,
        element: &mut dyn AnyElement,
    ) {
        let text: &mut Text = element.as_any_mut().downcast_mut().unwrap();

        match name {
            "click" => text.set_on_click(move |_| handler()),
            _ => {}
        }
    }
}
