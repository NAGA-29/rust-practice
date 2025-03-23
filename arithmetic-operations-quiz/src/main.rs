use rand::Rng;

fn main() {
    let mut num_of_correct = 0;
    while num_of_correct < 3 {
        let quiz_mode = rand::thread_rng().gen_range(1..=2);
        // P79 add
        match quiz_mode {
            1 => loop {
                let op1 = rand::thread_rng().gen_range(0..100);
                let op2 = rand::thread_rng().gen_range(0..100);

                println!("{} + {} = ??", op1, op2);
                println!("?? の値を入力してください");
                let mut ans_input = String::new();

                // P65
                // std::io::stdin().read_line(&mut ans_input).unwrap();
                // dbg!(ans_input);

                std::io::stdin().read_line(&mut ans_input).unwrap();

                // 　ans_inputからtrim()で改行を取り除き、parse()で整数(u32)に変換
                let ans_input = ans_input.trim().parse::<i32>().unwrap();

                dbg!(ans_input);

                if ans_input == op1 + op2 {
                    println!("正解です");
                    num_of_correct += 1;
                    break;
                    // if num_of_correct >= 3 {
                    //     println!("おめでとうございます！3問連続で正解しました！");
                    //     break;
                    // };
                } else {
                    println!("不正解です");
                }
            }
            2 => loop {
                // P69 減算
                let op1 = rand::thread_rng().gen_range(0..100);
                let op2 = rand::thread_rng().gen_range(0..100);

                println!("{} - {} = ??", op1, op2);
                println!("?? の値を入力してください");
                let mut ans_input = String::new();

                std::io::stdin().read_line(&mut ans_input).unwrap();

                // 　ans_inputからtrim()で改行を取り除き、parse()で整数(u32)に変換
                let ans_input = ans_input.trim().parse::<i32>().unwrap();

                dbg!(ans_input);

                if ans_input == op1 - op2 {
                    println!("正解です");
                    num_of_correct += 1;
                    break;
                    // if num_of_correct >= 3 {
                    //     println!("おめでとうございます！3問連続で正解しました！");
                    //     break;
                    // };
                } else {
                    println!("不正解です");
                }
            }
            _ => {
                unreachable!();
            } //     println!(
              //         "i32が扱えるデータ範囲: {} ~ {}",
              //         std::i32::MIN,
              //         std::i32::MAX
              //     );
              //     println!(
              //         "u32が扱えるデータ範囲: {} ~ {}",
              //         std::u32::MIN,
              //         std::u32::MAX
              //     );
              // }
        }
    }
    println!("クリア!");
}
