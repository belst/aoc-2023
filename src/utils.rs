use std::ops::{Index, IndexMut};

#[derive(Debug, Clone)]
pub struct Grid<T> {
    width: usize,
    height: usize,
    data: Vec<T>,
}

impl<T> Grid<T> {
    pub fn new(data: Vec<T>, width: usize) -> Self {
        let height = data.len() / width;
        assert_eq!(data.len(), width * height);
        Self {
            width,
            height,
            data,
        }
    }
    pub fn width(&self) -> usize {
        self.width
    }
    pub fn height(&self) -> usize {
        self.height
    }
}

impl<T> Index<(usize, usize)> for Grid<T> {
    type Output = T;

    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        assert!(x < self.width && y < self.height);
        &self.data[x + y * self.width]
    }
}

impl<T> IndexMut<(usize, usize)> for Grid<T> {
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut Self::Output {
        assert!(x < self.width && y < self.height);
        &mut self.data[x + y * self.width]
    }
}
