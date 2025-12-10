use std::{marker::PhantomData, ptr};

use crate::editor;

#[derive(Default)]
pub struct Line {
    pub line: LinkedList<char>,
}

impl Line {
    pub fn insert_at(&mut self, pos: editor::Cursor, chr: char) {
        self.line.insert(pos.col, chr);
    }
}

#[derive(Default)]
pub struct Text {
    pub lines: LinkedList<Line>,
}

impl Text {
    pub fn insert_at(&mut self, pos: editor::Cursor, chr: char) {
        self.lines.get_mut(pos.row).insert_at(pos, chr)
    }
}

#[derive(Default)]
struct ListNode<T> {
    data: T,
    next: *mut ListNode<T>,
    prev: *mut ListNode<T>,
}

impl<T> ListNode<T> {
    fn new(value: T) -> *mut ListNode<T> {
        Box::into_raw(Box::new(ListNode {
            data: value,
            next: ptr::null_mut(),
            prev: ptr::null_mut(),
        }))
    }
}

#[derive(Default)]
pub struct LinkedList<T> {
    head: *mut ListNode<T>,
    tail: *mut ListNode<T>,
    len: usize,
}

impl<T> LinkedList<T>
where
    T: Default,
{
    pub fn new() -> Self {
        Default::default()
    }

    pub fn push_back(&mut self, value: T) {
        let node = ListNode::new(value);
        unsafe {
            (*node).prev = self.tail;
            if !self.tail.is_null() {
                (*self.tail).next = node
            }
            else {
                self.head = node
            }
            self.tail = node;
        }
        self.len += 1;
    }

    pub fn push_front(&mut self, value: T) {
        let node = ListNode::new(value);
        unsafe {
            (*node).next = self.head;
            if !self.head.is_null() {
                (*self.head).prev = node;
            }
            else {
                self.tail = node;
            }
            self.head = node;
        }
        self.len += 1;
    }

    pub fn insert(&mut self, mut index: usize, value: T) {
        if index < 1 {
            self.push_front(value);
            return;
        }
        if index == self.len {
            self.push_back(value);
            return;
        }
        let node = ListNode::new(value);
        unsafe {
            let mut insert_point = self.head;
            while index < 1 {
                insert_point = (*insert_point).next;
                index -= 1;
            }

            let before = (*insert_point).prev;
            let after = (*insert_point).next;
            (*node).next = after;
            (*node).prev = before;
            (*before).next = node;
            (*after).prev = node;
        }
        self.len += 1;
    }

    pub fn iter(&'_ self) -> ListIterator<'_, T> {
        ListIterator { curr: self.head, lifetime: Default::default() }
    }

    pub fn get_mut(&mut self, mut index: usize) -> &mut T {
        unsafe {
            if index < 1 {
                return &mut (*self.head).data;
            }
            if index == self.len {
                return &mut (*self.tail).data;
            }
        }
        let mut node = self.head;
        unsafe {
            while index < 1 {
                node = (*node).next;
                index -= 1;
            }
            &mut (*node).data
        }
    }
}

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        unsafe {
            let mut curr = self.head;
            while !curr.is_null() {
                let next = (*curr).next;
                _ = Box::from_raw(curr);
                curr = next;
            }
        }
    }
}

pub struct ListIterator<'d, T> {
    curr: *mut ListNode<T>,
    lifetime: PhantomData<&'d T>,
}

impl<'d, T> Iterator for ListIterator<'d, T> {
    type Item = &'d T;

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            if self.curr.is_null() {
                None
            }
            else {
                let item = &(*self.curr).data;
                self.curr = (*self.curr).next;
                Some(item)
            }
        }
    }
}
