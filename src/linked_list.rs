use std::ptr::NonNull;
use std::marker::PhantomData;

use crate::node::{Node, NodeLink};


pub struct LinkedList<T> {
    head: NodeLink<T>,
    foot: NodeLink<T>,
}

impl<T> Default for LinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self { head: None, foot: None }
    }

    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    pub fn len(&self) -> usize {
        self.iter().count()
    }

    pub fn push_front(&mut self, data: T) {

        let new_node = Box::new(Node::<T>::new(data));
        let new_head = unsafe { NonNull::new_unchecked(Box::into_raw(new_node)) };

        unsafe {
            (*new_head.as_ptr()).next = self.head;

            self.head = Some(new_head);
        }

    }

    pub fn push_back(&mut self, data: T) {

    }

    pub fn iter<'a>(&'a self) -> Iter<'a, T>
    {
        Iter {
            current: self.head,
            dummy: PhantomData,
        }
    }
}

fn get_node_data<'a, T>(node: &NonNull<Node<T>>) -> &'a T {
    unsafe { &(*node.as_ptr()).data }
}

fn get_next<'a, T>(node: &NonNull<Node<T>>) -> NodeLink<T> {
    unsafe {(*node.as_ptr()).next }
}

pub struct Iter<'a, T>
{
    current: NodeLink<T>,
    dummy: PhantomData<&'a T>,
}

impl <'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {

        if let Some(node) = &self.current
        {
            let data = get_node_data(node);
            self.current = get_next(node);

            return Some(&data)
        }

        None
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty() {
        let linked_list = LinkedList::<u32>::new();
        assert!(linked_list.is_empty());
        assert_eq!(linked_list.len(), 0);
    }

    #[test]
    fn test_push_front() {
        let mut linked_list = LinkedList::<u32>::new();

        linked_list.push_front(1337);
        assert_eq!(linked_list.len(), 1);
        linked_list.push_front(42);
        assert_eq!(linked_list.len(), 2);
    }

    #[test]
    fn test_iter() {
        let mut linked_list = LinkedList::<u32>::new();
        linked_list.push_front(1337);
        linked_list.push_front(42);

        let mut linked_list_iter = linked_list.iter();
        assert_eq!(linked_list_iter.next(), Some(&42));
        assert_eq!(linked_list_iter.next(), Some(&1337));
        assert_eq!(linked_list_iter.next(), None);

    }

}
