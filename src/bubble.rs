/**
 * basic bubble sort implementation
 */
#[allow(dead_code)]
pub fn sort<T>(arr: &mut Box<[T]>)
where
    T: Ord + Copy,
{
    for i in 0..arr.len() - 1 {
        let mut flag = false;
        for j in 0..arr.len() - 1 - i {
            if arr[j] <= arr[j + 1] {
                continue;
            }

            let tmp = arr[j];
            arr[j] = arr[j + 1];
            arr[j + 1] = tmp;
            flag = true;
        }

        if !flag {
            break;
        }
    }
}

// TODO(me): maybe add more test case
#[test]
fn test_bubble() {
    use crate::random::generate_random_data;

    const TEST_N: usize = 1000;

    let mut arr = generate_random_data(TEST_N, 1000);
    sort(&mut arr);

    for i in 0..TEST_N - 1 {
        assert!(
            arr[i] <= arr[i + 1],
            "the order is not correct for position: {} {}",
            i,
            i + 1
        );
    }
}
