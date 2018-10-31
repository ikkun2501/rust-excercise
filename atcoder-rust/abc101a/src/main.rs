macro_rules! input {
    // ?
    (source = $s:expr, $($r:tt)*) => {
        let mut iter = $s.split_whitespace();
        let mut next = || { iter.next().unwrap() };
        input_inner!{next, $($r)*}
    };

    // stdinの生成 値の読み込み開始
    ($($r:tt)*) => {
        let stdin = std::io::stdin();
        let mut bytes = std::io::Read::bytes(std::io::BufReader::new(stdin.lock()));
        let mut next = move || -> String{
            bytes
                .by_ref()
                .map(|r|r.unwrap() as char)
                .skip_while(|c|c.is_whitespace())
                .take_while(|c|!c.is_whitespace())
                .collect()
        };
        input_inner!{next, $($r)*}
    };
}

macro_rules! input_inner {
    // ?
    ($next:expr) => {};
    // ?
    ($next:expr, ) => {};

    // 変数名:型の読み込みの繰り返し
    ($next:expr, $var:ident : $t:tt $($r:tt)*) => {
        let $var = read_value!($next, $t);
        input_inner!{$next $($r)*}
    };
}

macro_rules! read_value {
    // 値を読み込むベース 型,の繰り返し
    ($next:expr, ( $($t:tt),* )) => {
        ( $(read_value!($next, $t)),* )
    };

    // 配列の読み込み
    ($next:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    };

    // 文字列Vec<char>の読み込み
    ($next:expr, chars) => {
        read_value!($next, String).chars().collect::<Vec<char>>()
    };

    // usize1?の読み込み
    ($next:expr, usize1) => {
        read_value!($next, usize) - 1
    };

    // 指定した型で読み込み
    ($next:expr, $t:ty) => {
        $next().parse::<$t>().expect("Parse error")
    };
}

macro_rules! tests {
    ($($test_name:ident:$name:ident($( $args:expr ),*) => $expected:expr,)*) => {
        mod tests {
            $(
                #[test]
                fn $test_name() {
                    assert_eq!($expected,super::$name($($args),*));
                }
            )*
        }
    }
}

fn main() {
    input! {
        chars: chars,
    };

    println!("{}", solve(chars));
}

fn solve(chars: Vec<char>) -> i64{
    let mut ans: i64 = 0;
    for c in chars {
        match c {
            '+' => ans += 1,
            _ => ans -= 1,
        }
    }
    return ans;
}




tests! {
    //テスト名:[引数]=>expected
    test1:solve("+-++".chars().collect())=>2,
    test2:solve("-+--".chars().collect())=>-2,
    test3:solve("----".chars().collect())=>-4,
}
