use std::ptr::NonNull;
use std::fmt;

pub type NodeLinkSome<T> = NonNull<Node<T>>;
pub type NodeLink<T> = Option<NodeLinkSome<T>>;

#[derive(Debug)]
pub struct Node<T> {
    pub data: T,
    pub next: NodeLink<T>,
    pub previous: NodeLink<T>,
}

impl<T> Node<T> {
    pub fn new(data: T) -> Self {
        Self { data, next: None, previous: None }
    }
}

impl<T: fmt::Display> fmt::Display for Node<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_node() {
        let _node = Node::new("hej");
    }

    #[test]
    fn test_fmt() {
        let node = Node::new(1337);

        assert_eq!(format!("{node}"), "1337");
    }
}
