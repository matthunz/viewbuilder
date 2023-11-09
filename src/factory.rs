use crate::{element::Element, virtual_tree::DynAttribute};
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
