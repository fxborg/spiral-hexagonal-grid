use std::ops::{Add,Sub,Mul};
use std::cmp::{Ord,PartialOrd,Ordering};
use std::hash::{Hash,Hasher};
// ヘキサゴン 構造体
#[derive(Debug, Copy, Clone)]
pub struct Hexagon {
    pub q: i64,
    pub r: i64,
    pub s: i64,
}

 impl Hexagon {
    // コンストラクタ
    pub fn new(q:i64,r:i64) -> Self {
        Hexagon {
            q: q,
            r: r,
            s: -q-r,
        }
    }
}
// オペレータを実装
impl PartialEq for Hexagon {
    fn eq(&self, othr: &Hexagon) -> bool {
        self.q == othr.q && self.r == othr.r
    }
}

impl Eq for Hexagon {}

impl PartialOrd for Hexagon {
    fn partial_cmp(&self, othr: &Hexagon) -> Option<Ordering> {
        Some(self.cmp(othr))
    }
}

//　ハッシュ関数
impl Hash for Hexagon {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.q.hash(state);
        self.r.hash(state);
    }
}

// 順序の評価
impl Ord for Hexagon {
    fn cmp(&self, othr: &Hexagon) -> Ordering {
        (self.q.abs()+self.r.abs()+self.s.abs()).cmp(
            &(othr.q.abs()+othr.r.abs()+othr.s.abs())
        )
    }
}

// 加算
impl Add for Hexagon {
    type Output = Hexagon;
    fn add(self, othr: Hexagon) -> Hexagon {
        Hexagon::new(self.q + othr.q, self.r + othr.r)
    }
}

// 減算
impl Sub for Hexagon {
    type Output = Hexagon;
    fn sub(self, othr: Hexagon) -> Hexagon {
        Hexagon::new(self.q - othr.q, self.r - othr.r)
    }

}

// 乗算
impl Mul<i64> for Hexagon {
    type Output = Hexagon;
    fn mul(self, n: i64) -> Hexagon {
        Hexagon::new(self.q * n, self.r * n)
    }
}
