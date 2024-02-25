/**
 * basic insertion sort implementation
 */
pub fn sort<T>(arr: &mut Box<[T]>)
where
    T: Ord + Copy,
{
    if arr.len() < 2 {
        return;
    }

    for i in 1..arr.len() {
        let v = arr[i];
        let mut j = i - 1;
        let mut overflow = false;
        loop {
            if v >= arr[j] {
                break;
            }

            arr[j + 1] = arr[j];
            if j == 0 {
                overflow = true;
                break;
            }
            j -= 1;
        }
        // Now, j is in its final position
        arr[if overflow { 0 } else { j + 1 }] = v;
    }
}

#[test]
fn test_insertion_sort() {
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
