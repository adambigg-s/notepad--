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

    pub fn remove_at(&mut self, pos: editor::Cursor) {
        self.line.remove(pos.col);
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

    pub fn remove_at(&mut self, pos: editor::Cursor) {
        self.lines.get_mut(pos.row).remove_at(pos);
    }
}

#[derive(Default)]
struct ListNode<T> {
    next: Option<usize>,
    prev: Option<usize>,
    data: T,
}

impl<T> ListNode<T>
where
    T: Default,
{
    fn new(data: T) -> Self {
        Self { data, ..Default::default() }
    }
}

#[derive(Default)]
pub struct LinkedList<T> {
    head: Option<usize>,
    tail: Option<usize>,
    data: Vec<ListNode<T>>,
}

impl<T> LinkedList<T>
where
    T: Default,
{
    pub fn new() -> Self {
        Default::default()
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn push_front(&mut self, data: T) {
        let node_index = self.add_node(data);
        match self.head {
            | Some(head_index) => {
                self.data[head_index].prev = Some(node_index);
                self.data[node_index].next = Some(head_index);
            }
            | None => {
                self.tail = Some(node_index);
            }
        }
        self.head = Some(node_index);
    }

    pub fn push_back(&mut self, data: T) {
        let node_index = self.add_node(data);
        match self.tail {
            | Some(tail_index) => {
                self.data[tail_index].next = Some(node_index);
                self.data[node_index].prev = Some(tail_index);
            }
            | None => {
                self.head = Some(node_index);
            }
        }
        self.tail = Some(node_index);
    }

    pub fn insert(&mut self, index: usize, data: T) {
        if index == 0 {
            self.push_front(data);
            return;
        }
        else if index >= self.len() {
            self.push_back(data);
            return;
        }

        let node_index = self.add_node(data);
        let mut curr_index = self.head.unwrap();
        (0..index).for_each(|_| curr_index = self.data[curr_index].next.unwrap());

        let prev_index = self.data[curr_index].prev.unwrap();
        let next_index = self.data[curr_index].next.unwrap();
        self.data[node_index].prev = Some(prev_index);
        self.data[node_index].next = Some(next_index);
        self.data[prev_index].next = Some(node_index);
        self.data[next_index].prev = Some(node_index);
    }

    pub fn pop_front(&mut self) -> T {
        todo!()
    }

    pub fn pop_tail(&mut self) -> T {
        todo!()
    }

    pub fn remove(&mut self, index: usize) {
        if index == 0 {
            self.pop_front();
            return;
        }
        if index >= self.len() {
            self.pop_tail();
            return;
        }

        let mut curr_index = self.head.unwrap();
        (0..index).for_each(|_| curr_index = self.data[curr_index].next.unwrap());
        let prev_index = self.data[curr_index].prev.unwrap();
        let next_index = self.data[curr_index].next.unwrap();
        self.data[prev_index].next = Some(next_index);
        self.data[next_index].prev = Some(prev_index);
    }

    pub fn get(&self, index: usize) -> &T {
        if index == 0 {
            return &self.data[self.head.unwrap()].data;
        }
        else if index >= self.len() {
            return &self.data[self.tail.unwrap()].data;
        }

        let mut curr_index = self.head.unwrap();
        (0..index).for_each(|_| curr_index = self.data[curr_index].next.unwrap());
        &self.data[curr_index].data
    }

    pub fn get_mut(&mut self, index: usize) -> &mut T {
        if index == 0 {
            return &mut self.data[self.head.unwrap()].data;
        }
        else if index >= self.len() {
            return &mut self.data[self.tail.unwrap()].data;
        }

        let mut curr_index = self.head.unwrap();
        (0..index).for_each(|_| curr_index = self.data[curr_index].next.unwrap());
        &mut self.data[curr_index].data
    }

    pub fn iter<'d>(&'d self) -> ListIter<'d, T> {
        ListIter { list: self, curr: self.head }
    }

    fn add_node(&mut self, data: T) -> usize {
        let index = self.len();
        self.data.push(ListNode::new(data));
        index
    }
}

pub struct ListIter<'d, T> {
    list: &'d LinkedList<T>,
    curr: Option<usize>,
}

impl<'d, T> Iterator for ListIter<'d, T> {
    type Item = &'d T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.curr {
            | Some(curr) => {
                let item = &self.list.data[curr].data;
                self.curr = self.list.data[curr].next;
                Some(item)
            }
            | None => None,
        }
    }
}

#[allow(dead_code)]
#[derive(Default)]
struct RawListNode<T> {
    data: T,
    next: *mut RawListNode<T>,
    prev: *mut RawListNode<T>,
}

#[allow(dead_code)]
impl<T> RawListNode<T> {
    fn new(value: T) -> *mut RawListNode<T> {
        Box::into_raw(Box::new(RawListNode {
            data: value,
            next: ptr::null_mut(),
            prev: ptr::null_mut(),
        }))
    }
}

#[allow(dead_code)]
#[derive(Default)]
pub struct RawLinkedList<T> {
    head: *mut RawListNode<T>,
    tail: *mut RawListNode<T>,
    len: usize,
}

#[allow(dead_code)]
impl<T> RawLinkedList<T>
where
    T: Default,
{
    pub fn new() -> Self {
        Default::default()
    }

    pub fn push_back(&mut self, value: T) {
        let node = RawListNode::new(value);
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
        let node = RawListNode::new(value);
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

    pub fn insert(&mut self, index: usize, value: T) {
        if index == 0 {
            self.push_front(value);
            return;
        }
        if index >= self.len {
            self.push_back(value);
            return;
        }
        let node = RawListNode::new(value);
        unsafe {
            let mut curr = self.head;
            for _ in 0..index {
                curr = (*curr).next;
            }

            let before = (*curr).prev;
            let after = (*curr).next;
            (*node).next = after;
            (*node).prev = before;
            (*before).next = node;
            (*after).prev = node;
        }
        self.len += 1;
    }

    pub fn remove(&mut self, index: usize) {
        unsafe {
            let mut curr = self.head;
            for _ in 0..index {
                curr = (*curr).next
            }

            let before = (*curr).prev;
            let after = (*curr).next;
            (*before).next = after;
            (*after).prev = before;

            _ = Box::from_raw(curr);
        }
        self.len -= 1;
    }

    pub fn get(&self, mut index: usize) -> &T {
        unsafe {
            if index == 0 {
                return &(*self.head).data;
            }
            if index >= self.len {
                return &(*self.tail).data;
            }
        }
        let mut node = self.head;
        unsafe {
            while index > 0 {
                node = (*node).next;
                index -= 1;
            }
            &(*node).data
        }
    }

    pub fn get_mut(&mut self, mut index: usize) -> &mut T {
        unsafe {
            if index == 0 {
                return &mut (*self.head).data;
            }
            if index >= self.len {
                return &mut (*self.tail).data;
            }
        }
        let mut node = self.head;
        unsafe {
            while index > 0 {
                node = (*node).next;
                index -= 1;
            }
            &mut (*node).data
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn iter(&'_ self) -> RawListIterator<'_, T> {
        RawListIterator { curr: self.head, lifetime: Default::default() }
    }
}

pub struct RawListIterator<'d, T> {
    curr: *mut RawListNode<T>,
    lifetime: PhantomData<&'d T>,
}

impl<'d, T> Iterator for RawListIterator<'d, T> {
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

impl<T> Drop for RawLinkedList<T> {
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
