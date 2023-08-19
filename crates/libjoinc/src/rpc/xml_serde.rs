use crate::error::{Error, Result};
use crate::types::*;
use crate::xml;

// acts like the content/the children of self have to be rendered
// as child nodes of the given node, i.e. without opening a tag for self
pub trait SerializeInto {
    fn serialize_into(&self, node: xml::Node) -> xml::Node;
}

impl SerializeInto for Version {
    fn serialize_into(&self, mut node: xml::Node) -> xml::Node {
        node.add_new_content_child("major", self.major)
            .add_new_content_child("minor", self.minor)
            .add_new_content_child("release", self.release);
        node
    }
}

impl TryFrom<&xml::Node> for Version {
    type Error = Error;

    fn try_from(node: &xml::Node) -> Result<Self> {
        Ok(Version {
            major: node.try_child_into_content("major")?,
            minor: node.try_child_into_content("minor")?,
            release: node.try_child_into_content("release")?,
        })
    }
}
