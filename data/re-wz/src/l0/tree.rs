use std::collections::VecDeque;

use id_tree::{InsertBehavior, Node, Tree};

use crate::{file::WzIO, WzReader};

use super::{WzDirHeader, WzDirNode};

#[derive(Debug)]
pub struct WzTree {
    tree: Tree<WzDirNode>,
}

impl WzTree {
    pub fn read<R: WzIO>(r: &mut WzReader<R>) -> anyhow::Result<Self> {
        let mut tree = Tree::new();

        let root_id = tree.insert(
            Node::new(WzDirNode::Dir(WzDirHeader::root(1))),
            InsertBehavior::AsRoot,
        )?;
        let root = r.read_root_dir()?;
        let mut q = VecDeque::new();
        q.push_back((root_id, root));

        while let Some((parent_id, dir)) = q.pop_front() {
            for val in dir.entries.0.iter() {
                let new_node = tree
                    .insert(
                        Node::new(val.clone()),
                        InsertBehavior::UnderNode(&parent_id),
                    )
                    .unwrap();

                if let WzDirNode::Dir(dir) = val {
                    q.push_back((new_node.clone(), r.read_dir_node(dir)?));
                }
            }
        }

        Ok(Self { tree })
    }

    pub fn get_tree(&self) -> &Tree<WzDirNode> {
        &self.tree
    }
}
