use crate::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Matrix<T> {
    elements: Vec<T>,
    size: UVec2,
}

// Inner
impl<T> Matrix<T> {
    #[inline]
    pub(crate) fn inner(&self) -> &[T] {
        &self.elements
    }

    #[inline]
    fn inner_mut(&mut self) -> &mut [T] {
        &mut self.elements
    }
}

impl<T> Matrix<T> {
    pub fn new(size: UVec2) -> Self
    where
        T: Default + Clone,
    {
        Self::splat(size, T::default())
    }

    pub fn splat(size: UVec2, value: T) -> Self
    where
        T: Clone,
    {
        let vec = vec![value; size.element_product() as usize];

        Self {
            elements: vec,
            size,
        }
    }

    pub fn from_elements(elements: Vec<T>, size: UVec2) -> Self {
        debug_assert!(
            elements.len() == size.element_product() as usize,
            "Number of elements does not match size"
        );

        Self { elements, size }
    }

    pub fn from_elements_2d(elements: Vec<Vec<T>>) -> Self {
        let size = UVec2::new(elements.len() as u32, elements[0].len() as u32);
        let elements = elements.into_iter().flatten().collect_vec();

        debug_assert!(
            elements.len() == size.element_product() as usize,
            "Number of elements does not match size"
        );

        Self { elements, size }
    }
}

// Standard functions
impl<T> Matrix<T> {
    #[inline(always)]
    pub fn is_in_bounds(&self, pos: UVec2) -> bool {
        pos.x < self.size.x && pos.y < self.size.y
    }

    /// Uses size as an argument for better compiler optimisations (potentially)
    ///
    /// Use this function when there are many calls with the same matrix size
    #[inline(always)]
    pub fn is_in_bounds_multi(pos: UVec2, size: UVec2) -> bool {
        pos.x < size.x && pos.y < size.y
    }

    #[inline(always)]
    pub fn pos_to_idx(&self, pos: UVec2) -> u32 {
        debug_assert!(
            self.is_in_bounds(pos),
            "Position {} is out of bounds for matrix of size {}",
            pos,
            self.size
        );

        pos.y * self.size.x + pos.x
    }

    #[inline(always)]
    pub fn idx_to_pos(&self, idx: u32) -> UVec2 {
        debug_assert!(
            idx < self.element_count(),
            "Index {} is out of bounds for matrix of size {} ({} elements)",
            idx,
            self.size,
            self.element_count()
        );

        let width = self.size.x;

        UVec2::new(idx % width, idx / width)
    }

    #[inline(always)]
    pub fn get(&self, pos: UVec2) -> &T {
        // Safety: Bounds have already been checked
        unsafe { self.elements.get_unchecked(self.pos_to_idx(pos) as usize) }
    }

    #[inline(always)]
    pub fn get_mut(&mut self, pos: UVec2) -> &mut T {
        let idx = self.pos_to_idx(pos) as usize;

        // Safety: Bounds have already been checked
        unsafe { self.elements.get_unchecked_mut(idx) }
    }

    #[inline(always)]
    pub fn set(&mut self, pos: UVec2, value: T) {
        *self.get_mut(pos) = value;
    }

    #[inline]
    pub fn size(&self) -> UVec2 {
        self.size
    }

    #[inline]
    pub fn flat_vec(&self) -> &Vec<T> {
        &self.elements
    }

    #[inline]
    pub fn into_flat_vec(self) -> Vec<T> {
        self.elements
    }

    #[inline]
    pub fn positions(&self) -> Vec<UVec2> {
        self.size.positions()
    }

    #[inline]
    pub fn element_count(&self) -> u32 {
        self.size.element_product()
    }

    #[inline]
    pub fn iter(&self) -> MatrixIterator<T> {
        MatrixIterator::new(self.inner())
    }

    #[inline]
    pub fn iter_mut(&mut self) -> MatrixMutIterator<T> {
        MatrixMutIterator::new(self.inner_mut())
    }

    #[inline]
    pub fn iter_with_pos(&self) -> MatrixPosIterator<T> {
        MatrixPosIterator::new(self.inner(), self.size.x)
    }

    #[inline]
    pub fn iter_with_pos_mut(&mut self) -> MatrixPosMutIterator<T> {
        let width = self.size.x;

        MatrixPosMutIterator::new(self.inner_mut(), width)
    }
}
