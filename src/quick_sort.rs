/**
 * quick sort for arr
 */
#[allow(dead_code)]
fn sort<T: Ord + Copy>(arr: &mut [T]) {
    let l = arr.len();
    if l == 0 {
        return;
    }

    let mut queue = Vec::new();
    queue.push((0, l - 1));
    while let Some((s, e)) = queue.pop() {
        if s >= e {
            continue;
        }
        let i = traverse_swap(arr, s, e);
        if i > 0 {
            queue.push((s, i - 1));
        }
        queue.push((i + 1, e));
    }
}

/**
 * traverse and swap in array, choose i as min value at the begining
 * return split point
 */
fn traverse_swap<T: Ord + Copy>(arr: &mut [T], mut i: usize, mut j: usize) -> usize {
    let pivot_v = arr[i];
    let mut slot = i;
    while i < j {
        // iterate from right
        while arr[j] > pivot_v && i < j {
            j -= 1;
        }
        if i >= j {
            break;
        }
        arr[slot] = arr[j];
        slot = j;
        i += 1;

        // iterate from left
        while arr[i] < pivot_v && i < j {
            i += 1;
        }
        if i >= j {
            break;
        }
        arr[slot] = arr[i];
        slot = i;
        j -= 1;
    }
    arr[slot] = pivot_v;

    i
}

#[test]
fn test_quick_sort() {
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
