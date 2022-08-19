use std::ptr::NonNull;
use std::marker::PhantomData;
use std::iter::zip;

use crate::node::{Node, NodeLink, NodeLinkSome};
use crate::{next_unsafe, next, previous, data, previous_unsafe};

#[derive(Debug)]
pub struct LinkedList<T> {
    head: NodeLink<T>,
    foot: NodeLink<T>,
    length: usize,
}

impl<T> Default for LinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: PartialEq> PartialEq for LinkedList<T> {
    fn eq(&self, other: &Self) -> bool {
        zip(self.iter(), other.iter()).all(|(x,y)| {
            x == y
        })
    }
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self { head: None, foot: None, length: 0 }
    }

    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    pub fn len(&self) -> usize {
        self.length
    }

    fn new_node_link(data: T) -> NodeLinkSome<T> {
        let new_node = Box::new(Node::<T>::new(data));
        unsafe {
            NonNull::new_unchecked(Box::into_raw(new_node))
        }
    }

    pub fn push_front(&mut self, data: T) {
        Cursor {next: self.head, previous: None, list: self}
            .push(data);
    }

    pub fn push_back(&mut self, data: T) {
        Cursor {next: None, previous: self.foot, list: self}
            .push(data);
    }

    pub fn insert(&mut self, index: usize, data: T) {
        // Create cursor at front of the list
        let mut cursor = Cursor {
            next: self.head,
            previous: None,
            list: self
        };

        // Move the cursor next `index + 1` times
        (0 .. index + 1).for_each(|_| cursor.move_next());
        cursor.push(data);
    }

    pub fn pop_front(&mut self) -> Option<T> {
        Cursor {next: self.head, previous: None, list: self}
            .pop_front()
    }

    pub fn pop_back(&mut self) -> Option<T> {
        Cursor {next: None, previous: self.foot, list: self}
            .pop_back()
    }

    pub fn iter(&'_ self) -> Iter<'_, T> {
        Iter {
            current: self.head,
            length: self.length,
            _phantom: PhantomData,
        }
    }

    pub fn cursor_mut(&mut self) -> Cursor<'_, T> {
        Cursor {
            next: self.head,
            previous: None,
            list: self
        }
    }
}

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        while self.pop_back().is_some() {}
    }
}

pub struct Cursor<'a, T> {
    next: NodeLink<T>,
    previous: NodeLink<T>,
    list: &'a mut LinkedList<T>,
}

impl<'a, T> Cursor<'a, T> {
    pub fn next_data(&self) -> Option<&'a T> {
        self.next.map(|node| data!(node))
    }

    pub fn previous_data(&self) -> Option<&'a T> {
        self.previous.map(|node| data!(node))
    }

    pub fn move_next(&mut self) {
        if let Some(next) = self.next {
            // Set next to nexts next
            self.next = next_unsafe!(next);

            // Set previous to next
            self.previous = Some(next);
        }
    }

    pub fn move_previous(&mut self) {
        if let Some(previous) = self.previous {
            // Set previous to previous' previous
            self.previous = previous_unsafe!(previous);

            // Set next to previous
            self.next = Some(previous);
        }
    }

    pub fn push(&mut self, data: T) {
        let new_head = LinkedList::new_node_link(data);

        if let Some(head) = self.next {
            unsafe {
                // Previous of self.head should be set to new_head
                previous!(head) = Some(new_head);

                // Update new head node next to point at old head
                next!(new_head) = Some(head);
            }

            // Are we at the start of the list?
            if self.next == self.list.head {
                self.list.head = Some(new_head);
            }
        }
        else if let Some(foot) = self.previous {
            unsafe {
                // Previous of new_foot should be set to foot
                previous!(new_head) = Some(foot);

                // Update foot next to new_foot
                next!(foot) = Some(new_head);
            }

            // Are we at the end of the list?
            if self.previous == self.list.foot {
                self.list.foot = Some(new_head);
            }
        }

        if self.list.head.is_none() && self.list.foot.is_none() {
            // If this is the first element, just put it on the list
            self.list.head = Some(new_head);
            self.list.foot = Some(new_head);
        }

        // Update head and length
        self.list.length += 1;
    }

    pub fn pop_front(&mut self) -> Option<T> {
        self.list.head.map(|node| unsafe {
            // Set head to the current heads next node
            self.list.head = next!(node);

            // Set the new head previous to the old heads previous
            // I.e. at end of the list its None
            if let Some(head) = self.list.head {
                previous!(head) = previous!(node);
            }

            // Restore the node as a box and move its data
            let boxed_data = Box::from_raw(node.as_ptr()).data;

            self.list.length -= 1;

            // If the length is now 1, we need head to be equal foot
            // If the length is now 0, we need head and foot to be None
            if self.list.length <= 1
            {
                self.list.foot = self.list.head;
            }

            boxed_data
        })
    }

    pub fn pop_back(&mut self) -> Option<T> {
        self.list.foot.map(|node| unsafe {
            // Set foot to the current foods previous node
            self.list.foot = previous!(node);

            // Set the new foot next to the old heads next
            // I.e. at end of the list its None
            if let Some(foot) = self.list.foot {
                next!(foot) = next!(node)
            }

            // Restore the node as a box and move its data
            let boxed_data = Box::from_raw(node.as_ptr()).data;

            self.list.length -= 1;

            // If the length is now 1, we need head to be equal foot
            // If the length is now 0, we need head and foot to be None
            if self.list.length <= 1
            {
                self.list.head = self.list.foot;
            }

            boxed_data
        })
    }
}

pub struct Iter<'a, T> {
    current: NodeLink<T>,
    length: usize,
    _phantom: PhantomData<&'a T>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.current.map(|node| {
            self.current = next_unsafe!(node);

            data!(node)
        })
    }
}

impl<'a, T> ExactSizeIterator for Iter<'a, T> {
    fn len(&self) -> usize {
        self.length
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
    fn test_partial_eq() {
        let mut linked_list = LinkedList::<u32>::new();
        linked_list.push_front(1337);
        linked_list.push_front(42);

        let mut other_linked_list = LinkedList::<u32>::new();
        other_linked_list.push_front(1337);
        other_linked_list.push_front(42);

        assert_eq!(linked_list, other_linked_list);
        other_linked_list.push_front(666);

        assert_ne!(linked_list, other_linked_list);
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

    #[test]
    fn test_drop() {
        let mut linked_list = LinkedList::<u32>::new();
        linked_list.push_front(1337);
        linked_list.push_front(42);
        linked_list.push_back(666);

        drop(linked_list);
    }

    #[test]
    fn test_push_front_pop_front() {
        let mut linked_list = LinkedList::<String>::new();
        linked_list.push_front(String::from("foo"));
        linked_list.push_front(String::from("bar"));
        linked_list.push_front(String::from("baz"));

        assert_eq!(linked_list.len(), 3);
        assert_eq!(linked_list.pop_front().unwrap().as_str(), "baz");
        assert_eq!(linked_list.len(), 2);
        assert_eq!(linked_list.pop_front().unwrap().as_str(), "bar");
        assert_eq!(linked_list.len(), 1);
        assert_eq!(linked_list.pop_front().unwrap().as_str(), "foo");
        assert_eq!(linked_list.len(), 0);
        assert_eq!(linked_list.pop_front(), None);

        assert_eq!(linked_list.iter().count(), 0);
    }

    #[test]
    fn test_push_back_pop_back() {
        let mut linked_list = LinkedList::<String>::new();
        linked_list.push_back(String::from("foo"));
        linked_list.push_back(String::from("bar"));
        linked_list.push_back(String::from("baz"));

        assert_eq!(linked_list.pop_back().unwrap().as_str(), "baz");
        assert_eq!(linked_list.pop_back().unwrap().as_str(), "bar");
        assert_eq!(linked_list.pop_back().unwrap().as_str(), "foo");
        assert_eq!(linked_list.pop_back(), None);

        assert_eq!(linked_list.iter().count(), 0);
    }

    #[test]
    fn test_push_front_pop_back() {
        let mut linked_list = LinkedList::<u32>::new();

        linked_list.push_front(42);
        linked_list.push_front(1337);

        assert_eq!(linked_list.pop_back(), Some(42));
        assert!(linked_list.head.is_some());
        assert!(linked_list.foot.is_some());
        assert_eq!(linked_list.head, linked_list.foot);

        assert_eq!(linked_list.pop_back(), Some(1337));
        assert!(linked_list.head.is_none());
        assert!(linked_list.foot.is_none());
    }

    #[test]
    fn test_push_back_pop_front() {
        let mut linked_list = LinkedList::<String>::new();
        linked_list.push_back(String::from("foo"));
        linked_list.push_back(String::from("bar"));
        linked_list.push_back(String::from("baz"));

        assert_eq!(linked_list.pop_front().unwrap().as_str(), "foo");
        assert_eq!(linked_list.pop_front().unwrap().as_str(), "bar");
        assert_eq!(linked_list.pop_front().unwrap().as_str(), "baz");
        assert_eq!(linked_list.pop_front(), None);

        assert_eq!(linked_list.iter().count(), 0);
    }

    #[test]
    fn test_pop_none() {
        let mut linked_list = LinkedList::<String>::new();

        assert_eq!(linked_list.pop_front(), None);
        assert_eq!(linked_list.pop_back(), None);

        assert_eq!(linked_list.iter().count(), 0);
    }

    #[test]
    fn test_push_pop_alternate() {
        let mut linked_list = LinkedList::<String>::new();
        linked_list.push_back(String::from("foo"));
        linked_list.push_front(String::from("bar"));

        assert_eq!(linked_list.pop_back().unwrap().as_str(), "foo");
        assert_eq!(linked_list.pop_front().unwrap().as_str(), "bar");
        assert_eq!(linked_list.pop_front(), None);

        assert_eq!(linked_list.iter().count(), 0);
    }

    #[test]
    fn test_push_pop_alternate_2() {
        let mut linked_list = LinkedList::<String>::new();
        linked_list.push_front(String::from("foo"));
        linked_list.push_back(String::from("bar"));

        assert_eq!(linked_list.pop_front().unwrap().as_str(), "foo");
        assert_eq!(linked_list.pop_back().unwrap().as_str(), "bar");
        assert_eq!(linked_list.pop_back(), None);

        assert_eq!(linked_list.iter().count(), 0);
    }

    #[test]
    fn test_push_insert() {
        let mut linked_list = LinkedList::<String>::new();
        linked_list.push_back(String::from("foo"));
        linked_list.push_back(String::from("bar"));
        linked_list.insert(1, String::from("baz"));


        let mut linked_list_iter = linked_list.iter();
        assert_eq!(linked_list_iter.next().unwrap().as_str(), "foo");
        assert_eq!(linked_list_iter.next().unwrap().as_str(), "bar");
        assert_eq!(linked_list_iter.next().unwrap().as_str(), "baz");
        assert_eq!(linked_list_iter.next(), None);
    }

    #[test]
    fn test_cursor_move() {
        let mut linked_list = LinkedList::<u32>::new();

        linked_list.push_back(1337);
        linked_list.push_back(42);
        linked_list.push_back(666);

        let mut cursor = linked_list.cursor_mut();
        assert_eq!(cursor.next_data(), Some(&1337));
        cursor.move_next();
        assert_eq!(cursor.next_data(), Some(&42));
        cursor.move_next();
        assert_eq!(cursor.next_data(), Some(&666));
        cursor.move_next();
        assert_eq!(cursor.next_data(), None);

        // Aaaaand back again
        assert_eq!(cursor.previous_data(), Some(&666));
        cursor.move_previous();
        assert_eq!(cursor.previous_data(), Some(&42));
        cursor.move_previous();
        assert_eq!(cursor.previous_data(), Some(&1337));
        cursor.move_previous();
        assert_eq!(cursor.previous_data(), None);
    }
}
