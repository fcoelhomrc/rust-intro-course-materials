use std::fmt::Debug;

// Sorting Genérico:
// Desenvolver dois algoritmos de sorting (bubble, select)
// que sejam capaz de ordenar qualquer Vec, desde que T: Ord.

// Input: Vec<T> where T: Ord
// Output: sorted Vec

// procedure bubbleSort(A : list of sortable items)
// n := length(A)
// repeat
//     swapped := false
//     for i := 1 to n-1 inclusive do
//         { if this pair is out of order }
//         if A[i-1] > A[i] then
//            { swap them and remember something changed }
//             swap(A[i-1], A[i])
//             swapped := true
//         end if
//     end for
// until not swapped
// end procedure

fn bubble_sort<T>(collection: &mut Vec<T>)
where
    T: Ord,
{
    let size = collection.len();
    loop {
        let mut swapped = false;
        for i in (1..size).rev() {
            if collection[i - 1] > collection[i] {
                collection.swap(i - 1, i);
                swapped = true;
            }
        }
        if !swapped {
            break;
        }
    }
}

// procedure bubbleSort(A : list of sortable items)
//     n := length(A)
//     repeat
//         newn := 0
//         for i := 1 to n - 1 inclusive do
//             if A[i - 1] > A[i] then
//                 swap(A[i - 1], A[i])
//                 newn := i
//             end if
//         end for
//         n := newn
//     until n ≤ 1
// end procedure

fn better_bubble_sort<T>(collection: &mut Vec<T>)
where
    T: Ord,
{
    let mut size = collection.len();
    loop {
        let mut new_size: usize = 0;
        for i in 1..size {
            if collection[i - 1] > collection[i] {
                collection.swap(i - 1, i);
                new_size = i;
            }
        }
        size = new_size;
        if size <= 1 {
            break;
        }
    }
}

/* // Sorts (a portion of) an array, divides it into partitions, then sorts those
algorithm quicksort(A, lo, hi) is
    // Ensure indices are in correct order
    if lo >= hi || lo < 0 then
      return

    // Partition array and get the pivot index
    p := partition(A, lo, hi)

    // Sort the two partitions
    quicksort(A, lo, p - 1) // Left side of pivot
    quicksort(A, p + 1, hi) // Right side of pivot

// Divides array into two partitions
algorithm partition(A, lo, hi) is
    pivot := A[hi] // Choose the last element as the pivot

    // Temporary pivot index
    i := lo

    for j := lo to hi - 1 do
    // If the current element is less than or equal to the pivot
    if A[j] <= pivot then
        // Swap the current element with the element at the temporary pivot index
        swap A[i] with A[j]
        // Move the temporary pivot index forward
        i := i + 1

    // Swap the pivot with the last element
    swap A[i] with A[hi]
    return i // the pivot index */


// NEW: when possible, operate on slices rather than specific collections (e.g. Vec)
//      to make code more flexible
fn quick_sort<T: Ord + Debug>(slice: &mut [T]) {
    if slice.len() <= 1 {
        return;
    }
    let p = partition(slice);
    println!("P[{}] -> {:?}", p, slice);  // just to debug

    let (left, right) = slice.split_at_mut(p); // NEW: &mut [start, end) -> &mut [start, mid) + &mut [mid, end)
    quick_sort(left);
    quick_sort(&mut right[1..]);
}

fn partition<T: Ord>(slice: &mut [T]) -> usize {
    let pivot_index = slice.len() - 1;
    let mut i = 0;

    for j in 0..pivot_index {
        if slice[j] <= slice[pivot_index] {  // All elements smaller than pivot go to the left
            slice.swap(i, j);
            i += 1;
        }
    }

    slice.swap(i, pivot_index);  // Put pivot in correct position
    i // final pivot index
}

fn main() {
    let mut test = vec![10, 7, 3, 8, 1];
    println!("{:#?}", test);
    bubble_sort(&mut test);
    println!("{:#?}", test);
}

mod test {
    use crate::{better_bubble_sort, bubble_sort, quick_sort};

    #[test]
    fn test_bubble_sort() {
        let mut test_integer = vec![10, 7, 3, 8, 1];
        let mut test_str = vec!["orange", "apple", "strawberry", "banana", "coconut"];
        bubble_sort(&mut test_integer);
        assert_eq!(test_integer, vec![1, 3, 7, 8, 10]);
        bubble_sort(&mut test_str);
        assert_eq!(
            test_str,
            vec!["apple", "banana", "coconut", "orange", "strawberry"]
        );
    }

    #[test]
    fn test_better_bubble_sort() {
        let mut test_integer = vec![10, 7, 3, 8, 1];
        let mut test_str = vec!["orange", "apple", "strawberry", "banana", "coconut"];
        better_bubble_sort(&mut test_integer);
        assert_eq!(test_integer, vec![1, 3, 7, 8, 10]);
        better_bubble_sort(&mut test_str);
        assert_eq!(
            test_str,
            vec!["apple", "banana", "coconut", "orange", "strawberry"]
        );
    }

    #[test]
    fn test_quick_sort() {
        let mut test_integer = vec![10, 7, 3, 8, 1];
        let mut test_str = vec!["orange", "apple", "strawberry", "banana", "coconut"];
        quick_sort(&mut test_integer);
        assert_eq!(test_integer, vec![1, 3, 7, 8, 10]);
        quick_sort(&mut test_str);
        assert_eq!(
            test_str,
            vec!["apple", "banana", "coconut", "orange", "strawberry"]
        );
    }
}
