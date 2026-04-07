use super::Sorter;

pub struct InsertionSort;

impl Sorter for InsertionSort {
    fn sort<T>(slice: &mut [T])
    where
        T: Ord,
    {
        for unsorted in 1..slice.len() {
            // slice[unsorted..] is not sorted
            // take slice[unsorted] and place in sorted location in slice[..=unsorted]
            // [ 1 3 4 | 2 ]
            // [ 1 3 4 2 | ]
            // [ 1 3 2 4 | ]
            // [ 1 2 3 4 | ]
            let mut i = unsorted;
            while i > 0 && slice[i - 1] > slice[i] {
                slice.swap(i - 1, i);
                i -= 1;
            }
        }
    }
}

#[test]
fn it_works() {
    let mut nums = vec![4, 2, 7, 1, 9];
    InsertionSort::sort(&mut nums);
    assert_eq!(nums, vec![1, 2, 4, 7, 9]);
}
