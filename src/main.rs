// [Rust入門書 : 第2章] https://doc.rust-jp.rs/book-ja/ch02-00-guessing-game-tutorial.html

// std(標準ライブラリ)であるio(入出力ライブラリ)
use std::io; // io(入出力)モジュール
use rand::Rng; //乱数モジュール
use std::cmp::Ordering; //比較モジュール

fn main() {
    // rand::thread_rng関数は gen_rangeメソッド(乱数生成器)を返す
    // gen_rangeメソッドは 第1引数-1 から 第2引数-1 までの乱数を生成する(OSからシード値を得て生成。)
    let secret_number = rand::thread_rng().gen_range(1, 101);

    /*println!("The secret number is: {}", secret_number);*/

    println!("出力されゑ数價を的中(しなさい) [ 0~100 ]"); // 数を当ててごらん

    loop {
        println!("早く入力レて！"); // ほら、予想を入力してね

        // ユーザ入力を取得する変数
        // Rust では変数はデフォルトで不変
        // "let mut" とすることで可変にできる
        // new関数がStringの関連関数*になっていることを :: が示す
        // *関連関数とは型(ここではstr)に対して実装された関数。静的(static)メソッドと呼ばれることもある
        // new関数とは新しく空の文字列を生成する関数
        // つまり空の文字列を可変変数guess に代入しているということになる
        let mut guess = String::new();

        // ioモジュールの stdin関数
        // 事前に　"use std::io" としない場合は関数を std::io::stdin() と記載する
        // stdin関数は std::io::Stdin OBJを返し、それはターミナルの標準入力への標準入力ハンドル(識別子)となる(?)
        // .read_line() は標準入力ハンドルの read_lineメソッドであり、入力された物を引数にある可変変数に投げる
        // & はその引数が参照であることを表す。Rust では楽に安全に参照を扱える
        // なお、参照もデフォルトで不変なので mut を付けて可変にする
        io::stdin().read_line(&mut guess)
            // この .expectメソッドは .read_lineメソッドに同じ行で連結出来るがこの様に改行することも可能
            // 前の .read_lineメソッド は io::Result(ioモジュールでのResult型)を返す
            // Result型はエラー情報をコード化する為のもので、列挙子(種類?)は Ok と Err
            // Ok は成功時のもので生成された値を持つ
            // Err は失敗時のもので失敗した過程や理由などをの情報を持つ
            // そして「ioモジュールのResult型」(io::Result)にはexpectメソッドがある
            // Result が Err値の場合 expectメソッドはプログラムを中止して引数にある物を表示する
            // Result が Ok値 の場合 expectメソッドは Ok値が保持する値を返す
            // なお、Result値を無視するとコンパイラに怒られる模様
            .expect("行ゐ読込に失敗レた！\n大變た！\n刂トラ亻."); // 行の読み込みに失敗しました

        // そして、guess (str)と secret_number (int)は型が違うので guess を以下のように...
        // guess という変数はすでに存在するが元の guess を基にして u32型のguess　に入れ替える
        // guess に trimメソッドが呼び出されていて、こいつが両端の空白や "\n" を全て消してくれる。
        // 何故消すのかというと u32型(32bit非負数字) には数字しか含めない故
        // ユーザが read_line の処理を終えるのに Enter を押す。すると改行文字(\n)が文字列に入ってしまう。
        // perseメソッドは天才的頭脳で数値型を示唆してくれる
        // ":" でコンパイラに変え先変数の型を注釈する合図をして u32(上限が低い自然数型)を注釈する
        // これで secret_number変数と比較ができる
        // match式については少し下の説明を参照しろ
        let guess: u32 = match guess.trim().parse() {
            // parseメソッドはエラりやすいので対処してあげる
            // match式がperseの返す列挙子[Ok || Err]に応じて分枝
            Ok(num) => num, // num (guessのu32型の奴)を返す
            Err(_) => { // "_" は何でもいいからマッチしたらという意味。
                println!("\n貴樣！\n数價を入力(しなさい)");
                continue; // ここでのcontinue はloopをやり直し。
            } 
        };

        // この {} はプレースホルダーという
        println!("貴樣ゐ予想: {}", guess); // 次のように予想しました: {}

        // std::cmp::Ordering (Ordering型)の列挙子(種類?)は Less, Greater, Equal
        // cmpメソッドで2値を比較できる。
        // "<比較対象1>.cmp(&<比較対象2>)" で比較して列挙子を返してくれるので match式* で列挙子に応じた処理をさせる
        // *match式とは複数のアームから出来ていて与えられた値が各アームと合致した場合そのアームが走る(run)
        // match式の文法は "match <与える値> { <値1> => <処理1>, <値2> => <処理2>, }" のような感じ
        // 処理部分は "{ <処理1>; <処理2>; }"" のようにして複数にできる
        // なお、比較対象1に cmoメソッドを呼び出して比較対象2を参照するので比較対象2には & を付けます
        // 不変で良いので mut は付けない
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("矮小過ぎゑてあゑ"), //小さすぎ！
            Ordering::Greater => println!("過巨大だ！！"), //大きすぎ！
            Ordering::Equal => {
                println!("貴樣ゐ勝利てず。\n賞金ゐ1000000000000元を贈呈ずゑ！！"); //やったね！
                break; // プログラムの終了
            }
        }
    }
}