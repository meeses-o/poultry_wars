//Created by mujib
//2026-05-22
//Used to calculate rank given a string

pub fn calc_rank(rank: &String) -> String {
    let rank_int = rank.parse::<i32>().unwrap();
    if rank_int < 15 {
        return "1".to_string();
    }else if rank_int < 30 {
        return "2".to_string();
    }else if rank_int < 50 {
        return "3".to_string();
    }else if rank_int < 65 {
        return "4".to_string();
    }else {
        return "5".to_string();
    }
}