use std::mem;

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
    len: usize,
}

impl<T> LinkedList<T>
where
    T: Default,
{
    pub fn new() -> Self {
        Default::default()
    }

    pub fn len(&self) -> usize {
        self.len
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
        self.len += 1;
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
        self.len += 1;
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
        self.data[node_index].prev = Some(prev_index);
        self.data[node_index].next = Some(curr_index);
        self.data[prev_index].next = Some(node_index);
        self.data[curr_index].prev = Some(node_index);
        self.len += 1;
    }

    pub fn pop_front(&mut self) -> Option<T> {
        let head_index = self.head?;
        let next_index = self.data[head_index].next;
        if let Some(next) = next_index {
            self.data[next].prev = None;
            self.head = Some(next);
        }
        else {
            self.head = None;
            self.tail = None;
        }

        self.len -= 1;

        Some(mem::take(&mut self.data[head_index].data))
    }

    pub fn pop_tail(&mut self) -> Option<T> {
        let tail_index = self.tail?;
        let prev_index = self.data[tail_index].prev;
        if let Some(prev_index) = prev_index {
            self.data[prev_index].prev = None;
            self.head = Some(prev_index);
        }
        else {
            self.head = None;
            self.tail = None;
        }

        self.len -= 1;

        Some(mem::take(&mut self.data[tail_index].data))
    }

    pub fn remove(&mut self, index: usize) -> Option<T> {
        if index == 0 {
            return self.pop_front();
        }
        if index >= self.len() {
            return self.pop_tail();
        }

        let mut curr_index = self.head.unwrap();
        (0..index).for_each(|_| curr_index = self.data[curr_index].next.unwrap());
        let prev_index = self.data[curr_index].prev.unwrap();
        let next_index = self.data[curr_index].next.unwrap();
        self.data[prev_index].next = Some(next_index);
        self.data[next_index].prev = Some(prev_index);
        self.len -= 1;

        Some(mem::take(&mut self.data[curr_index].data))
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
        let index = self.data.len();
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
