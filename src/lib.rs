pub mod node;
pub mod linked_list;

#[macro_export(local_inner_macros)]
macro_rules! data_unsafe {
    ($node:ident) => (
        &(*$node.as_ptr()).data
    );
}

#[macro_export(local_inner_macros)]
macro_rules! data {
    ($node:ident) => (
        unsafe{ data_unsafe!($node) }
    );
}

#[macro_export(local_inner_macros)]
macro_rules! next {
    ($node:ident) => (
        (*$node.as_ptr()).next
    );
}

#[macro_export(local_inner_macros)]
macro_rules! next_unsafe {
    ($node:ident) => (
        unsafe{ next!($node) }
    );
}

#[macro_export(local_inner_macros)]
macro_rules! previous {
    ($node:ident) => (
        (*$node.as_ptr()).previous
    );
}

#[macro_export(local_inner_macros)]
macro_rules! previous_unsafe {
    ($node:ident) => (
        unsafe { previous!($node) }
    );
}

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
