use std::collections::HashMap;

use quick_xml::{
    events::{BytesStart, Event},
    reader::Reader,
};

#[derive(Debug)]
pub struct XMLItem {
    pub name: String,
    pub attributes: HashMap<String, String>,
    pub children: Vec<XMLItem>,
}

fn parse_tag(bytes_start: BytesStart) -> XMLItem {
    let name = String::from_utf8(bytes_start.local_name().as_ref().to_vec())
        .expect("Failed to parse tag name");
    let attributes = bytes_start
        .attributes()
        .map(|attribute| {
            let attribute = attribute.expect("Failed to read attribute");
            (
                String::from_utf8(attribute.key.local_name().as_ref().to_vec())
                    .expect("Failed to parse attribute name"),
                attribute
                    .unescape_value()
                    .expect("Failed to parse attribute value")
                    .to_string(),
            )
        })
        .collect::<HashMap<_, _>>();

    XMLItem {
        name,
        attributes,
        children: vec![],
    }
}

/// Recursively parses an XML structure
pub fn parse_xml(data: &str) -> XMLItem {
    let mut reader = Reader::from_str(data);
    let mut stack = vec![];

    loop {
        match reader.read_event().unwrap() {
            // Start tag contains attributes and has children
            Event::Start(bytes_start) => {
                let tag = parse_tag(bytes_start);
                stack.push(tag);
            }
            // End tag contains no attributes
            Event::End(_) => {
                let child = stack.pop().expect("Found end tag with no matching start!");
                if let Some(parent) = stack.last_mut() {
                    parent.children.push(child);
                } else {
                    stack.push(child);
                }
            }
            // Empty tag has attributes but no children
            Event::Empty(bytes_start) => {
                let tag = parse_tag(bytes_start);
                if let Some(parent) = stack.last_mut() {
                    parent.children.push(tag);
                }
            }
            Event::Eof => break,
            _ => {}
        }
    }

    assert_eq!(stack.len(), 1, "Only root node should be left.");
    stack.pop().unwrap()
}
