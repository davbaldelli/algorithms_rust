use rand::Rng;

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

pub fn quicksort(vec: &mut Vec<i32>, head : usize, tail : usize){
    if head < tail{
        let split = rand_partition(vec, head, tail);
        if split > 0 {
            quicksort(vec, head, split - 1);
        }
        quicksort(vec, split + 1, tail);
    }
}

pub fn partition(vec: &mut Vec<i32>, head : usize, tail : usize ) -> usize {
    let last = vec[tail];
    let mut i: i32 = head as i32 - 1;
    for j in head..tail {
        if vec[j] <= last {
            i += 1;
            vec.swap(i as usize, j)
        }
    }
    vec.swap((i + 1) as usize, tail);
    return (i + 1) as usize;
}

pub fn rand_partition(vec: &mut Vec<i32>, head : usize, tail : usize) -> usize{
    let mut rng = rand::thread_rng();
    let i : usize = rng.gen_range(head..tail+1);
    vec.swap(i, tail);
    partition(vec, head, tail)
}


pub fn rand_select(vec : &mut Vec<i32>, head : usize, tail : usize, i : usize) -> i32{
    if head == tail {
        vec[head]
    } else {
        let split = rand_partition(vec, head, tail);
        let k = split - head + 1;
        if k == i {
            vec[split]
        } else if i < k {
            rand_select(vec, head, split - 1, i)
        } else {
            rand_select(vec, split + 1, tail, i - k)
        }
    }
}