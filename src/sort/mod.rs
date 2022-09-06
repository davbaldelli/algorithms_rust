pub fn insertion_sort(array: &mut [i32]) {
    for i in 1..array.len() {
        let key = array[i];
        let mut j = i - 1;
        while array[j] > key {
            array.swap(j, j + 1);
            if j == 0 {
                break;
            }
            j -= 1;
        }
    }
}
