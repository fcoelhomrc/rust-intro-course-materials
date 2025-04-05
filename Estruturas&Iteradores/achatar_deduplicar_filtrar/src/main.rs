use std::collections::HashSet;

fn achatar_deduplicar_filtrar(v: Vec<Vec<u32>>) -> Vec<u32> {
    // sem elementos repetidos e com apenas múltiplos de 2 e 3.
    v  // this arg is passed by ownership (see fn signature - no & borrow)
        .into_iter()  // into_iter (consumes)
        .flatten()  // nested collection into a single flat collection
        .collect::<HashSet<u32>>() // remove dupes with a set
        .into_iter()  // into_inter (consumes)
        .filter(|x| x % 2 == 0 || x % 3 == 0)  // only multiples 2,3
        .collect::<Vec<u32>>()  // return desired collection
}

fn main() {}

#[cfg(test)]
mod achatar_deduplicar_filtrar_test {
    use std::collections::HashSet;

    #[test]
    fn test_func() {
        let vec = vec![vec![1, 2, 3], vec![3, 4, 5], vec![5, 6, 7]];

        let result = super::achatar_deduplicar_filtrar(vec);

        assert!(result.iter().all(|x| x % 2 == 0 || x % 3 == 0));

        let mut seen = HashSet::new();

        assert!(result.iter().all(|x| seen.insert(x)));
    }
}
