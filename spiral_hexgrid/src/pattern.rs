use std::collections::HashMap;
use super::hexagon::Hexagon;

pub(crate) struct Pattern;

///　方位ごとの距離パターンが記憶される
impl Pattern {

    ///タプルからハッシュマップを生成
    fn to_hash(vals:&[(i8,i8)])->HashMap<i8,i8>{
        vals.iter().cloned().collect::<HashMap<_, _>>()
    }
    /// ｑ（-1〜1）とｒ（-1〜1）から一意な７つの番号（6,2,9,5,1,8,4）を生成
    pub(crate) fn get_id(hex:&Hexagon) -> i8 {
        if hex.q.abs()>1 || hex.r.abs()>1 {
            -1
         }else{
             let q:i8 =hex.q as i8;
             let r:i8 =hex.r as i8;
             ((q + 1) << 2) | (r + 1)
         }
    }

    /// 方位ごとの距離パターンを返す
    pub(crate) fn get(q:i64,r:i64,s:i64) -> HashMap<i8,i8> {
        if q == 0 && r == 0 && s==0 {        // 中央
            Self::to_hash(&[(6,1),(2,1),(9,1),(5,0),(1,1),(8,1),(4,1)])
        }else if q==0 || r==0 || s==0 {     // 6コーナー
            if q == 0 {
                if r > 0 {                  //北　角
                    Self::to_hash(&[(6,1),(2,1),(9,1),(5,0),(1,0),(8,0),(4,-1)])
                }else {                      //南　角
                    Self::to_hash(&[(6,-1), (2,0), (9,0), (5,0),(1,1),(8,1),(4,1)])
                }
            } else if r == 0 {
               if q > 0{                    //北東　角
                   Self::to_hash(&[(6,1),(2,0),(9,1),(5,0),(1,-1),(8,1),(4,0)])
               }else{                       //南西　角
                   Self::to_hash(&[(6,0),(2,1),(9,-1),(5,0),(1,1),(8,0),(4,1)])
               }
            }else {
                if r > 0{                   //北西　角
                   Self::to_hash(&[(6,1),(2,1),(9,0),(5,0),(1,1),(8,-1),(4,0)])
               }else{                       //南東　角
                   Self::to_hash(&[(6,0),(2,-1),(9,1),(5,0),(1,0),(8,1),(4,1)])
               }
            }
        }else{
            let q_:i64 = q.abs();
            let r_:i64 = r.abs();
            let s_:i64 = s.abs();

            if q_ > r_ && q_ > s_ {
                if q > 0 {		// 東　辺
                        Self::to_hash(&[(6,0),(2,-1),(9,1),(5,0),(1,-1),(8,1),(4,0)])
                }else{			// 西　辺
                        Self::to_hash(&[(6,0),(2,1),(9,-1),(5,0),(1,1),(8,-1),(4,0)])
                }
            }else{
                 if r_ > s_ {
                    if r > 0 {          //北西　辺
                            Self::to_hash(&[(6,1),(2,1),(9,0),(5,0),(1,0),(8,-1),(4,-1)])
                    }else{		//南東　辺
                            Self::to_hash(&[(6,-1),(2,-1),(9,0),(5,0),(1,0),(8,1),(4,1)])
                    }
                }else{
                    if s > 0 {          //南西　辺
                            Self::to_hash(&[(6,-1),(2,0),(9,-1),(5,0),(1,1),(8,0),(4,1)])
                    }else{		//北東　辺
                            Self::to_hash(&[(6,1),(2,0),(9,1),(5,0),(1,-1),(8,0),(4,-1)])
                    }
                }
            }
        }
    }
}


