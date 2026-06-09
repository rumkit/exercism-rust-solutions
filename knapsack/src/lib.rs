use std::cmp::max;

#[derive(Debug, Clone)]
pub struct Item {
    pub weight: u32,
    pub value: u32,
}

pub fn maximum_value(max_weight: u32, items: &[Item]) -> u32 {
    let mut max_value = 0;

    if max_weight == 0 {
        return max_value;
    }

    let mut items = items.to_vec();
    for _ in 0..items.len() {
        let next_item = items.pop().unwrap();
        if next_item.weight <= max_weight {
            max_value = max(
                max_value,
                next_item.value + maximum_value(max_weight - next_item.weight, &items),
            );
        }
    }

    max_value
}
