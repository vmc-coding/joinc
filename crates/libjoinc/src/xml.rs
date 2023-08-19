// Very simple XML wrapper which only supports the stuff we need.

use crate::error::{Error, Result};
use quick_xml::events::BytesStart;
use quick_xml::events::Event;
use quick_xml::reader::Reader;
use std::fmt;

pub const BOINC_GUI_RPC_REQUEST_TAG: &str = "boinc_gui_rpc_request";
pub const BOINC_GUI_RPC_REPLY_TAG: &str = "boinc_gui_rpc_reply";

pub type Tag = String;

#[derive(Debug, Default, PartialEq)]
pub struct Content {
    data: String,
}

#[derive(Debug)]
pub struct Node {
    pub tag: Tag,
    pub content: Content,
    pub children: Vec<Node>,
}

// ---- impl Content ----

impl Content {
    fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}

impl fmt::Display for Content {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.data)
    }
}

impl From<bool> for Content {
    fn from(data: bool) -> Self {
        (data as i32).into()
    }
}

impl From<i32> for Content {
    fn from(data: i32) -> Self {
        format!("{}", data).into()
    }
}

impl From<f64> for Content {
    fn from(data: f64) -> Self {
        format!("{}", data).into()
    }
}

impl From<String> for Content {
    fn from(data: String) -> Self {
        Content { data }
    }
}

impl From<&str> for Content {
    fn from(data: &str) -> Self {
        data.to_string().into()
    }
}

impl TryFrom<&Content> for bool {
    type Error = Error;

    fn try_from(content: &Content) -> Result<Self> {
        i32::try_from(content).map(|i| i != 0)
    }
}

impl TryFrom<&Content> for i32 {
    type Error = Error;

    fn try_from(content: &Content) -> Result<Self> {
        content
            .data
            .parse::<Self>()
            .map_err(|_| Error::InvalidFormat)
    }
}

impl TryFrom<&Content> for f64 {
    type Error = Error;

    fn try_from(content: &Content) -> Result<Self> {
        content
            .data
            .parse::<Self>()
            .map_err(|_| Error::InvalidFormat)
    }
}

impl<'a> TryFrom<&'a Content> for &'a str {
    type Error = Error;

    fn try_from(content: &'a Content) -> Result<Self> {
        Ok(&content.data)
    }
}

// TODO We're always storing the result as string in some response struct
// so let's try to move it instead of copying it
impl TryFrom<&Content> for String {
    type Error = Error;

    fn try_from(content: &Content) -> Result<Self> {
        Ok(content.data.clone())
    }
}

// ---- impl Node ----

impl Node {
    pub fn new<T: Into<Tag>>(tag: T) -> Self {
        Node {
            tag: tag.into(),
            content: Content::default(),
            children: vec![],
        }
    }

    pub fn with_content<C: Into<Content>>(mut self, content: C) -> Self {
        self.content = content.into();
        self
    }

    pub fn add_child(&mut self, child: Node) -> &mut Self {
        self.children.push(child);
        self
    }

    pub fn add_new_child<T: Into<Tag>>(&mut self, tag: T) -> &mut Self {
        self.add_child(Node::new(tag));
        self.children
            .last_mut()
            .expect("Just added node should exist")
    }

    pub fn add_new_content_child<T: Into<Tag>, C: Into<Content>>(
        &mut self,
        tag: T,
        content: C,
    ) -> &mut Self {
        self.add_child(Node::new(tag).with_content(content));
        self
    }

    pub fn find_child<T: Into<Tag>>(&self, tag: T) -> Option<&Node> {
        let to_find: String = tag.into();
        self.children.iter().find(|child| child.tag == to_find)
    }

    pub fn pretty_print(&self, indention_width: usize) -> String {
        self.serialize(indention_width, 0)
    }

    pub fn try_into_content<'a, C: std::convert::TryFrom<&'a Content, Error = Error>>(
        &'a self,
    ) -> Result<C> {
        C::try_from(&self.content)
    }

    pub fn try_child_into_content<
        'a,
        C: std::convert::TryFrom<&'a Content, Error = Error>,
        T: Into<Tag>,
    >(
        &'a self,
        child_tag: T,
    ) -> Result<C> {
        self.find_child(child_tag)
            .map(|c| c.try_into_content())
            .ok_or(Error::InvalidFormat)?
    }

    fn serialize(&self, indention_width: usize, indention_level: usize) -> String {
        let indention = " ".repeat(indention_level * indention_width);
        let mut txt = String::new();

        if !self.children.is_empty() {
            txt = txt + &format!("{}<{}>{}\n", indention, self.tag, self.content);
            for child in &self.children {
                txt = txt + &child.serialize(indention_width, indention_level + 1) + "\n";
            }
            txt = txt + &format!("{}</{}>", indention, self.tag);
        } else if self.content.is_empty() {
            // From https://boinc.berkeley.edu/trac/wiki/GuiRpcProtocol (Feb 17)
            // "Self-closing tags must not have a space before the slash,
            // or current client and server will not parse it correctly."
            txt = txt + &format!("{}<{}/>", indention, self.tag);
        } else {
            txt = txt + &format!("{}<{}>{}</{}>", indention, self.tag, self.content, self.tag);
        }

        txt
    }
}

pub fn boinc_request_tree() -> Node {
    Node::new(BOINC_GUI_RPC_REQUEST_TAG)
}

pub fn parse(xml: &[u8]) -> Result<Node> {
    let mut root_node: Option<Node> = None;
    let mut reader =
        Reader::from_str(std::str::from_utf8(xml).map_err(|_| Error::UnsupportedEncoding)?);

    loop {
        match reader
            .read_event()
            .map_err(|err| Error::InvalidXml(format!("The XML could not be parsed: {}.", err)))?
        {
            Event::Start(e) => match root_node {
                Some(_) => {
                    return Err(Error::InvalidXml(
                        "The XML contains more than one root element.".to_string(),
                    ))
                }
                None => {
                    root_node = Some(parse_node(&mut reader, Node::new(parse_tag(e)?))?);
                }
            },
            Event::Empty(e) => match root_node {
                Some(_) => {
                    return Err(Error::InvalidXml(
                        "The XML contains more than one root element.".to_string(),
                    ))
                }
                None => {
                    root_node = Some(Node::new(parse_tag(e)?));
                }
            },
            Event::Eof => break,
            Event::Comment(_) | Event::Decl(_) | Event::PI(_) | Event::DocType(_) => continue,
            Event::Text(mut txt) => {
                // ignore new lines
                if !txt.inplace_trim_start() {
                    return Err(Error::InvalidXml(
                        "Thx XML contains unexpected text on root level.".to_string(),
                    ));
                }
            }
            _ => {
                return Err(Error::InvalidXml(
                    "Thx XML contains at least one unexpected node on root level.".to_string(),
                ))
            }
        }
    }

    root_node.ok_or(Error::InvalidXml(
        "The XML doesn't contain a valid root element.".to_string(),
    ))
}

fn parse_node(reader: &mut Reader<&[u8]>, mut node: Node) -> Result<Node> {
    loop {
        match reader
            .read_event()
            .map_err(|err| Error::InvalidXml(format!("The XML could not be parsed: {}.", err)))?
        {
            Event::Start(e) => {
                node.add_child(parse_node(reader, Node::new(parse_tag(e)?))?);
            }
            Event::Empty(e) => {
                node.add_child(Node::new(parse_tag(e)?));
            }
            Event::Text(e) => {
                node.content = e
                    .unescape()
                    .map(|txt| txt.into_owned().trim().into())
                    .map_err(|txt| {
                        Error::InvalidXml(format!("The XML contains invalid content: '{}'.", txt))
                    })?;
            }
            Event::CData(e) => {
                node.content = String::from_utf8_lossy(&e.into_inner()).to_string().into();
            }
            Event::End(_) => return Ok(node),
            Event::Eof => {
                return Err(Error::InvalidXml(format!(
                    "The XML is missing a closing tag for {}.",
                    node.tag
                )))
            }
            _ => continue,
        }
    }
}

fn parse_tag(b: BytesStart) -> Result<String> {
    String::from_utf8_lossy(b.local_name().into_inner())
        .split(' ')
        .next()
        .map(|tag| tag.to_string())
        .ok_or(Error::InvalidXml(
            "The XML contains an invalid tag".to_string(),
        ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize_empty_node() {
        let node = Node::new("foo");
        assert_eq!(node.pretty_print(0), "<foo/>");
    }

    #[test]
    fn serialize_node_with_content() {
        let node = Node::new("foo").with_content("bar");
        assert_eq!(node.pretty_print(0), "<foo>bar</foo>");
    }

    #[test]
    fn serialize_node_with_child() {
        let mut node = Node::new("foo");
        node.add_new_child("bar");

        assert_eq!(node.pretty_print(0), "<foo>\n<bar/>\n</foo>");
    }

    #[test]
    fn deserialize_empty_node() {
        let xml = "<foo/>";
        let tree = parse(xml.as_bytes()).expect("Should be parseable");
        assert_eq!(tree.pretty_print(0), xml);
    }

    #[test]
    fn deserialize_tree_with_content() {
        let xml = "<foo>bar</foo>";
        let tree = parse(xml.as_bytes()).expect("Should be parseable");
        assert_eq!(tree.pretty_print(0), xml);
    }

    #[test]
    fn deserialize_tree_with_child() {
        let xml = "<a>\n<b/>\n</a>";
        let tree = parse(xml.as_bytes()).expect("Should be parseable");
        assert_eq!(tree.pretty_print(0), xml);
    }

    #[test]
    fn deserialize_tree_with_childs() {
        let xml = "<a>\n<b>\n<c>d</c>\n</b>\n</a>";
        let tree = parse(xml.as_bytes()).expect("Should be parseable");
        assert_eq!(tree.pretty_print(0), xml);
    }

    #[test]
    fn deserialize_cdata() {
        let xml = "<foo><![CDATA[ <bar> ]]></foo>";
        let tree = parse(xml.as_bytes()).expect("Should be parseable");
        assert_eq!(tree.content.data, " <bar> ".to_string());
    }

    #[test]
    fn dont_deserialize_without_root() {
        let xml = "<?foo?>";
        let result = parse(xml.as_bytes());
        assert!(result.is_err());
    }

    #[test]
    fn dont_deserialize_invalid_root() {
        let xml = "<foo>";
        let result = parse(xml.as_bytes());
        assert!(result.is_err());
    }

    #[test]
    fn dont_deserialize_invalid_tree() {
        let xml = "<foo><bar/>";
        let result = parse(xml.as_bytes());
        assert!(result.is_err());
    }

    #[test]
    fn dont_deserialize_with_unopened_closing_tags() {
        let xml = "<foo></bar>";
        let result = parse(xml.as_bytes());
        assert!(result.is_err());
    }

    #[test]
    fn set_boolean_content() {
        let node = Node::new("foo").with_content(false);
        assert_eq!(node.pretty_print(0), "<foo>0</foo>");
        let node = Node::new("foo").with_content(true);
        assert_eq!(node.pretty_print(0), "<foo>1</foo>");
    }

    #[test]
    fn set_integer_content() {
        let node = Node::new("foo").with_content(1234);
        assert_eq!(node.pretty_print(0), "<foo>1234</foo>");
    }

    #[test]
    fn set_floating_point_content() {
        let node = Node::new("foo").with_content(1234.56789);
        assert_eq!(node.pretty_print(0), "<foo>1234.56789</foo>");
    }

    #[test]
    fn set_string_content() {
        let node = Node::new("foo").with_content("bar");
        assert_eq!(node.pretty_print(0), "<foo>bar</foo>");
    }

    #[test]
    fn get_boolean_content() {
        let xml = "<foo>0</foo>";
        let tree = parse(xml.as_bytes()).expect("Should be parseable");
        assert_eq!(tree.try_into_content::<bool>().unwrap(), false);

        let xml = "<foo>1</foo>";
        let tree = parse(xml.as_bytes()).expect("Should be parseable");
        assert_eq!(tree.try_into_content::<bool>().unwrap(), true);
    }

    #[test]
    fn get_integer_content() {
        let xml = "<foo>1234</foo>";
        let tree = parse(xml.as_bytes()).expect("Should be parseable");
        assert_eq!(tree.try_into_content::<i32>().unwrap(), 1234);
    }

    #[test]
    fn get_float_content() {
        let xml = "<foo>1234.56789</foo>";
        let tree = parse(xml.as_bytes()).expect("Should be parseable");
        assert_eq!(tree.try_into_content::<f64>().unwrap(), 1234.56789);
    }

    #[test]
    fn get_string_content() {
        let xml = "<foo>bar</foo>";
        let tree = parse(xml.as_bytes()).expect("Should be parseable");
        assert_eq!(tree.try_into_content::<&str>().unwrap(), "bar");
    }
}
