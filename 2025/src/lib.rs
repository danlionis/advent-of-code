pub mod template;

// Use this file to add helper functions and additional modules.

#[derive(Debug, Clone)]
pub struct BoundedGrid<T> {
    pub data: Vec<T>,

    pub width: usize,
    pub height: usize,
}

impl<T> BoundedGrid<T>
where
    T: Default + Clone,
{
    pub fn new(x: usize, y: usize) -> Self {
        let capacity = x * y;

        let data = vec![T::default(); capacity];

        BoundedGrid {
            data,
            width: x,
            height: y,
        }
    }

    pub fn clear(&mut self) {
        self.data.fill_with(T::default);
    }
}

impl<T> BoundedGrid<T> {
    pub fn set(&mut self, x: usize, y: usize, element: T) {
        let index = y * self.width + x;
        // assert!(index < self.data.len());

        self.data[index] = element;
    }

    pub fn get_unchecked(&self, x: usize, y: usize) -> &T {
        let index = y * self.width + x;
        assert!(index < self.data.len());

        unsafe { self.data.get_unchecked(index) }
    }

    pub fn get(&self, x: usize, y: usize) -> Option<&T> {
        if x >= self.width || y >= self.height {
            return None;
        }

        let index = y * self.width + x;

        Some(unsafe { self.data.get_unchecked(index) })
    }

    pub fn iter(&self) -> BoundedGridIter<'_, T> {
        BoundedGridIter {
            inner: self,
            pos: 0,
        }
    }

    pub fn positions(&self) -> Positions<'_, T> {
        Positions {
            inner: self,
            idx: 0,
        }
    }
}

pub struct Positions<'a, T> {
    inner: &'a BoundedGrid<T>,
    idx: usize,
}

impl<'a, T> Iterator for Positions<'a, T> {
    type Item = (usize, usize, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        if self.idx >= self.inner.data.len() {
            return None;
        }

        let x = self.idx % self.inner.width;
        let y = self.idx / self.inner.width;

        self.idx += 1;

        Some((x, y, self.inner.get_unchecked(x, y)))
    }
}

pub struct BoundedGridIter<'a, T> {
    inner: &'a BoundedGrid<T>,
    pos: usize,
}

impl<'a, T> Iterator for BoundedGridIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos >= self.inner.data.len() {
            return None;
        }

        let res = unsafe { self.inner.data.get_unchecked(self.pos) };

        self.pos += 1;

        Some(res)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bounded_grid_new() {
        let grid = BoundedGrid::<u32>::new(5, 10);
        assert_eq!(grid.width, 5);
        assert_eq!(grid.height, 10);
        assert_eq!(grid.data.len(), 5 * 10);
        assert!(grid.data.iter().all(|&x| x == 0));
    }

    #[test]
    fn test_bounded_grid_set_get() {
        let mut grid = BoundedGrid::<char>::new(2, 2);
        grid.set(0, 0, 'a');
        grid.set(1, 0, 'b');
        grid.set(0, 1, 'c');
        grid.set(1, 1, 'd');

        assert_eq!(*grid.get_unchecked(0, 0), 'a');
        assert_eq!(*grid.get_unchecked(1, 0), 'b');
        assert_eq!(*grid.get_unchecked(0, 1), 'c');
        assert_eq!(*grid.get_unchecked(1, 1), 'd');
    }

    #[test]
    #[should_panic]
    fn test_bounded_grid_set_out_of_bounds() {
        let mut grid = BoundedGrid::<u8>::new(1, 1);
        grid.set(1, 0, 5);
    }

    #[test]
    #[should_panic]
    fn test_bounded_grid_get_out_of_bounds() {
        let grid = BoundedGrid::<u8>::new(1, 1);
        grid.get_unchecked(0, 1);
    }

    #[test]
    fn test_bounded_grid_get() {
        let mut grid = BoundedGrid::<char>::new(2, 2);
        grid.set(0, 0, 'a');
        grid.set(1, 0, 'b');
        grid.set(0, 1, 'c');
        grid.set(1, 1, 'd');

        // Test in-bounds access
        assert_eq!(grid.get(0, 0), Some(&'a'));
        assert_eq!(grid.get(1, 0), Some(&'b'));
        assert_eq!(grid.get(0, 1), Some(&'c'));
        assert_eq!(grid.get(1, 1), Some(&'d'));

        // Test out-of-bounds access with too large coordinates
        assert_eq!(grid.get(2, 0), None);
        assert_eq!(grid.get(0, 2), None);
        assert_eq!(grid.get(2, 2), None);
    }

    #[test]
    fn test_bounded_grid_iter() {
        let mut grid = BoundedGrid::<u8>::new(2, 2);
        grid.set(0, 0, 1);
        grid.set(1, 0, 2);
        grid.set(0, 1, 3);
        grid.set(1, 1, 4);

        let mut iter = grid.data.iter();
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&4));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_custom_bounded_grid_iter() {
        let mut grid = BoundedGrid::<u8>::new(2, 2);
        grid.set(0, 0, 10);
        grid.set(1, 0, 20);
        grid.set(0, 1, 30);
        grid.set(1, 1, 40);

        let mut custom_iter = grid.iter();

        assert_eq!(custom_iter.next(), Some(&10));
        assert_eq!(custom_iter.next(), Some(&20));
        assert_eq!(custom_iter.next(), Some(&30));
        assert_eq!(custom_iter.next(), Some(&40));
        assert_eq!(custom_iter.next(), None);
    }

    #[test]
    fn test_positions_iterator() {
        let grid = BoundedGrid::<()>::new(2, 2);
        // Test a 2x2 grid
        let mut positions = grid.positions();
        assert_eq!(positions.next(), Some((0, 0, &())));
        assert_eq!(positions.next(), Some((1, 0, &())));
        assert_eq!(positions.next(), Some((0, 1, &())));
        assert_eq!(positions.next(), Some((1, 1, &())));
        assert_eq!(positions.next(), None);

        // Test a 3x1 grid
        let grid = BoundedGrid::<()>::new(3, 1);
        let mut positions = grid.positions();
        assert_eq!(positions.next(), Some((0, 0, &())));
        assert_eq!(positions.next(), Some((1, 0, &())));
        assert_eq!(positions.next(), Some((2, 0, &())));
        assert_eq!(positions.next(), None);

        // Test a 1x3 grid
        let grid = BoundedGrid::<()>::new(1, 3);
        let mut positions = grid.positions();
        assert_eq!(positions.next(), Some((0, 0, &())));
        assert_eq!(positions.next(), Some((0, 1, &())));
        assert_eq!(positions.next(), Some((0, 2, &())));
        assert_eq!(positions.next(), None);

        // Test an empty grid (0 width)
        let grid = BoundedGrid::<()>::new(0, 3);
        let mut positions = grid.positions();
        assert_eq!(positions.next(), None);

        // Test an empty grid (0 height)
        let grid = BoundedGrid::<()>::new(3, 0);
        let mut positions = grid.positions();
        assert_eq!(positions.next(), None);

        // Test a 0x0 grid
        let grid = BoundedGrid::<()>::new(0, 0);
        let mut positions = grid.positions();
        assert_eq!(positions.next(), None);
    }
}
