use num_traits::{Num, NumCast};

/// A monotone priority queue with all common operations being O(C) where C is a constant.
/// This data structure is comonly used to implement Dijkstra's algorithm for shortest paths
/// on graphs with small, non-negatie, integer edge weights with maximum weight C which,
/// using this data strucutre, has run time complexity of O(n + mC)
/// (instead of O(n + m log n) using fibonacci heaps.)
///
/// The idea of this data structure is based on the fact that in dijkstra, when scanning any
/// given node, the maximum value that can be added to the queue is m + C,
/// where m is the distance of the previously scanned node and C is the maximum edge weight,
/// and the next scanned node will have at least a distance of m to the origin.
///
/// # Operations
/// ```text
/// + ------------ + ----- +
/// | insert       |  O(1  |
/// + ------------ + ----- +
/// | remove       |  O(1) |
/// + ------------ + ----- +
/// | pop_min      |  O(C) |
/// + ------------ + ----- +
/// | get_min      |  O(C) |
/// + ------------ + ----- +
/// | decrease_key |  O(1) |
/// + ------------ + ----- +
/// ```
pub struct BucketQueue<T>
where
    T: Copy + Num + NumCast + PartialOrd,
{
    buckets: Vec<usize>,
    max_increment: T,
    last_min: T,
}

impl<T> BucketQueue<T>
where
    T: Copy + Num + NumCast + PartialOrd,
{
    /// Create a new [`BucketQueue`], given the maximum increment of values.
    pub fn new(max_increment: T) -> Self {
        Self {
            buckets: vec![0; (max_increment + T::one()).to_usize().unwrap()],
            max_increment,
            last_min: T::zero(),
        }
    }

    /// Insert a new element to this queue
    ///
    /// # Panic
    /// This method panics if the parameter is not at least `last_min` (the value most recently returned
    /// by `pofp_min`) or not smaller than or equal to `last_min` + `max_increment` (`max_increment` was specified at creation)
    pub fn insert(&mut self, value: T) {
        assert!(
            value < self.last_min + self.max_increment && value >= self.last_min,
            "BucketQueues only work if inserted elements are monotone, and only increase by a maximum of `max_step`"
        );

        self.buckets[(value % (self.max_increment + T::one()))
            .to_usize()
            .unwrap()] += 1;
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
        (0..=self.max_increment.to_usize().unwrap())
            .map(|offset| self.last_min.to_usize().unwrap() + offset)
            .find(|&index| {
                self.buckets[index % (self.max_increment + T::one()).to_usize().unwrap()] > 0
            })
            .map(|index| T::from(index).unwrap())
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
    /// Internaly, this method updates the `last_min` field, meaning that after this method is called, only
    /// elements larger than or equal to its return value can be added to the queue. (see `insert`)
    pub fn pop_min(&mut self) -> Option<T> {
        if let Some(min_index) = (0..=self.max_increment.to_usize().unwrap())
            .map(|offset| self.last_min.to_usize().unwrap() + offset)
            .find(|&index| {
                self.buckets[index % (self.max_increment + T::one()).to_usize().unwrap()] > 0
            })
        {
            let min = T::from(min_index).unwrap();
            self.last_min = min;
            self.buckets[min_index % (self.max_increment + T::one()).to_usize().unwrap()] -= 1;
            Some(min)
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
    #[should_panic]
    fn overflow() {
        let mut queue = BucketQueue::new(5);
        queue.insert(10);
    }
}
