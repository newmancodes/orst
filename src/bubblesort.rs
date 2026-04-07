use super::Sorter;

pub struct Bubblesort;

impl Sorter for Bubblesort {
    fn sort<T>(slice: &mut [T])
    where
        T: Ord,
    {
        let mut swapped = true;
        while swapped {
            swapped = false;
            for i in 1..slice.len() {
                if slice[i - 1] > slice[i] {
                    slice.swap(i - 1, i);
                    swapped = true;
                }
            }
        }
    }
}

#[test]
fn bubblesort_works() {
    let mut nums = vec![4, 2, 7, 1, 9];
    super::sort::<_, Bubblesort>(&mut nums);
    assert_eq!(nums, vec![1, 2, 4, 7, 9]);
}