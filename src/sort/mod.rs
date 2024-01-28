pub fn bubble(arr: &mut Vec<i32>) {
    let mut _swapped = false;
    for i in 0..arr.len() {
        _swapped = false;
        for j in 0..arr.len() - i - 1 {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
                _swapped = true;
            }
        }
        if _swapped == false {
            break;
        }
    }
}

pub fn selection(arr: &mut Vec<i32>) {
    let mut _small = 0;
    for i in 0..arr.len() {
        _small = i;
        for j in i + 1..arr.len() {
            if arr[j] < arr[_small] {
                _small = j;
            }
        }
        if _small != i {
            arr.swap(_small, i);
        }
    }
}

#[test]
fn test() {
    let functions: Vec<fn(&mut Vec<i32>)> = vec![bubble, selection];
    for func in functions {
        let mut unsorted_vector = vec![5, 2, 9, 1, 5, 6];
        let expected_result = vec![1, 2, 5, 5, 6, 9];
        func(&mut unsorted_vector);
        assert_eq!(unsorted_vector, expected_result);
    }
}
