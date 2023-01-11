// Problem: https://leetcode.com/problems/flood-fill/
pub fn flood_fill(image: Vec<Vec<i32>>, sr: i32, sc: i32, color: i32) -> Vec<Vec<i32>> {
    if image.is_empty() || image[0].is_empty() {
        panic!("Invalid input image");
    }

    let target_color = image[sr as usize][sc as usize];
    let (m, n) = (image.len(), image[0].len());

    let mut cell_to_flood: Vec<(usize, usize)> = Vec::new();
    cell_to_flood.push((sr as usize, sc as usize));

    let mut result_image = image;
    result_image[sr as usize][sc as usize] = color;
    let mut visited = vec![vec![false; n]; m];

    while let Some((this_r, this_c)) = cell_to_flood.pop() {
        let (prev_r, next_r, prev_c, next_c) = (this_r > 0, this_r < m - 1, this_c > 0, this_c < n - 1);

        let mut process_flood_cell = |next_r: usize, next_c: usize| {
            if !visited[next_r][next_c] {
                if result_image[next_r][next_c] == target_color {
                    cell_to_flood.push((next_r, next_c));
                    result_image[next_r][next_c] = color;
                }
                visited[next_r][next_c] = true;
            }
        };

        if prev_r {
            process_flood_cell(this_r - 1, this_c);
        }
        if next_r {
            process_flood_cell(this_r + 1, this_c);
        }
        if prev_c {
            process_flood_cell(this_r, this_c - 1);
        }
        if next_c {
            process_flood_cell(this_r, this_c + 1);
        }
    }

    result_image
}

// pub fn main() {
//     fn plus(x: &mut i32) -> i32 {
//         *x += 5;
//         *x
//     }
//     let mut x = 5;
//     plus(&mut x);
//     println!("{}", x);
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unique_target_cell() {
        let input_image = vec![vec![0, 0, 0], vec![0, 1, 0], vec![0, 0, 0]];
        let output_image = vec![vec![0, 0, 0], vec![0, 2, 0], vec![0, 0, 0]];
        assert_eq!(flood_fill(input_image, 1, 1, 2), output_image);
    }

    #[test]
    fn typical_case() {
        let input_image = vec![vec![1, 1, 1], vec![1, 1, 0], vec![1, 0, 1]];
        let output_image = vec![vec![2, 2, 2], vec![2, 2, 0], vec![2, 0, 1]];
        assert_eq!(flood_fill(input_image, 1, 1, 2), output_image);
    }

    #[test]
    fn all_same_as_target_cell_color() {
        let input_image = vec![vec![1, 1, 1], vec![1, 1, 1], vec![1, 1, 1]];
        let output_image = vec![vec![2, 2, 2], vec![2, 2, 2], vec![2, 2, 2]];
        assert_eq!(flood_fill(input_image, 1, 1, 2), output_image);
    }

    #[test]
    fn all_same_as_fill_color() {
        let input_image = vec![vec![1, 1, 1], vec![1, 1, 1], vec![1, 1, 1]];
        let output_image = vec![vec![1, 1, 1], vec![1, 1, 1], vec![1, 1, 1]];
        assert_eq!(flood_fill(input_image, 1, 1, 1), output_image);
    }
}
