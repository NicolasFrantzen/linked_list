use std::ptr::NonNull;

type NodeLinkSome<T> = NonNull<Node<T>>;
pub type NodeLink<T> = Option<NodeLinkSome<T>>;

pub trait NodeAccess<'a, T> {
    fn get_data() -> &'a T;
    fn get_next() -> NodeLink<T>;
    fn get_previous() -> NodeLink<T>;
}

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

impl<'a, T> NodeAccess<'a, T> for NodeLinkSome<T> {
    fn get_data() -> &'a T { todo!() }
    fn get_next() -> NodeLink<T> { todo!() }
    fn get_previous() -> NodeLink<T> { todo!() }

    // pub fn get_node_data<'a, T>(node: &NonNull<Node<T>>) -> &'a T {
    //     unsafe { &(*node.as_ptr()).data }
    // }

    // pub fn get_next<'a, T>(node: &NonNull<Node<T>>) -> NodeLink<T> {
    //     unsafe {(*node.as_ptr()).next }
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_node() {
        let _node = Node::new("hej");
    }
}
