fn word_wrap_solution(i: i32, m: i32, words_length: &Vec<i32>, memoization_vec: &mut Vec<i32>) -> Option<i32> {
    if i < 0 {
        return None;
    }

    if memoization_vec[i as usize] >= 0 {
        return Some(memoization_vec[i as usize]);
    }

    let result = (0..i+1).into_iter().map(|x| {
        let base_cost = m - words_length[x as usize];
        let remaining_cost = (x+1..i+1).into_iter().map(|j| 1 + words_length[j as usize]).sum::<i32>();
        let cost = base_cost - remaining_cost;
        let quadratic_cost;
        if cost < 0 {
            quadratic_cost = i16::max_value() as i32;
        } else {
            quadratic_cost = cost * cost;
        }

        match word_wrap_solution(x - 1, m, words_length, memoization_vec) {
            Some(result) => result + quadratic_cost,
            None => quadratic_cost
        }
    }).min();

    if let Some(save) = result {
        memoization_vec[i as usize] = save;
    }

    return result;
}

fn main() {
    let words_length = vec![3, 2, 2, 5];
    let mut memoization_vec = vec![-1; words_length.len()];
    println!("word_wrap_solution({}, {}, {:?}) = {:?}", 3, 6, words_length, word_wrap_solution(3, 6, &words_length, &mut memoization_vec));
    println!("memoization_vec = {:?}", memoization_vec);
}
