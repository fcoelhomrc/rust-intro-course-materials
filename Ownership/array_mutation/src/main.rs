

fn array_mut_ownership(array: [u32; 5], operation: char, other_member: u32) -> [u32; 5] {
    // todo!( consume the array and return a new object )

    // This function takes ownership of 'array'

    // 'array' is immutable, but we want to change it
    // so let's shadow 'array' by moving it to a ->mutable<- variable also named 'array'
    let mut array = array;
    for i in 0..5 {
        if operation == '+' {
            array[i] += other_member;
        } else if operation == '-' {
            array[i] -= other_member;
        } else if operation == '*' {
            array[i] *= other_member;
        } else if operation == '/' {
            array[i] /= other_member;
        } else {
            return array;  // invalid operation = ignore (do nothing to array)
        }
    }
    array
}

fn array_mut_mut(array: &mut [u32], operation: char, other_member: u32) {
    // todo!( modify inplace )
    // Question: why do we need to explicitly dereference in this case?
    // e.g. If we are modifying a slice, dereferencing is handled for us
    // However, -> we need to explicitly dereference primitive types <-
    // e.g. In this case, 'member' is a mutable reference to the integer
    // stored in the array collection.

    for member in array {  // using 'for' to iterate over a slice
        if operation == '+' {
            *member += other_member;  // need to dereference before modifying the value
        } else if operation == '-' {
            *member -= other_member;
        } else if operation == '*' {
            *member *= other_member;
        } else if operation == '/' && other_member != 0 {  // avoid division by zero
            *member /= other_member;
        } else {
            break;  // invalid operation = ignore (do nothing to array)
        }
    }
}

#[cfg(test)]
mod array_mutation_test {

    const OWNERSHIP_TEST_ARRAY: [u32; 5] = [1, 2, 3, 4, 5];

    #[test]
    fn test_ownership_mutation() {
        assert_eq!(super::array_mut_ownership(OWNERSHIP_TEST_ARRAY, '+', 1), [2, 3, 4, 5, 6]);
        assert_eq!(super::array_mut_ownership(OWNERSHIP_TEST_ARRAY, '-', 1), [0, 1, 2, 3, 4]);
        assert_eq!(super::array_mut_ownership(OWNERSHIP_TEST_ARRAY, '*', 2), [2, 4, 6, 8, 10]);
        assert_eq!(super::array_mut_ownership(OWNERSHIP_TEST_ARRAY, '/', 2), [0, 1, 1, 2, 2]);
    }

    #[test]
    fn test_mut_ref_mutation() {
        let mut array = OWNERSHIP_TEST_ARRAY.clone();

        super::array_mut_mut(&mut array, '+', 1);

        assert_eq!(array, [2, 3, 4, 5, 6]);

        let mut array = OWNERSHIP_TEST_ARRAY.clone();

        super::array_mut_mut(&mut array, '-', 1);

        assert_eq!(array, [0, 1, 2, 3, 4]);

        let mut array = OWNERSHIP_TEST_ARRAY.clone();

        super::array_mut_mut(&mut array, '*', 2);

        assert_eq!(array, [2, 4, 6, 8, 10]);

        let mut array = OWNERSHIP_TEST_ARRAY.clone();

        super::array_mut_mut(&mut array, '/', 2);

        assert_eq!(array, [0, 1, 1, 2, 2]);
    }

}

fn main() {}