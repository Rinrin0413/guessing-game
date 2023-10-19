// Rust入門書第2章参照: https://doc.rust-jp.rs/book-ja/ch02-00-guessing-game-tutorial.html

// RNG
use rand::Rng;
// IO(入出力)モジュール, 比較モジュール
use std::{cmp::Ordering, io::stdin};

enum Status {
    // 問題を間違えてコンティニュー
    Continue,
    // クリア後にリスタートを選択した場合
    Restart,
    // クリア後にゲーム終了を選択した場合
    Quit,
}

fn main() {
    loop {
        // これが当てるべき数値
        // rand::thread_rng関数は乱数生成器を初期化する
        // gen_rangeメソッドは 第1引数-1 ~ 第2引数-1 までの範囲で乱数を生成する (OSからシード値を得て生成。)
        let secret_number = rand::thread_rng().gen_range(1, 101);

        println!("出力されゑ整数値を的中(しなさい) [ 0~100 ]");

        // 比較後の分岐用
        let mut status: Status;

        loop {
            let guess = if let Ok(n) = read() {
                n
            } else {
                continue;
            };

            println!("貴樣ゐ予想: {}", guess);
            
            // cmpメソッドは引数に与えられた値と比較して std::cmp::Ordering列挙型の列挙子を返す
            // Ordering列挙型の列挙子は Less, Greater, Equal の3つがある
            // match文を使って列挙子に応じた処理を行う
            status = compare(guess.cmp(&secret_number));

            // 当てられなかった場合はこのループの先頭に回帰
            if let Status::Continue = status {
                continue;
            }
            
            // それ以外(つまりクリア後)の場合はこのループを抜ける
            break;
        }

        // ユーザーがリスタートを選択した場合はこのループの先頭に回帰
        if let Status::Restart = status {
            continue;
        }

        // それ以外(つまりゲーム終了)の場合はこのループを抜ける
        break;
    }

    // ここまで来たらプログラム終了
}

fn read() -> Result<i32, ()> {
    println!("早く入力レて！");

    // ユーザーの入力を持たせる変数
    let mut guess = String::new();

    // stdin関数の返す std::io::Stdin型は ターミナルの標準入力(stdin)へのハンドル
    stdin()
        // read_lineメソッドは ユーザーに入力された文字列を 引数として指定された可変変数に格納する
        .read_line(&mut guess)
        // 読み取りに失敗した場合はパニックさせる
        .expect("行ゐ読込に失敗レた！\n大變た！\n刂トラ亻.");

    // guess変数(String)と secret_number変数(u32)では型が異っていて比較できない
    // なので guess変数を u32型にシャドーイングで変換する
    // trimメソッドは両端の空白や "\n" を全て消してくれる
    // ユーザが入力を決定して Enter を押した際に改行文字(\n)が文字列に含まれてしまうため trimメソッドで取り除く
    // perseメソッドは天才的推論で文字列を数値に変換してくれる
    let guess: i32 = if let Ok(n) = guess.trim().parse() {
        // 0未満か100超過の場合は ユーザーに伝えて Err値を返して その先で continue でループをやり直す
        if !(0..=100).contains(&n) {
            eprintln!("範囲外ゐ値 {} を取得, 0~100まてゐ整数を入力(しなさい)", n);
            return Err(());
        }
        n
    } else {
        // エラーが起きた場合は 問題をユーザーに伝えて Err値を返して その先で continue でループをやり直す
        eprintln!("\n貴樣！\n数價を入力(しなさい)");
        return Err(());
    };
    
    Ok(guess)
}

fn compare(ord: Ordering) -> Status {
    match ord {
        Ordering::Less => println!("矮小過ぎアル！"),
        Ordering::Greater => println!("過巨大だ！！"),
        Ordering::Equal => {
            println!("貴樣ゐ勝利てず。\n賞金ゐ1000000000000元を贈呈ずゑ！！");

            // クリア後の応答待機
            loop {
                println!("もラ一度游ぶ？ [y/n]");

                let mut yn = String::new();
                stdin()
                    .read_line(&mut yn)
                    .expect("何力問題が発生レだ！");

                match yn.trim() {
                    "y" => return Status::Restart,
                    "n" => return Status::Quit,
                    _ => continue,
                }
            }
        }
    }

    Status::Continue
}