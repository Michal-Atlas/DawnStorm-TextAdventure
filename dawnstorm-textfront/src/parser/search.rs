use dawnstorm_core::world::Node;

/// Searches for the most relevant child of the given node
/// by the query.
/// Checks recursively if the returned result has a relevant child,
/// if yes, returns the child.
pub fn room_search<'a>(item: &'a Node, query: &[String]) -> Option<&'a Node> {
    let mut acc_max = 0;
    let mut ret = None;
    for n in item.children.as_ref()? {
        let mut curr_max = 0;
        for q in query {
            if n.aliases.contains(&q) {
                curr_max += 1;
            }
        }
        if curr_max > acc_max {
            acc_max = curr_max;
            ret = Some(n);
        }
    }
    if let Some(s) = room_search(ret?, query) {
        return Some(s);
    }

    ret
}
