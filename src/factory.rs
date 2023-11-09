use crate::{element::{Element, View}, virtual_tree::DynAttribute};
use dioxus_native_core::node::{OwnedAttributeDiscription, OwnedAttributeValue};
use std::{collections::HashMap, hash::BuildHasherDefault};

pub trait Factory {
    fn from_attrs(
        &mut self,
        attrs: &HashMap<
            OwnedAttributeDiscription,
            OwnedAttributeValue<DynAttribute>,
            BuildHasherDefault<rustc_hash::FxHasher>,
        >,
    ) -> Box<dyn Element>;
}


pub struct ViewFactory {

}

impl Factory for ViewFactory {
    fn from_attrs(
        &mut self,
        attrs: &HashMap<
            OwnedAttributeDiscription,
            OwnedAttributeValue<DynAttribute>,
            BuildHasherDefault<rustc_hash::FxHasher>,
        >,
    ) -> Box<dyn Element> {
        Box::new(View {

        })
        
    }
}
