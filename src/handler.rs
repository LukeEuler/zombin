pub fn set_max_min_num(max: f32, min: f32, num: u32) {
    let mut actual_max = max;
    let mut actual_min = min;
    if max < min {
        actual_max = min;
        actual_min = max;
    }
    if num < 2 {
        return;
    }

    println!("amax: {}, amin: {}, num: {}", actual_max, actual_min, num);

    let span = actual_max - actual_min;
    let interval: f32 = span / num as f32;

    let max_tail = interval / (num - 1) as f32;

    let accuracy: f32 = get_head(max_tail);

    let actual_interval = interval - interval % accuracy + accuracy;

    actual_min = actual_min - actual_min % accuracy;
    actual_max = actual_min + actual_interval * num as f32;
    println!("y_min: {}, y_max {}, y_interval: {}",
             actual_min,
             actual_max,
             actual_interval);
}

fn get_head(value: f32) -> f32 {
    if value < 0 as f32 {
        let next: f32 = -value;
        return -get_head(next);
    }

    if value == 0 as f32 {
        return 1 as f32;
    }

    if value < 1 as f32 {
        let next: f32 = 10 as f32 * value;
        return 0.1 as f32 * get_head(next);
    }

    if value >= 10 as f32 {
        let next: f32 = value / 10 as f32;
        return 10 as f32 * get_head(next);
    }

    1 as f32
}
