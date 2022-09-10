pub struct HeapElem {
    key: usize,
    pub prio: f32,
}

pub struct MinHeap {
    pub heap: Vec<HeapElem>,
    positions: Vec<i32>,
}

fn parent(i: i32) -> i32 {
    ((i + 1) / 2) - 1
}

fn l_child(i: usize) -> usize {
    2 * i + 1
}

fn r_child(i: usize) -> usize {
    l_child(i) + 1
}

#[allow(dead_code)]
impl MinHeap {
    pub fn new() -> MinHeap {
        return MinHeap {
            heap: Vec::new(),
            positions: Vec::new(),
        };
    }

    pub fn insert(&mut self, key: usize, prio: f32) {
        self.heap.push(HeapElem { key, prio });
        self.positions.push((self.count() - 1) as i32);
        self.move_up(self.count() - 1);
    }

    pub fn delete_min(&mut self) -> usize {
        let result = self.min();
        self.heap.swap_remove(0);
        self.positions[result] = -1;
        if !self.is_empty() {
            self.move_down(0);
        }
        result
    }

    pub fn change_prio(&mut self, key: usize, new_prio: f32) {
        let j = self.positions[key] as usize;
        let old_prio = self.heap[j].prio;
        self.heap[j].prio = new_prio;
        if new_prio > old_prio {
            self.move_down(j);
        } else {
            self.move_up(j);
        }
    }

    pub fn clear(&mut self) {
        self.heap.clear();
        self.positions.clear();
    }

    pub fn is_empty(&self) -> bool {
        self.heap.is_empty()
    }

    fn count(&self) -> usize {
        self.heap.len()
    }

    fn min(&self) -> usize {
        if self.is_empty() {
            panic!("{}", "Empty heap");
        }
        self.heap[0].key
    }

    fn swap(&mut self, i: usize, j: usize) {
        self.heap.swap(i, j);
        self.positions[self.heap[i].key] = i as i32;
        self.positions[self.heap[j].key] = j as i32;
    }

    fn min_child_of(&self, i: usize) -> usize {
        let l = l_child(i);
        let r = r_child(i);
        let mut result = l;
        if r < self.count() && self.heap[r].prio < self.heap[l].prio {
            result = r;
        }
        result
    }

    fn move_up(&mut self, mut i: usize) {
        let mut p = parent(i as i32);
        while p >= 0 && self.heap[i].prio < self.heap[p as usize].prio {
            self.swap(i, p as usize);
            i = p as usize;
            p = parent(i as i32);
        }
    }

    fn move_down(&mut self, mut i: usize) {
        loop {
            let dst = self.min_child_of(i);
            if dst < self.count() && self.heap[dst].prio < self.heap[i].prio {
                self.swap(i, dst);
                i = dst;
            } else {
                break;
            }
        }
    }
}
