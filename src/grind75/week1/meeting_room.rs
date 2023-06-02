// Problem:

pub fn can_attend_all_meetings(mut scheduled_meetings: Vec<[i32; 2]>) -> bool {
    if scheduled_meetings.len() < 2 {
        return true;
    }

    scheduled_meetings.sort_by(|meeting_1, meeting_2| meeting_1[0].cmp(&meeting_2[0]));

    let mut end = scheduled_meetings[0][1];
    for meeting in scheduled_meetings.iter().skip(1) {
        if meeting[0] < end {
            return false;
        }

        end = end.max(meeting[1]);
    }

    true
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn example_1() {
        assert!(can_attend_all_meetings(vec![[1, 3], [5, 7]]));
    }

    #[test]
    fn example_2() {
        assert!(!can_attend_all_meetings(vec![[1, 3], [5, 7], [3, 4], [4, 16], [7, 10], [12, 20]]));
    }

    #[test]
    fn example_3() {
        assert!(can_attend_all_meetings(vec![[1, 3]]));
        assert!(can_attend_all_meetings(vec![]));
    }
}
