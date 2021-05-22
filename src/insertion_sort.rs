/// Insertion Sort: It sorts the array using __incremental_ approach for more info see CLRS.
/// It has a time complexity of O(N*N) and space complexity of O(1).
/// It is a good algorithm for sorting __small__ number of elements.
/// It is an __in place__ algorithm.
/// Loop Invariant: At each new iteration the first i elements are in sorted order.
pub(crate) fn insertion_sort<T: Ord>(v: &mut [T]) {
    'outer: for i in 1..v.len() {
        for j in (0..i).rev() {
            if v[j] <= v[j + 1] {
                continue 'outer;
            } else {
                v.swap(j, j + 1);
            }
        }
    }
}
