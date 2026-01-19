pub fn get_tick_group_index(tick_index: i32, tick_group_size: u16) -> i32 {
    if tick_index < 0 && tick_index % tick_group_size as i32 != 0 {
        return (tick_index / tick_group_size as i32) - 1;
    }

    tick_index / tick_group_size as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_tick_group_index() {
        assert_eq!(get_tick_group_index(0, 64), 0);
        assert_eq!(get_tick_group_index(63, 64), 0);
        assert_eq!(get_tick_group_index(64, 64), 1);
        assert_eq!(get_tick_group_index(-1, 64), -1);
        assert_eq!(get_tick_group_index(-64, 64), -1);
        assert_eq!(get_tick_group_index(-65, 64), -2);
        assert_eq!(get_tick_group_index(-129, 64), -3);
    }
}
