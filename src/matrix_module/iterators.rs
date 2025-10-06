use crate::*;

// TODO - add back Filter iterators for extraction functions (better performance ?)
// TODO - impl Exact size iterators

/// Iterator over references
pub struct MatrixIterator<'a, T> {
    elements: &'a [T],
    current_idx: usize,
}

impl<'a, T> MatrixIterator<'a, T> {
    pub(crate) fn new(elements: &'a [T]) -> Self {
        Self {
            elements,
            current_idx: 0,
        }
    }
}

impl<'a, T> Iterator for MatrixIterator<'a, T> {
    type Item = &'a T;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.current_idx < self.elements.len() {
            // Safety: Bounds have already been checked
            let item = unsafe { self.elements.get_unchecked(self.current_idx) };

            self.current_idx += 1;

            Some(item)
        } else {
            None
        }
    }
}

/// Iterator over mutable references
pub struct MatrixMutIterator<'a, T> {
    elements: &'a mut [T],
    current_idx: usize,
}

impl<'a, T> MatrixMutIterator<'a, T> {
    pub(crate) fn new(elements: &'a mut [T]) -> Self {
        Self {
            elements,
            current_idx: 0,
        }
    }
}

impl<'a, T> Iterator for MatrixMutIterator<'a, T> {
    type Item = &'a mut T;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.current_idx >= self.elements.len() {
            return None;
        }

        // SAFETY: We ensure:
        // 1. Each item is only yielded once
        // 2. No overlapping mutable references
        let item = unsafe {
            let ptr = self.elements.as_mut_ptr();

            &mut *ptr.add(self.current_idx)
        };

        self.current_idx += 1;

        Some(item)
    }
}

pub struct MatrixPosIterator<'a, T> {
    data: &'a [T],
    width: u32,
    current_idx: usize,
}

impl<'a, T> MatrixPosIterator<'a, T> {
    #[inline]
    pub fn new(data: &'a [T], width: u32) -> Self {
        Self {
            data,
            width,
            current_idx: 0,
        }
    }
}

impl<'a, T> Iterator for MatrixPosIterator<'a, T> {
    type Item = (&'a T, UVec2);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.current_idx >= self.data.len() {
            return None;
        }

        // Calculate 2D position from linear index
        let x = self.current_idx as u32 % self.width;
        let y = self.current_idx as u32 / self.width;
        let pos = UVec2::new(x, y);

        // SAFETY: Bounds checked above
        let value = unsafe { self.data.get_unchecked(self.current_idx) };
        self.current_idx += 1;

        Some((value, pos))
    }

    // // Optional: Provide size hint for better iterator fusion
    // #[inline]
    // fn size_hint(&self) -> (usize, Option<usize>) {
    //     let remaining = self.data.len() - self.index;
    //     (remaining, Some(remaining))
    // }
}

pub struct MatrixPosMutIterator<'a, T> {
    elements: &'a mut [T],
    width: u32,
    current_idx: usize,
}

impl<'a, T> MatrixPosMutIterator<'a, T> {
    pub(crate) fn new(elements: &'a mut [T], width: u32) -> Self {
        Self {
            elements,
            width,
            current_idx: 0,
        }
    }
}

impl<'a, T> Iterator for MatrixPosMutIterator<'a, T> {
    type Item = (&'a mut T, UVec2);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.current_idx >= self.elements.len() {
            return None;
        }

        // Calculate 2D position from linear index
        let x = self.current_idx as u32 % self.width;
        let y = self.current_idx as u32 / self.width;
        let pos = UVec2::new(x, y);

        // SAFETY: We ensure:
        // 1. Each item is only yielded once
        // 2. No overlapping mutable references
        let item = unsafe {
            let ptr = self.elements.as_mut_ptr();

            &mut *ptr.add(self.current_idx)
        };

        self.current_idx += 1;

        Some((item, pos))
    }
}
