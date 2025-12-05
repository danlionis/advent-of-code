pub mod template;

// Use this file to add helper functions and additional modules.

#[derive(Debug, Clone)]
pub struct BoundedGrid<T> {
    pub data: Box<[Option<T>]>,
    pub active: Vec<usize>,

    pub width: usize,
    pub height: usize,
    pub stale: bool,
}

impl<T> BoundedGrid<T>
where
    T: Clone,
{
    pub fn new(x: usize, y: usize) -> Self {
        let capacity = x * y;

        let data = vec![None; capacity];

        BoundedGrid {
            data: data.into_boxed_slice(),
            active: Vec::with_capacity(capacity),
            width: x,
            height: y,
            stale: false,
        }
    }

    pub fn clear(&mut self) {
        self.data.fill(None);
    }
}

impl<T> BoundedGrid<T> {
    pub fn set(&mut self, x: usize, y: usize, element: T) {
        if let Some(index) = self.index(x, y) {
            // assert!(index < self.data.len());

            if self.stale {
                self.cleanup();
            }

            match unsafe { self.data.get_unchecked_mut(index) } {
                v @ None => {
                    *v = Some(element);
                    self.active.push(index);
                }
                v @ Some(_) => {
                    *v = Some(element);
                }
            }
        }
    }

    pub fn get(&self, x: usize, y: usize) -> Option<&T> {
        let index = self.index(x, y)?;
        // dbg!(index);

        unsafe { self.data.get_unchecked(index).as_ref() }
    }

    pub fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut T> {
        let index = self.index(x, y)?;

        unsafe { self.data.get_unchecked_mut(index).as_mut() }
    }

    pub fn remove(&mut self, x: usize, y: usize) -> Option<T> {
        let index = self.index(x, y)?;

        let active = unsafe { self.data.get_unchecked_mut(index) };

        self.stale = true;
        active.take()
    }

    pub fn iter(&self) -> BoundedGridIter<'_, T> {
        BoundedGridIter {
            inner: self,
            idx: 0,
        }
    }

    pub fn positions(&self) -> Positions<'_, T> {
        Positions {
            inner: self,
            idx: 0,
        }
    }

    pub fn index(&self, x: usize, y: usize) -> Option<usize> {
        if x >= self.width || y >= self.height {
            return None;
        }
        let res = y * self.width + x;
        Some(res)
    }

    pub fn cleanup(&mut self) {
        self.active
            .retain(|&i| unsafe { self.data.get_unchecked(i).is_some() });
        self.stale = false;
    }

    pub fn len(&self) -> usize {
        self.active.len()
    }

    pub fn is_empty(&self) -> bool {
        self.active.is_empty()
    }
}

pub struct Positions<'a, T> {
    inner: &'a BoundedGrid<T>,
    idx: usize,
}

impl<'a, T> Iterator for Positions<'a, T> {
    type Item = (usize, usize, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let idx = self.inner.active.get(self.idx)?;
            self.idx += 1;
            if let Some(active) = unsafe { self.inner.data.get_unchecked(*idx) } {
                let x = idx % self.inner.width;
                let y = idx / self.inner.width;
                return Some((x, y, active));
            }
        }
    }
}

pub struct BoundedGridIter<'a, T> {
    inner: &'a BoundedGrid<T>,
    idx: usize,
}

impl<'a, T> Iterator for BoundedGridIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let idx = self.inner.active.get(self.idx)?;
            self.idx += 1;
            if let Some(active) = unsafe { self.inner.data.get_unchecked(*idx) } {
                return Some(active);
            }
        }
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
        // assert!(grid.data.iter().all(|&x| x == 0));
    }

    #[test]
    fn test_bounded_grid_set_get() {
        let mut grid = BoundedGrid::<char>::new(2, 2);
        grid.set(0, 0, 'a');
        grid.set(1, 0, 'b');
        grid.set(0, 1, 'c');
        grid.set(1, 1, 'd');

        assert_eq!(grid.get(0, 0), Some(&'a'));
        assert_eq!(grid.get(1, 0), Some(&'b'));
        assert_eq!(grid.get(0, 1), Some(&'c'));
        assert_eq!(grid.get(1, 1), Some(&'d'));

        assert_eq!(grid.active.len(), 4);
    }

    #[test]
    fn test_bounded_grid_set_out_of_bounds() {
        let mut grid = BoundedGrid::<u8>::new(1, 1);
        grid.set(1, 0, 5);
    }

    #[test]
    fn test_bounded_grid_get_out_of_bounds() {
        let grid = BoundedGrid::<u8>::new(1, 1);
        assert!(grid.get(0, 1).is_none());
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

        let mut iter = grid.iter();
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

    // #[test]
    // fn test_positions_iterator() {
    //     let grid = BoundedGrid::<()>::new(2, 2);
    //     // Test a 2x2 grid
    //     let mut positions = grid.positions();
    //     assert_eq!(positions.next(), Some((0, 0, &())));
    //     assert_eq!(positions.next(), Some((1, 0, &())));
    //     assert_eq!(positions.next(), Some((0, 1, &())));
    //     assert_eq!(positions.next(), Some((1, 1, &())));
    //     assert_eq!(positions.next(), None);
    //
    //     // Test a 3x1 grid
    //     let grid = BoundedGrid::<()>::new(3, 1);
    //     let mut positions = grid.positions();
    //     assert_eq!(positions.next(), Some((0, 0, &())));
    //     assert_eq!(positions.next(), Some((1, 0, &())));
    //     assert_eq!(positions.next(), Some((2, 0, &())));
    //     assert_eq!(positions.next(), None);
    //
    //     // Test a 1x3 grid
    //     let grid = BoundedGrid::<()>::new(1, 3);
    //     let mut positions = grid.positions();
    //     assert_eq!(positions.next(), Some((0, 0, &())));
    //     assert_eq!(positions.next(), Some((0, 1, &())));
    //     assert_eq!(positions.next(), Some((0, 2, &())));
    //     assert_eq!(positions.next(), None);
    //
    //     // Test an empty grid (0 width)
    //     let grid = BoundedGrid::<()>::new(0, 3);
    //     let mut positions = grid.positions();
    //     assert_eq!(positions.next(), None);
    //
    //     // Test an empty grid (0 height)
    //     let grid = BoundedGrid::<()>::new(3, 0);
    //     let mut positions = grid.positions();
    //     assert_eq!(positions.next(), None);
    //
    //     // Test a 0x0 grid
    //     let grid = BoundedGrid::<()>::new(0, 0);
    //     let mut positions = grid.positions();
    //     assert_eq!(positions.next(), None);
    // }
    //
    // #[test]
    fn test_remove() {
        let mut grid = BoundedGrid::<()>::new(2, 2);

        grid.set(0, 0, ());
        grid.set(0, 1, ());
        grid.set(0, 1, ());

        assert_eq!(grid.active.len(), 2);

        grid.remove(0, 0);

        // stale entry still there
        assert_eq!(grid.active.len(), 2);

        assert_eq!(grid.get(0, 0), None);

        grid.cleanup();

        assert_eq!(grid.active.len(), 1);
    }
}
