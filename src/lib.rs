mod insertion_sort;
/// Sorting `enum` basically contains all the sorting algorithms given in CLRS.
/// Implemented in the style of rust
/// Example
/// ```
///     fn main() {
///         let mut v = vec![3, 2, 1, 5, 4];
///         sorting::Sorting::InsertionSort.sort(&mut v);
///         assert_eq!(vec![1, 2, 3, 4, 5], v);
///     }
/// ```
pub enum Sorting {
    InsertionSort,
}

impl Sorting {
    pub fn sort<T: Ord>(&self, v: &mut [T]) {
        //! Call sort when to sort the mutable slice with a reference to the sorting algorithm and
        //! with specific options.
        match self {
            Self::InsertionSort => insertion_sort::insertion_sort(v),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Sorting::*;
    #[test]
    fn insertion_sort_example() {
        let mut v = vec![5, 4, 3, 2, 1];
        InsertionSort.sort(&mut v);
        assert_eq!(vec![1, 2, 3, 4, 5], v);
    }
}
