pub mod node;
pub mod linked_list;

/// Get data from node
#[doc(hidden)]
#[macro_export(local_inner_macros)]
macro_rules! data {
    ($node:ident) => (
        &(*$node.as_ptr()).data
    );
}

/// Unsafe get data from node
#[doc(hidden)]
#[macro_export(local_inner_macros)]
macro_rules! data_unsafe {
    ($node:ident) => (
        unsafe{ data!($node) }
    );
}

/// Get next node
#[doc(hidden)]
#[macro_export(local_inner_macros)]
macro_rules! next {
    ($node:ident) => (
        (*$node.as_ptr()).next
    );
}

/// Unsafe get next node
#[doc(hidden)]
#[macro_export(local_inner_macros)]
macro_rules! next_unsafe {
    ($node:ident) => (
        unsafe{ next!($node) }
    );
}

/// Get previous node
#[doc(hidden)]
#[macro_export(local_inner_macros)]
macro_rules! previous {
    ($node:ident) => (
        (*$node.as_ptr()).previous
    );
}

/// Unsafe get previous node
#[doc(hidden)]
#[macro_export(local_inner_macros)]
macro_rules! previous_unsafe {
    ($node:ident) => (
        unsafe { previous!($node) }
    );
}

/// Creates a LinkedList containing the arguments.
///
/// Create a LinkedLIst using list syntax:
/// ```
/// # #[macro_use] extern crate doubly_linked_list;
/// let list = linkedlist!("foo", "bar", "baz");
///
/// let mut list_iter = list.iter();
/// assert_eq!(list_iter.next(), Some(&"foo"));
/// assert_eq!(list_iter.next(), Some(&"bar"));
/// assert_eq!(list_iter.next(), Some(&"baz"));
/// assert_eq!(list_iter.next(), None);
///
/// assert_eq!(list.get(0), Some(&"foo"));
/// assert_eq!(list.get(1), Some(&"bar"));
/// assert_eq!(list.get(2), Some(&"baz"));
/// assert_eq!(list.get(3), None);
/// ```
#[macro_export]
macro_rules! linkedlist {
    () => (
        linked_list::new()
    );
    // match val, val, ...
    ($($val:expr$(,)?)+) => (
        {
            let mut list = $crate::linked_list::LinkedList::new();
            $(list.push_back($val);)*

            list
        }
    );
}
