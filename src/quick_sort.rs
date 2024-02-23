use std::collections::VecDeque;

/**
 * quick sort for arr
 */
fn sort(arr: &mut [i32]) {
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
        queue.push((s, i - 1));
        queue.push((i + 1, e));
    }
}

/**
 * traverse and swap in array, choose i as min value at the begining
 * return split point
 */
fn traverse_swap(arr: &mut [i32], mut i: usize, mut j: usize) -> usize {
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
    let mut arr = [1, 2, 3, 4, 5, 1, 5, 4, 3, 2, 1, 10, 9, 8, 7, 6];
    sort(&mut arr);
    println!("sorted value: {:?}", arr);
}
