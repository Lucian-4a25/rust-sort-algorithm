/**
 * basic selection sort implementation
 */
#[allow(dead_code)]
pub fn sort<T>(arr: &mut Box<[T]>)
where
    T: Ord + Copy,
{
    if arr.len() < 2 {
        return;
    }

    for i in 0..arr.len() - 1 {
        let mut min_v = arr[i];
        let mut min_idx = i;

        for j in i + 1..arr.len() {
            if arr[j] < min_v {
                min_v = arr[j];
                min_idx = j;
            }
        }

        if min_idx != i {
            let tmp = arr[i];
            arr[i] = min_v;
            arr[min_idx] = tmp;
        }
    }
}

#[test]
fn test_selection_sort() {
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
