use std::ptr::NonNull;

pub type NodeLinkSome<T> = NonNull<Node<T>>;
pub type NodeLink<T> = Option<NodeLinkSome<T>>;

pub trait NodeAccess<'a, T> {
    fn data(&self) -> &'a T;
    fn next(&self) -> NodeLink<T>;
    fn previous(&self) -> NodeLink<T>;
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
    fn data(&self) -> &'a T {
        unsafe { &(*self.as_ptr()).data }
    }
    fn next(&self) -> NodeLink<T> {
        unsafe {(*self.as_ptr()).next }
    }
    fn previous(&self) -> NodeLink<T> {
        unsafe {(*self.as_ptr()).previous }
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
