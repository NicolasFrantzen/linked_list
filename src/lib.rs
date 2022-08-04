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
macro_rules! next_unsafe {
    ($node:ident) => (
        (*$node.as_ptr()).next
    );
}

#[macro_export(local_inner_macros)]
macro_rules! next {
    ($node:ident) => (
        unsafe{ next_unsafe!($node) }
    );
}

#[macro_export(local_inner_macros)]
macro_rules! previous_unsafe {
    ($node:ident) => (
        (*$node.as_ptr()).previous
    );
}

#[macro_export(local_inner_macros)]
macro_rules! previous {
    ($node:ident) => (
        unsafe { previous_unsafe!($node) }
    );
}
