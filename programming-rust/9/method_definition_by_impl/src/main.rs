// 文字の先入れ先出しキュー
pub struct Queue {
    // 古い要素たち、最も古いものが最後
    older: Vec<char>,
    // 新しい要素たち、最も新しいものが最後
    younger: Vec<char>,
}

impl Queue {
    // 文字をキューの末尾にプッシュ
    pub fn push(&mut self, c: char) {
        self.younger.push(c);
    }

    // キューの先端から文字をポップする。ポップする文字があれば、Some(c)を返す。
    // 殻ならNoneを返す。
    pub fn pop(&mut self) -> Option<char> {
        if self.older.is_empty() {
            if self.younger.is_empty() {
                return None;
            }

            use std::mem::swap;
            swap(&mut self.older, &mut self.younger);
            self.older.reverse();
        }
        // ここにきたら、olderには何かが入っているはず。
        // VecのpopメソッドをはOptionを返すので、そののまま返す。
        self.older.pop()
    }

    pub fn is_empty(&self) -> bool {
        self.older.is_empty() && self.younger.is_empty()
    }
    // move
    pub fn split(self) -> (Vec<char>, Vec<char>) {
        (self.older, self.younger)
    }

    pub fn new() -> Queue {
        Queue { older: Vec::new(), younger: Vec::new() }
    }
}

fn main() {
    println!("Hello, world!");
}

#[test]
fn test_queue() {
    let mut q = Queue::new();
    q.push('0');
    q.push('1');
    assert_eq!(q.pop(), Some('0'));

    q.push('∞');
    assert_eq!(q.pop(), Some('1'));
    assert_eq!(q.pop(), Some('∞'));
    assert_eq!(q.pop(), None);

    assert!(q.is_empty());
    q.push('2');
    assert!(!q.is_empty());
    q.pop();


    q.push('P');
    q.push('D');
    assert_eq!(q.pop(), Some('P'));
    q.push('X');

    let (older, younger) = q.split();
    assert_eq!(older, vec!['D']);
    assert_eq!(younger, vec!['X']);
}
