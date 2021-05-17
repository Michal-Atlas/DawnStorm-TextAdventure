pub type World = Node;
pub type Action = Option<&'static dyn Fn()>;

#[derive(Clone)]
pub struct Node {
    pub name: &'static str,
    pub aliases: &'static [&'static str],
    pub children: &'static [Node],
    pub r#move: Action,
}
