use std::ptr::NonNull;
use std::marker::PhantomData;
use std::mem;

use crate::node::{Node, NodeLink, NodeLinkSome};
use crate::{next, next_unsafe, previous, data};


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

        let new_head = Self::new_node_link(data);

        unsafe { next_unsafe!(new_head) = self.head; }

        if self.head.is_none() {
            self.foot = Some(new_head);
        }

        self.head = Some(new_head);

    }

    pub fn push_back(&mut self, data: T) {
        let new_foot = Some(Self::new_node_link(data));

        if let Some(foot) = self.foot
        {
            unsafe{ next_unsafe!(foot) = new_foot; }
        }
        else
        {
            self.head = new_foot;
        }

        self.foot = new_foot;
    }

    pub fn pop_front(&mut self) -> Option<T> {
        self.head.map(|node| unsafe {
            Box::from_raw(node.as_ptr()).data
        })
    }

    pub fn pop_back(&mut self) -> T {
        todo!()
    }

    pub fn iter(&'_ self) -> Iter<'_, T>
    {
        Iter {
            current: self.head,
            _phanton: PhantomData,
        }
    }
}

pub struct Cursor<'a, T>
{
    current: NodeLink<T>,
    list: &'a LinkedList<T>,
}

pub struct Iter<'a, T>
{
    current: NodeLink<T>,
    _phanton: PhantomData<&'a T>,
}

impl <'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.current.map(|node| {
            self.current = next!(node);

            data!(node)
        })
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
