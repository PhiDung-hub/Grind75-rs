// Problem:

pub fn can_attend_all_meetings(mut scheduled_meetings: Vec<[i32; 2]>) -> bool {
    if scheduled_meetings.len() < 2 {
        return true;
    }

    scheduled_meetings.sort_by(|meeting_1, meeting_2| meeting_1[0].cmp(&meeting_2[0]));

    let mut end = scheduled_meetings[0][1];
    for i in 1..scheduled_meetings.len() {
        if scheduled_meetings[i][0] < end {
            return false;
        }

        end = end.max(scheduled_meetings[i][1]);
    }

    true
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(can_attend_all_meetings(vec![[1, 3], [5, 7]]), true);
    }

    #[test]
    fn example_2() {
        assert_eq!(can_attend_all_meetings(vec![[1, 3], [5, 7], [3, 4], [4, 16], [7, 10], [12, 20]]), false);
    }

    #[test]
    fn example_3() {
        assert_eq!(can_attend_all_meetings(vec![[1, 3]]), true);
        assert_eq!(can_attend_all_meetings(vec![]), true);
    }
}
