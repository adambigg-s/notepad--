use std::{
    fmt::{self, Display},
    io::{self, Write},
    ops::{Index, IndexMut, RangeBounds},
};

use crate::editor::Editor;

impl Display for Editor {
    fn fmt(&self, frm: &mut fmt::Formatter) -> fmt::Result {
        let mut output = String::from("\x1b[0H\x1b[2J");
        self.text.lines.iter().for_each(|line| {
            line.line.iter().for_each(|char| {
                output.push(*char);
            });
            output.push('\n');
        });
        write!(frm, "{}", output)?;
        write!(frm, "\x1b[{};{}H#", self.cursor.row + 1, self.cursor.col + 1)?;
        io::stdout().flush().unwrap();
        Ok(())
    }
}

#[derive(Default)]
pub struct ScreenWindow {
    pub bounds: BoundingBox<2, usize>,
    pub write_buffer: Buffer<2, char>,
}

impl Display for ScreenWindow {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

pub struct BoundingBox<const N: usize, T> {
    low: [T; N],
    high: [T; N],
}

impl<const N: usize, T> BoundingBox<N, T> {
    pub fn new(low: [T; N], high: [T; N]) -> Self {
        Self { low, high }
    }

    pub fn surrounds(&self, point: [T; N]) -> bool
    where
        T: Clone + Copy + RangeBounds<T> + PartialOrd,
    {
        for i in 0..N {
            if !(self.low[i]..self.high[i]).contains(&point[i]) {
                return false;
            }
        }
        true
    }
}

impl<const N: usize, T> Default for BoundingBox<N, T>
where
    T: Default + Clone + Copy,
{
    fn default() -> Self {
        Self { low: [T::default(); N], high: [T::default(); N] }
    }
}

pub struct Buffer<const N: usize, T> {
    data: Box<[T]>,
    size: [usize; N],
}

impl<const N: usize, T> Buffer<N, T> {
    pub fn new(size: [usize; N]) -> Self
    where
        T: Default + Clone + Copy,
    {
        Self {
            data: vec![T::default(); size.iter().product()].into_boxed_slice(),
            size,
        }
    }

    pub fn linearize(&self, indices: [usize; N]) -> usize {
        let mut index = 0;
        let mut stride = 1;
        (0..N).for_each(|idx| {
            index += indices[idx] * stride;
            stride *= self.size[idx];
        });
        index
    }

    pub fn surrounds(&self, indices: [usize; N]) -> bool {
        for i in 0..N {
            if indices[1] >= self.size[i] {
                return false;
            }
        }
        true
    }
}

impl<const N: usize, T> Index<usize> for Buffer<N, T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl<const N: usize, T> IndexMut<usize> for Buffer<N, T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

impl<const N: usize, T> Index<[usize; N]> for Buffer<N, T> {
    type Output = T;

    fn index(&self, index: [usize; N]) -> &Self::Output {
        &self.data[self.linearize(index)]
    }
}

impl<const N: usize, T> IndexMut<[usize; N]> for Buffer<N, T> {
    fn index_mut(&mut self, index: [usize; N]) -> &mut Self::Output {
        &mut self.data[self.linearize(index)]
    }
}

impl<const N: usize, T> Default for Buffer<N, T>
where
    T: Default + Clone + Copy,
{
    fn default() -> Self {
        Self { data: Default::default(), size: [Default::default(); N] }
    }
}
