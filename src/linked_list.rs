use std::ptr::NonNull;
use std::marker::PhantomData;

use crate::node::{Node, NodeLink, NodeLinkSome, NodeAccess};


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

    fn new_node_link(data: T) -> NodeLinkSome<T> {
        let new_node = Box::new(Node::<T>::new(data));
        unsafe {
            NonNull::new_unchecked(Box::into_raw(new_node))
        }
    }

    pub fn push_front(&mut self, data: T) {

        let new_head = Some(Self::new_node_link(data));

        unsafe {
            (*new_head.unwrap().as_ptr()).next = self.head;

            if self.head.is_none() {
                self.foot = new_head;
            }

            self.head = new_head;

        }
    }

    pub fn push_back(&mut self, data: T) {
        let new_foot = Some(Self::new_node_link(data));

        unsafe {
            if let Some(foot) = self.foot
            {
                // Maybe implement next(), previos() and data() as macro?
                (*foot.as_ptr()).next = new_foot;
            }
            else
            {
                self.head = new_foot;
            }

            self.foot = new_foot;
        }
    }

    pub fn iter(&'_ self) -> Iter<'_, T>
    {
        Iter {
            current: self.head,
            _phanton: PhantomData,
        }
    }
}


pub struct Iter<'a, T>
{
    current: NodeLink<T>,
    _phanton: PhantomData<&'a T>,
}

impl <'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {

        if let Some(node) = &self.current
        {
            let data = node.data();
            self.current = node.next();

            return Some(data)
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
    fn test_push_back() {
        let mut linked_list = LinkedList::<u32>::new();

        linked_list.push_back(1337);
        assert_eq!(linked_list.len(), 1);
        linked_list.push_back(42);
        assert_eq!(linked_list.len(), 2);
    }

    #[test]
    fn test_iter() {
        let mut linked_list = LinkedList::<u32>::new();
        linked_list.push_front(1337);
        linked_list.push_front(42);
        linked_list.push_back(666);

        let mut linked_list_iter = linked_list.iter();
        assert_eq!(linked_list_iter.next(), Some(&42));
        assert_eq!(linked_list_iter.next(), Some(&1337));
        assert_eq!(linked_list_iter.next(), Some(&666));
        assert_eq!(linked_list_iter.next(), None);
    }

}
