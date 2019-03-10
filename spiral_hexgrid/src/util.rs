use std::collections::HashMap;
use std::cmp::max;
use super::hexagon::Hexagon;
use super::pattern::Pattern;

pub(crate) struct Util;

impl Util {

    /// グリッド上の距離を計算
    pub(crate) fn hex_distance(hex:&Hexagon)->i32{
            max(max(hex.q.abs(), hex.r.abs()), hex.s.abs()) as i32
    }

    /// 実数座標をグリッド座標へ変換
    pub(crate) fn round_hex(fq:f64, fr:f64, fs:f64)->Hexagon{
        let q = fq.round();
        let r = fr.round();
        let s = fs.round();
        let q_diff = (q - fq).abs();
        let r_diff = (r - fr).abs();
        let s_diff = (s - fs).abs();

        if q_diff > r_diff && q_diff > s_diff {
            Hexagon::new((-r - s) as i64, r as i64)
        }else if r_diff > s_diff{
            Hexagon::new(q as i64, (-q - s) as i64)
        }else{
            Hexagon::new(q as i64, r as i64)
        }
    }

    /// インデックスを利用した距離の計算
    pub(crate) fn measure_distance(

            hexagons:&Vec<Hexagon>,
            center_pos:&Hexagon,
            pattern: &HashMap<i8,i8>,
            distance: i32
            ) -> HashMap<i32, Vec<Hexagon>>{

        let mut rslt:HashMap<i32, Vec<Hexagon>>=HashMap::new();

        for hex in hexagons{
            let id:i8 = Pattern::get_id(&(*hex - *center_pos));

            if let Some(val) = pattern.get(&id){
                let dist:i32 = distance + (*val as i32);
                let  elem = rslt.entry(dist).or_insert(Vec::new());
                (*elem).push(hex.clone());

            }else{

            }
        }
        rslt

    }
    /// 中心グリッドを返す
    pub(crate) fn belongs_to_center(h:&Hexagon) -> Vec<(Hexagon,u8)> {
        let n:i8 = Self::hex_to_seven_num(h);
        match n{
            0 => vec![(Hexagon::new( 0, 0),0b10111111)]/*中央*/ ,
            1 => vec![(Hexagon::new( 0, 1),0b10100011)/*南　*/,(Hexagon::new( 1,-2),0b00001100)/*北北西*/,(Hexagon::new(-2, 0),0b00010000)/*北西２*/],
            2 => vec![(Hexagon::new( 1, 0),0b10000111)/*南西*/,(Hexagon::new(-1,-1),0b00011000)/*北北東*/,(Hexagon::new(-2, 2),0b00100000)/*南東２*/],
            3 => vec![(Hexagon::new( 1,-1),0b10001110)/*北西*/,(Hexagon::new(-2, 1),0b00110000)/*東２　*/,(Hexagon::new( 0, 2),0b00000001)/*南２　*/],
            4 => vec![(Hexagon::new( 0,-1),0b10011100)/*南　*/,(Hexagon::new(-1, 2),0b00100001)/*南南東*/,(Hexagon::new( 2, 0),0b00000010)/*南西２*/],
            5 => vec![(Hexagon::new(-1, 0),0b10111000)/*北東*/,(Hexagon::new( 1, 1),0b00000011)/*南南西*/,(Hexagon::new( 2,-2),0b00000100)/*北西２*/],
            6 => vec![(Hexagon::new(-1, 1),0b10110001)/*南東*/,(Hexagon::new( 2,-1),0b00000110)/*西２　*/,(Hexagon::new( 0,-2),0b00001000)/*北２　*/],
            _ => vec![(Hexagon::new( 0, 0),0b10111111)]/*中央*/ ,
        }
    }

    /// ７ナンバーを返す
    pub(crate) fn hex_to_seven_num(h:&Hexagon)->i8 {
        Self::to_seven_num(Self::mod7(h.q - h.r*2))
    }

    /// 対応表
    pub(crate) fn to_seven_num(n:i8)->i8 {
        match n {
            0 => 0,
            1 => 5,
            2 => 1,
            3 => 6,
            4 => 3,
            5 => 4,
            6 => 2,
            _ => 0,
        }
    }

    /// フィルタ
    pub(crate) fn get_filter(n:i8)->u8 {
       match n {
           0 => 0b10000000,
           1 => 0b00000001,
           2 => 0b00000010,
           3 => 0b00000100,
           4 => 0b00001000,
           5 => 0b00010000,
           6 => 0b00100000,
           _ => 0b10000000,
        }
    }
    /// ７で除算
    fn mod7(a:i64) -> i8 {
        if a >= 0 { (a % 7) as i8 } else { (7 - ((-a) % 7)) as i8 }
    }


}
