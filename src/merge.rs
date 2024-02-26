use std::collections::VecDeque;

/**
 * Basic merge-sort implementation
 */
#[allow(dead_code)]
pub fn sort<T>(arr: &mut Box<[T]>)
where
    T: Copy + Ord + Default,
{
    if arr.len() < 2 {
        return;
    }

    // init sort container
    let mut sort_container = Vec::new();
    sort_container.resize_with(arr.len(), || Default::default());

    // the arr range to merge
    let mut groups = Vec::new();
    let mut temp = VecDeque::new();
    temp.push_back((0usize, arr.len()));
    // get split positions, inclusive from start, exclusive for end
    while let Some((s, e)) = temp.pop_front() {
        // no more one element left, no need to split
        if e - s <= 1 {
            continue;
        }
        let mid = (s + e) / 2;
        groups.push(((s, mid), (mid, e)));
        temp.push_back((s, mid));
        temp.push_back((mid, e));
    }

    // merge group data
    while let Some(((s1, e1), (s2, e2))) = groups.pop() {
        assert_eq!(e1, s2);
        let mut i = s1;
        let mut j = s2;
        let mut s = i;
        while i < e1 && j < e2 {
            if arr[i] < arr[j] {
                sort_container[s] = arr[i];
                i += 1;
                s += 1;
                continue;
            }
            sort_container[s] = arr[j];
            j += 1;
            s += 1;
        }

        if i == e1 {
            sort_container[s..e2].copy_from_slice(&arr[j..e2]);
        }
        if j == e2 {
            sort_container[s..e2].copy_from_slice(&arr[i..e1]);
        }
        // copy value from sort container to arr
        arr[s1..e2].copy_from_slice(&sort_container[s1..e2]);
    }

    // println!("merge sort yeah!");
}

#[test]
fn test_merge_sort() {
    // let arr = [1, 2, 3];
    // println!("value: {:?}", &arr[1..1]);

    use crate::random::generate_random_data;
    const TEST_N: usize = 100;

    let mut arr = generate_random_data(TEST_N, 100);
    sort(&mut arr);

    for i in 0..TEST_N - 1 {
        assert!(
            arr[i] <= arr[i + 1],
            "the order is not correct for position: {} {}",
            i,
            i + 1
        );
    }
    println!("sorted value: {:?}", arr);
}
