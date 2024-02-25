/**
 * Shell sort algorithem implementation (a upgraded version of insertion sort)
 */
pub fn sort<T>(arr: &mut Box<[T]>)
where
    T: Ord + Copy,
{
    if arr.len() < 2 {
        return;
    }

    // define a series step with, based it to sort every col in the array
    // NB: There is better strategy for choosing width, see https://zh.wikipedia.org/wiki/%E5%B8%8C%E5%B0%94%E6%8E%92%E5%BA%8F
    let mut step = arr.len() / 2;
    while step >= 1 {
        // start from step, do insertion sort logic from low index to high index for every cols
        for i in step..arr.len() {
            let pivot = arr[i];
            let mut j: usize = i - step;
            let mut overflow = false;
            // replace position if pivot is the lower value
            while arr[j] > pivot {
                arr[j + step] = arr[j];
                if j > step {
                    j -= step;
                } else {
                    overflow = true;
                    break;
                }
            }

            arr[if overflow { j } else { j + step }] = pivot;
        }

        step = step / 2;
    }
}

#[test]
fn test_shell_sort() {
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
