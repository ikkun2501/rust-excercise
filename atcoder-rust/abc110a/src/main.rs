macro_rules! input {
    (source = $s:expr, $($r:tt)*) => {
        let mut iter = $s.split_whitespace();
        let mut next = || { iter.next().unwrap() };
        input_inner!{next, $($r)*}
    };
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
    ($next:expr) => {};
    ($next:expr, ) => {};

    ($next:expr, $var:ident : $t:tt $($r:tt)*) => {
        let $var = read_value!($next, $t);
        input_inner!{$next $($r)*}
    };
}

macro_rules! read_value {

    ($next:expr, ( $($t:tt),* )) => {
        ( $(read_value!($next, $t)),* )
    };

    ($next:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    };

    ($next:expr, chars) => {
        read_value!($next, String).chars().collect::<Vec<char>>()
    };

    ($next:expr, usize1) => {
        read_value!($next, usize) - 1
    };

    ($next:expr, $t:ty) => {
        $next().parse::<$t>().expect("Parse error")
    };
}
fn main() {
    input! {
          n: [usize;3]
    };

    let output = solve(n[0], n[1], n[2]);
    println!("{}", output);
}


fn solve(a: usize, b: usize, c: usize) -> usize {
    use std::cmp;
    let max: usize = cmp::max(cmp::max(a, b), c);
//    let max: &usize = vec![&a, &b, &c].iter().max().unwrap();
    return max * 10 - max + a + b + c;
}


// 通常テスト
//#[cfg(test)]
//mod tests {
//    use super::*;
//
//    #[test]
//    fn solve_test() {
//        assert_eq!(108, solve(9, 9, 9));
//    }
//}

// マクロで複数の引数を１個１個定義したテスト
//macro_rules! tests {
//    ($($name:ident:$a:expr,$b:expr,$c:expr => $output:expr,)*) => {
//        mod tests {
//            $(
//                #[test]
//                fn $name() {
//                    assert_eq!($output,super::solve($a,$b,$c));
//                }
//            )*
//        }
//    }
//}
//
//tests! {
//    test1:1,5,2=>53,
//    test2:9,9,9=>108,
//    test3:6,6,7=>82,
//}

// マクロで引数を複数渡したテスト
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
    test1:[1,5,2]=>53,
    test2:[9,9,9]=>108,
    test3:[6,6,7]=>82,
}
