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
fn main() {
    input! {
        n: usize,
        nums : [u64;n],
    };


    println!("{}", solve(nums));
}

fn solve(mut nums: Vec<u64>) -> usize {
    let mut counter: usize = 0;
    while nums.iter().all(|x| x % 2 == 0) {
        counter = counter + 1;
        nums = nums.into_iter().map(|x| x / 2).collect()
    }
    return counter;
}

macro_rules! tests {
    ($($name:ident:[$( $args:expr ),*] => $output:expr,)*) => {
        mod tests {
            $(
                #[test]
                fn $name() {
                    assert_eq!($output,super::solve($($args),*));
                }
            )*
        }
    }
}


tests! {
    //テスト名:[引数]=>expected
    test1: [vec![8, 12, 40]] => 2,
    test2: [vec![5, 6, 8, 10]] => 0,
    test3: [vec![382253568, 723152896, 37802240, 379425024, 404894720, 471526144]]=> 8,
}
