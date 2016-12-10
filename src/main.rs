fn word_wrap_solution(n: usize, line_size: i32, words_length: &Vec<i32>) -> Vec<i32> {
    let mut print_vec = vec![0; n + 1];

    let mut memoization_vec = vec![i16::max_value() as i32; (n + 2) as usize];
    memoization_vec[0] = 0;
    memoization_vec[1] = (line_size - words_length[0]).pow(2);
    for i in 1..n + 2 {
        let mut line_cost = line_size + 1;
        let start = std::cmp::max(1, i as i32 - (line_size / 2) + 1);
        for j in (start..i as i32 + 1).rev() {
            line_cost -= words_length[(j - 1) as usize] + 1;
            if line_cost < 0 {
                break;
            }

            let real_cost = line_cost.pow(2);
            if memoization_vec[(j - 1) as usize] + real_cost < memoization_vec[i as usize] {
                memoization_vec[i] = memoization_vec[(j - 1) as usize] + real_cost;
                print_vec[i - 2] = j - 1;
            }
        }
    }

    print_vec[n] = print_vec[n - 1] + 1;
    print_vec
}

fn pretty_print(print_vec: &Vec<i32>, words_vec: &Vec<String>) {
    for i in 0..print_vec.len()-1 {
        for word in print_vec[i]..print_vec[i+1] {
            print!("{} ", words_vec[word as usize]);
        }
        print!("\n");
    }
}

fn main() {
    let line_or_panic = || -> String {
        let mut line = String::new();
        if let Err(error) = std::io::stdin().read_line(&mut line) {
            panic!("Couldn't read line. Error: {}", error);
        }

        String::from(line.trim())
    };

    let word_wrap_col = line_or_panic().parse::<i32>().unwrap_or(0);
    if word_wrap_col == 0 {
        println!("Word wrap column can only be a number and higher than zero");
        return;
    }

    let words_number = line_or_panic().parse::<usize>().unwrap_or(0);
    if words_number == 0 {
        println!("Specify words number");
        return;
    }

    let words_vec: Vec<String> = (0..words_number).into_iter().map(|_| line_or_panic()).collect();
    if words_vec.iter().any(|ref x| x.len() > word_wrap_col as usize) {
        println!("One or more words are greater than word wrap column size");
        return;
    }

    let print_vec =
        word_wrap_solution(words_number - 1,
                           word_wrap_col,
                           &words_vec.iter().map(|ref x| x.len() as i32).collect::<Vec<i32>>());

    pretty_print(&print_vec, &words_vec);
}
