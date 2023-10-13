use crate::{
    element::{TextElement, ViewElement},
    tree::Tree,
    Operation,
};
use dioxus::{
    core::{BorrowedAttributeValue, ElementId, Mutation},
    prelude::{Component, TemplateAttribute, TemplateNode, VirtualDom},
};
use skia_safe::{Font, Typeface};
use slotmap::DefaultKey;
use std::{collections::HashMap, fmt};

enum Attribute {
    Dynamic { id: usize },
}

enum Node {
    Text(String),
    Element {
        attrs: Vec<Attribute>,
        children: Vec<Self>,
    },
}

impl Node {
    fn from_template(template_node: &TemplateNode) -> Self {
        match template_node {
            TemplateNode::Text { text } => Node::Text(text.to_string()),
            TemplateNode::Element {
                tag: _,
                namespace: _,
                attrs,
                children,
            } => {
                let children = children.iter().map(Self::from_template).collect();
                let attrs = attrs
                    .into_iter()
                    .map(|attr| match attr {
                        TemplateAttribute::Dynamic { id } => Attribute::Dynamic { id: *id },
                        _ => todo!(),
                    })
                    .collect();
                Node::Element { attrs, children }
            }
            _ => todo!(),
        }
    }
}

struct Template {
    roots: Vec<Node>,
}

pub struct VirtualTree {
    pub(crate) vdom: VirtualDom,
    pub(crate) tree: Tree,
    templates: HashMap<String, Template>,
    elements: HashMap<ElementId, DefaultKey>,
    pub(crate) root: DefaultKey,
}

impl VirtualTree {
    pub fn new(app: Component) -> Self {
        let mut tree = Tree::default();
        let root = tree.insert(Box::new(ViewElement::new(Vec::new())));

        let mut elements = HashMap::new();
        elements.insert(ElementId(0), root);

        Self {
            vdom: VirtualDom::new(app),
            tree,
            templates: HashMap::new(),
            elements,
            root,
        }
    }

    pub fn rebuild(&mut self) {
        let mutations = self.vdom.rebuild();
        dbg!(&mutations);
        for template in mutations.templates {
            let roots = template.roots.iter().map(Node::from_template).collect();
            self.templates
                .insert(template.name.to_string(), Template { roots });
        }

        let mut stack = Vec::new();
        for edit in mutations.edits {
            match edit {
                Mutation::LoadTemplate { name, index, id } => {
                    let root = &self.templates[name].roots[index];
                    let key = insert(&mut self.tree, root);
                    self.elements.insert(id, key);
                    stack.push(key);
                }
                Mutation::AppendChildren { id, m } => {
                    let key = self.elements[&id];

                    for child_key in stack.splice(stack.len() - m.., []) {
                        self.tree.push_child(key, child_key)
                    }
                }
                Mutation::SetAttribute {
                    name,
                    value,
                    id,
                    ns,
                } => {
                    let key = self.elements[&id];
                    let elem = self.tree.get_mut(key);

                    match value {
                        BorrowedAttributeValue::Any(any) => elem.set_attr(name, &*any.as_any()),
                        _ => todo!(),
                    }
                }
                _ => todo!(),
            }
        }
    }
}

fn insert(tree: &mut Tree, node: &Node) -> DefaultKey {
    match node {
        Node::Text(text) => {
            let typeface = Typeface::new("Arial", Default::default()).unwrap();
            let font = Font::new(typeface, 100.);

            tree.insert(Box::new(TextElement::new(text.to_string(), &font)))
        }
        Node::Element { children, .. } => {
            let child_keys = children.iter().map(|child| insert(tree, child)).collect();
            tree.insert(Box::new(ViewElement::new(child_keys)))
        }
    }
}

impl fmt::Display for VirtualTree {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut stack = vec![Operation::Push(self.root)];

        let mut level = 0;
        while let Some(op) = stack.pop() {
            match op {
                Operation::Push(key) => {
                    let elem = self.tree.get(key);

                    for _ in 0..level {
                        write!(f, "    ")?;
                    }
                    writeln!(f, "{}", elem)?;

                    level += 1;
                    stack.push(Operation::Pop);
                    stack.extend(elem.children().into_iter().flatten().map(Operation::Push));
                }
                Operation::Pop => {
                    level -= 1;

                    for _ in 0..level {
                        write!(f, "    ")?;
                    }
                    writeln!(f, "}}")?;
                }
            }
        }

        Ok(())
    }
}
