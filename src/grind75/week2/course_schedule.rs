// Problem: https://leetcode.com/problems/course-schedule/

pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
    // equivalent to cycle detection in graph
    let n = num_courses as usize;
    let mut prereq_of_courses: Vec<Vec<i32>> = vec![vec![]; n];
    let mut prereq_counts = vec![0; n];
    let mut finishable_courses = Vec::<usize>::new();

    // Step 1 - Build graph
    for prereq in prerequisites {
        prereq_of_courses[prereq[1] as usize].push(prereq[0]);
        prereq_counts[prereq[0] as usize] += 1;
    }
    let mut count_finishable_courses = 0;

    // Step 2 - Get available course
    for i in 0..n {
        if prereq_counts[i] == 0 {
            finishable_courses.push(i);
            count_finishable_courses += 1;
        }
    }

    // Step 3 - Count available
    while let Some(course) = finishable_courses.pop() {
        while let Some(course) = prereq_of_courses[course].pop() {
            let course = course as usize;
            prereq_counts[course] -= 1;
            if prereq_counts[course] == 0 {
                finishable_courses.push(course);
                count_finishable_courses += 1;
            }
        }
    }

    count_finishable_courses == n
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn example_1() {
        let prerequisites = [[0, 1]].map(|array| array.to_vec()).to_vec();
        assert_eq!(can_finish(2, prerequisites), true);
    }

    #[test]
    fn example_2() {
        let prerequisites = [[0, 1], [1, 0]].map(|array| array.to_vec()).to_vec();
        assert_eq!(can_finish(2, prerequisites), false);
    }

    #[test]
    fn example_3() {
        let prerequisites = [[0, 1], [1, 2], [2, 1], [2, 0]].map(|array| array.to_vec()).to_vec();
        assert_eq!(can_finish(3, prerequisites), false);
    }

    #[test]
    fn example_4() {
        let prerequisites = [[0, 1], [2, 1], [2, 0]].map(|array| array.to_vec()).to_vec();
        assert_eq!(can_finish(3, prerequisites), true);
    }
}
