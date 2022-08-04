use std::ptr::NonNull;



pub type NodeLinkSome<T> = NonNull<Node<T>>;
pub type NodeLink<T> = Option<NodeLinkSome<T>>;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_node() {
        let _node = Node::new("hej");
    }
}
