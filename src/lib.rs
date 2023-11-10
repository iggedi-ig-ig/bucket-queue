//! # bucket-queue
//! A bucket queue implementation in the rust programming language.
//!
//! # Bucket Queues
//! Bucket queues are a specialized priority queue data structure that work well for monotonous, integer data with some maximum increment.
//! Their most common application are for Dijkstra's algorithm for shortest paths, but they can be applied to any problem where:
//! - keys are integers
//! - keys are monotonous, meaning after the smallest key was extracted, no key that's smaller than the previous minimum can be added
//! - there is a constant (small) range of values that can be added to the queue
//!
//! This is true for for example Dijkstra on Graphs with integer edge weights with a maximum edge weight of C, because:
//! - edge weights are integers
//! - if a value is pushed to the queue, it will have the value of the previous minimum plus some edge weight
//! - that edge weight will be smaller than or equal to C
//! This is also the most common application for bucket queues.
#![deny(missing_docs)]

use std::marker::PhantomData;

/// A monotone priority queue with all common operations being O(C) where C is a constant.
/// This data structure is comonly used to implement Dijkstra's algorithm for shortest paths
/// on graphs with small, non-negatie, integer edge weights with maximum weight C which,
/// using this data strucutre, has run time complexity of O(m + nC)
/// (instead of O(m + n log n) using fibonacci heaps.)
///
/// The idea of this data structure is based on the fact that in dijkstra, when scanning any
/// given node, the maximum value that can be added to the queue is M + C,
/// where M is the distance of the previously scanned node and C is the maximum edge weight,
/// and the next scanned node will have at least a distance of m to the origin.
///
/// Internally, this data structure converts all `T`s to `usize`, so it might act unexpectedly
/// when `T` holds state or something similar.
/// This data structure is meant to only be used with `T` being a primitive.
///
/// # Operations
/// Operation | Complexity
/// ---|---
/// `insert` | O(1)
/// `remove` | O(1)
/// `decrease_key` | O(1)
/// `get_min` | O(C)
/// `pop_min` | O(C)
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BucketQueue<T>
where
    usize: From<T>,
    T: From<usize>,
{
    buckets: Vec<usize>,
    max_increment: usize,
    last_min: usize,
    __phantom: PhantomData<T>,
}

impl<T> BucketQueue<T>
where
    usize: From<T>,
    T: From<usize>,
{
    /// Create a new [`BucketQueue`], given the maximum increment of values.
    pub fn new(max_increment: T) -> Self {
        let max_increment = max_increment.into();
        Self {
            buckets: vec![0; max_increment + 1],
            max_increment,
            last_min: 0,
            __phantom: PhantomData,
        }
    }

    /// Insert a new element to this queue
    ///
    /// # Panic
    /// This method panics if the parameter is not at least `last_min` (the value most recently returned
    /// by `pofp_min`) or not smaller than or equal to `last_min` + `max_increment` (`max_increment` was specified at creation)
    pub fn insert(&mut self, value: T) {
        let value: usize = value.into();
        assert!(
            value < self.last_min + self.max_increment && value >= self.last_min,
            "BucketQueues only work if inserted elements are monotone, and only increase by a maximum of `max_step`"
        );

        let index = value % (self.max_increment + 1);
        self.buckets[index] += 1;
    }

    /// Removes an element if it is present. If it was present, it is returned.
    pub fn remove(&mut self, value: T) -> Option<T> {
        let value: usize = value.into();
        assert!(
            value < self.last_min + self.max_increment && value >= self.last_min,
            "Value has to be between `last_min` and `last_min + max_step`"
        );

        let index = value % (self.max_increment + 1);
        if let Some(new_value) = self.buckets[index].checked_sub(1) {
            self.buckets[index] = new_value;
            Some(value.into())
        } else {
            None
        }
    }

    /// Moves an element from priority `value` to `new_key`. If no element with `value` exists, nothing changes.
    /// Note that `value` and `new_key` both have to be between `last_min` and `last_min + max_increment`,
    /// and `new_key` has to be smaller than or equal to `value`. Otherwise, this method panics.
    pub fn decrease_key(&mut self, value: T, new_key: T) {
        let value: usize = value.into();
        assert!(
            value < self.last_min + self.max_increment && value >= self.last_min,
            "Value has to be between `last_min` and `last_min + max_step`"
        );
        let old_index = value % (self.max_increment + 1);

        let new_key: usize = new_key.into();
        assert!(
            value < self.last_min + self.max_increment && value >= self.last_min,
            "New key has to be between `last_min` and `last_min + max_step`"
        );
        let new_index = new_key % (self.max_increment + 1);

        // technically, the key could just aswell increase as long as it doesn't get larger than last_min + max_increment
        assert!(new_key <= value, "Key has to decrease");

        if let Some(new_count) = self.buckets[old_index].checked_sub(1) {
            self.buckets[old_index] = new_count;
            self.buckets[new_index] += 1;
        }
    }

    /// Returns the smallest element from this queue, without removing it. To return and remove the smallest
    /// element, call `pop_min`.
    ///
    /// # Returns
    /// The value returned by this method will be larger than or equal to `last_min` (the value returned by
    /// the most recent call to `pop_min`, or zero), and smaller than or equal to `last_min` + `max_increment`
    /// (`max_increment` was specified at creation).
    /// This method returns `None` if the queue is empty.
    pub fn get_min(&self) -> Option<T> {
        (self.last_min..=self.last_min + self.max_increment)
            .find(|&index| self.buckets[index % (self.max_increment + 1)] > 0)
            .map(|index| T::from(index))
    }

    /// Returns the smallest element from this queue and removes it. To only read the current minimum, call
    /// `get_min`.
    ///
    /// # Returns
    /// The value returned by this method will be larger than or equal to `last_min` (the value returned by
    /// the most recent call to `pop_min`, or zero), and smaller than or equal to `last_min` + `max_increment`
    /// (`max_increment` was specified at creation).
    /// This method returns `None` if the queue is empty.
    ///
    /// # Side effects
    /// Internaly, this method updates `last_min`, meaning that after this method is called, only
    /// elements larger than or equal to its return value can still be added to the queue. (see `insert`)
    pub fn pop_min(&mut self) -> Option<T> {
        if let Some(min_index) = (self.last_min..=self.last_min + self.max_increment)
            .find(|&index| self.buckets[index % (self.max_increment + 1)] > 0)
        {
            self.last_min = min_index;
            self.buckets[min_index % (self.max_increment + 1)] -= 1;
            Some(min_index.into())
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple() {
        let mut queue = BucketQueue::new(10);

        queue.insert(5);
        queue.insert(4);
        queue.insert(3);
        queue.insert(7);

        assert_eq!(queue.pop_min(), Some(3));
        assert_eq!(queue.pop_min(), Some(4));
        assert_eq!(queue.pop_min(), Some(5));
        assert_eq!(queue.pop_min(), Some(7));
        assert_eq!(queue.pop_min(), None);
    }

    #[test]
    fn remove() {
        let mut queue = BucketQueue::new(10);

        queue.insert(5);
        queue.insert(4);
        queue.insert(6);
        queue.insert(3);

        assert_eq!(queue.remove(3), Some(3));
        assert_eq!(queue.remove(4), Some(4));

        assert_eq!(queue.remove(3), None);

        assert_eq!(queue.pop_min(), Some(5));
    }

    #[test]
    #[should_panic]
    fn overflow() {
        let mut queue = BucketQueue::new(5);
        queue.insert(10);
    }
}
