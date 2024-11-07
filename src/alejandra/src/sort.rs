use rnix::SyntaxElement;

pub(crate) fn sort_attr_set_entries(entries: &[SyntaxElement]) -> Vec<SyntaxElement> {
    let mut sorted = entries.to_vec();
    sorted.sort_by(|a, b| {
        let a_key = get_attr_key(a);
        let b_key = get_attr_key(b);
        
        // Keep "self" first
        if a_key == "self" { return std::cmp::Ordering::Less; }
        if b_key == "self" { return std::cmp::Ordering::Greater; }
        
        a_key.cmp(&b_key)
    });
    sorted
}

fn get_attr_key(element: &SyntaxElement) -> String {
    if let Some(node) = element.as_node() {
        match node.kind() {
            rnix::SyntaxKind::NODE_KEY_VALUE => {
                // Extract key from key-value pair
                if let Some(key_node) = node.children().find(|n| n.kind() == rnix::SyntaxKind::NODE_KEY) {
                    return key_node.text().to_string();
                }
            }
            rnix::SyntaxKind::NODE_INHERIT => {
                // Extract first inherited name
                if let Some(ident) = node
                    .children_with_tokens()
                    .find(|n| n.kind() == rnix::SyntaxKind::TOKEN_IDENT) 
                {
                    return ident.to_string();
                }
            }
            _ => {}
        }
    }
    String::new()
}
