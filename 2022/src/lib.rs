pub fn bubble_sort(items: &mut [u32]) {
    let max_index: usize = items.len() - 1;
    let mut num_changes = 0;

    loop {
        let mut idx = 0;

        while idx < max_index {
            if items[idx] < items[idx + 1] {
                let temp = items[idx];
                items[idx] = items[idx + 1];
                items[idx + 1] = temp;
                num_changes += 1;
            }
            idx += 1;
        }

        if num_changes == 0 {
            break;
        }

        num_changes = 0;
    }
}
