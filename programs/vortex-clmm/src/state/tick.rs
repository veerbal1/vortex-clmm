pub fn is_valid_tick(tick: i32, tick_spacing: u16) -> bool {
    tick % tick_spacing as i32 == 0
}
