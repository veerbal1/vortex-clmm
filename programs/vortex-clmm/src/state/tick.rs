pub fn is_valid_tick(tick: i32, tick_spacing: u16) -> bool {
    tick % tick_spacing as i32 == 0
}

pub fn get_start_tick_index(tick: i32, tick_spacing: u16) -> i32 {
    let ticks_per_array = 88 * tick_spacing as i32;

    let mut start = (tick / ticks_per_array) * ticks_per_array;

    if tick < 0 && tick % ticks_per_array != 0 {
        start -= ticks_per_array;
    }

    start
}
