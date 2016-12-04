fn word_wrap_solution(n: i32, line_size: i32, words_length: &Vec<i32>) -> i32 {
    let mut memoization_vec = vec![i16::max_value() as i32; (n + 2) as usize];
    memoization_vec[0] = 0;
    memoization_vec[1] = (line_size - words_length[0]).pow(2);
    for i in 1..n+2 {
        let mut line_cost = line_size + 1;
        let start = std::cmp::max(1, i - (line_size/2) as i32 +1);
        for j in (start..i+1).rev() {
            line_cost -= words_length[(j - 1) as usize] + 1;
            if line_cost < 0 {
                break;
            }

            let real_cost = line_cost.pow(2);
            if memoization_vec[(j - 1) as usize] + real_cost < memoization_vec[i as usize] {
                memoization_vec[i as usize] = memoization_vec[(j - 1) as usize] + real_cost;
            }
        }
    }

    println!("memo vec {:?}", memoization_vec);
    memoization_vec[n as usize]
}

fn main() {
    let words_length = vec![3, 2, 2, 5];
    word_wrap_solution(3, 6, &words_length);
}
