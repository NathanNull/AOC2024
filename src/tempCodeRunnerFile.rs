if target_pos.0 < curr_pos.0 {
            dirs.extend(vec!['<'; (curr_pos.0 - target_pos.0) as usize])
        }
        if target_pos.0 > curr_pos.0 {
            dirs.extend(vec!['>'; (target_pos.0 - curr_pos.0) as usize])
        }