use std::collections::VecDeque;

use id_tree::{InsertBehavior, Node, Tree};

use crate::file::{WzIO, WzImgReader};

use super::{canvas::WzCanvas, obj::WzObject, prop::WzValue};

pub struct WzImgNode {
    pub name: String,
    pub canvas: Option<WzCanvas>,
}

impl WzImgNode {
    pub fn named(name: String) -> Self {
        Self { name, canvas: None }
    }

    pub fn canvas(canvas: WzCanvas) -> Self {
        Self {
            name: format!("[CANVAS]: {}", canvas.other_byte.is_some()),
            canvas: Some(canvas),
        }
    }
}

pub struct WzImgTree {
    tree: Tree<WzImgNode>,
}

impl WzImgTree {
    pub fn read<R>(r: &mut WzImgReader<R>) -> anyhow::Result<Self>
    where
        R: WzIO,
    {
        let mut tree = Tree::new();

        let obj = r.read_root_obj()?;

        let root_id = tree.insert(
            Node::new(WzImgNode::named("Root".to_string())),
            InsertBehavior::AsRoot,
        )?;
        let mut q = VecDeque::new();
        q.push_back((root_id, obj));

        while let Some((parent_id, obj)) = q.pop_front() {
            match obj {
                WzObject::Canvas(canvas) => {
                    tree.insert(
                        Node::new(WzImgNode::canvas(canvas)),
                        InsertBehavior::UnderNode(&parent_id),
                    )?;
                }
                WzObject::Property(prop) => {
                    for prop in prop.entries.0.iter() {
                        let val = match &prop.val {
                            WzValue::Null => "null".to_string(),
                            WzValue::Short1(v) | WzValue::Short2(v) => v.to_string(),
                            WzValue::Int1(v) | WzValue::Int2(v) => v.0.to_string(),
                            WzValue::Long(v) => v.0.to_string(),
                            WzValue::F32(v) => v.0.to_string(),
                            WzValue::F64(v) => v.to_string(),
                            WzValue::Str(v) => v.as_str().unwrap_or("UNICODE").to_string(),
                            WzValue::Obj(_v) => "obj".to_string(),
                        };

                        let prop_node = tree.insert(
                            Node::new(WzImgNode::named(format!(
                                "{}: {}",
                                prop.name.as_str().unwrap(),
                                val
                            ))),
                            InsertBehavior::UnderNode(&parent_id),
                        )?;

                        if let WzValue::Obj(ref obj) = prop.val {
                            let obj = r.read_obj(obj)?;
                            q.push_back((prop_node, obj));
                        }
                    }
                }
                WzObject::UOL(uol) => {
                    tree.insert(
                        Node::new(WzImgNode::named(format!(
                            "[UOL]: {}",
                            uol.entries.as_str().unwrap_or("")
                        ))),
                        InsertBehavior::UnderNode(&parent_id),
                    )?;
                }
                WzObject::Vec2(vec2) => {
                    tree.insert(
                        Node::new(WzImgNode::named(format!(
                            "[VEC2] x={},y={}",
                            vec2.x.0, vec2.y.0
                        ))),
                        InsertBehavior::UnderNode(&parent_id),
                    )?;
                }
            };
        }

        Ok(Self { tree })
    }

    pub fn get_tree(&self) -> &Tree<WzImgNode> {
        &self.tree
    }
}
