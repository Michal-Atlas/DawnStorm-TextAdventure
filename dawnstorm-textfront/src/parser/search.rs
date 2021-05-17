use dawnstorm_core::world::Node;

pub fn room_search(item: &Node, query: Vec<String>) -> Option<&Node> {
    for n in &item.children {
        for q in &query {
            if n.aliases.contains(&q) {
                return Some(&n);
            }
        }
    }
    None
}
