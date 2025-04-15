// Sorting Genérico:
// Desenvolver dois algoritmos de sorting (bubble, select)
// que sejam capaz de ordenar qualquer Vec, desde que T: Ord.

// Input: Vec<T> where T: Ord
// Output: sorted Vec

/*procedure bubbleSort(A : list of sortable items)
n := length(A)
repeat
    swapped := false
    for i := 1 to n-1 inclusive do
        { if this pair is out of order }
        if A[i-1] > A[i] then
           { swap them and remember something changed }
            swap(A[i-1], A[i])
            swapped := true
        end if
    end for
until not swapped
end procedure*/

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

fn select_sort<T>(collection: &mut Vec<T>)
where
    T: Ord,
{
    todo!()
}

fn main() {

    let mut test = vec![10, 7, 3, 8, 1];
    println!("{:#?}", test);
    bubble_sort(&mut test);
    println!("{:#?}", test);
    
}


mod test {
    use crate::bubble_sort;

    #[test]
    fn test_bubble_sort() {
        let mut test_integer = vec![10, 7, 3, 8, 1];
        // let mut test_float = vec![2.3, 1.2, 7.5, 0.3, 5.5];  // FIXME: why floats fail??
        let mut test_str = vec!["orange", "apple", "strawberry", "banana", "coconut"];
        bubble_sort(&mut test_integer);
        assert_eq!(test_integer, vec![1, 3, 7, 8, 10]);
        // bubble_sort(&mut test_float);
        // assert_eq!(test_float, vec![0.3, 1.2, 2.3, 5.5, 7.5]);
        bubble_sort(&mut test_str);
        assert_eq!(test_str, vec!["apple", "banana", "coconut", "orange", "strawberry"]);
    }
}
