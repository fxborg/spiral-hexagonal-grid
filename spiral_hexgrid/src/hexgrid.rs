
use std::collections::HashMap;
use super::hexagon::Hexagon;
use super::util::Util;
use super::pattern::Pattern;

/// 六角形の向き（水平、垂直 ）
#[derive(Clone, Copy, PartialEq)]
pub enum Layout{
        HORIZONTAL, // Flat Hexagon
        VERTICAL,  // Pointy Hexagon
}

/// pub const  HEXAGON_MIN:i64 = -92233720368547758; // INT_64_MIN*0.01
/// pub const  HEXAGON_MAX:i64 =  92233720368547758; // INT_64_MAX*0.01

/// 係数
#[derive(Clone, Copy)]
struct Orientation{
    f0:f64,
    f1:f64,
    f2:f64,
    f3:f64,
    b0:f64,
    b1:f64,
    b2:f64,
    b3:f64,

}


impl Orientation {
    /// コンストラクタ
    pub(crate) fn new(layout:Layout) -> Self {
        let args = if layout == Layout::HORIZONTAL {
            (3.0 / 2.0, 0.0, 3.0_f64.sqrt() / 2.0, 3.0_f64.sqrt(), 2.0 / 3.0, 0.0, -1.0 / 3.0, 3.0_f64.sqrt() / 3.0 )
        }else{
            (3.0_f64.sqrt(), 3.0_f64.sqrt() / 2.0, 0.0, 3.0 / 2.0, 3.0_f64.sqrt() / 3.0, -1.0 / 3.0, 0.0, 2.0 / 3.0 )
        };

        Orientation{
        f0 : args.0,
        f1 : args.1,
        f2 : args.2,
        f3 : args.3,
        b0 : args.4,
        b1 : args.5,
        b2 : args.6,
        b3 : args.7
        }
    }
}
/// ヘキサゴングリッド
pub struct HexGrid{
    radius:f64,
    ori:Orientation,
    positions:[Hexagon; 7],
    directions:[Hexagon; 7],
    indexes: HashMap<Hexagon, HashMap<Hexagon,u8>>,
    hexagon_map:HashMap<Hexagon, Vec<(f64,f64)>>,
}

impl HexGrid {
    /// コンストラクタ
    pub fn new(radius:f64,layout:Layout) -> Self {
        HexGrid{
            radius: radius, // グリッドサイズ
            ori: Orientation::new(layout), //係数
            positions:[Hexagon::new( 0, 0),	// 中央　位置
                      Hexagon::new( 0,-1),	// 南　　位置
                      Hexagon::new(-1, 0),	// 南西　位置
                      Hexagon::new(-1, 1),	// 北西　位置
                      Hexagon::new( 0, 1),	// 北　　位置
                      Hexagon::new( 1, 0),	// 北東　位置
                      Hexagon::new( 1,-1)],	// 南東　位置

            directions:[Hexagon::new( 0, 0),	// 中央
                        Hexagon::new( 1,-3),	// 南
                        Hexagon::new(-2,-1),	// 南西
                        Hexagon::new(-3, 2),	// 北西
                        Hexagon::new(-1, 3),	// 北
                        Hexagon::new( 2, 1),	// 北東
                        Hexagon::new( 3,-2)],	// 南東

            indexes:HashMap::new(), //インデックス
            hexagon_map:HashMap::new(), // グリッド
        }
    }

    /// 円環状にセルを移動しながら点群が存在する座標を調る
    fn walk_on_ring(&self, center:&Hexagon, step:i32)->Vec<Hexagon>{
        if step < 0 {return vec![]};
        if step == 0 {return vec![*center]};
        let mut results=vec![];
        // 5の方角に移動する。
        let mut hex:Hexagon = *center + (self.directions[5] * step as i64);
        for i in 1..7 {// ６回方向を変える
             for _j in 0..step { // 辺の長さ分直進する
                  hex = self.directions[i] + hex;
                  results.push(hex);
             }
        }
        results
    }

    /// 近傍点を探索
    pub fn neighbors(&self,origin:&Hexagon, distance:f64)-> HashMap<i32, Vec<Hexagon>>{
        // 点群が存在するグリッド座標を格納する
        let mut merge_results:HashMap<i32, Vec<Hexagon>>= HashMap::new();
        // グリッド間の距離の計算
        let grid_size:f64 = 2.0_f64 * self.radius * 3.0_f64.sqrt();
        // 探索対象範囲の上限
        let limit:i32 = (distance / grid_size).ceil() as i32 + 1_i32;
        // 起点のインデックス位置を確認
        let seven_num:i8 = Util::hex_to_seven_num(origin);
        let relative_pos:Hexagon = self.positions[seven_num as usize]; //インデックス位置からの座標
        let origin_index_pos:Hexagon  = *origin - relative_pos;//インデックス位置
        let filter:u8 = Util::get_filter(seven_num);//インデックス用フィルタ

        let mut n:i32 = 0;
        while 3*n-1 <= limit{
            let hexagons:Vec<Hexagon> = self.walk_on_ring(&origin_index_pos, n);

            for hex in hexagons {
                let dist:i32 = Util::hex_distance(&(origin_index_pos - hex));

                let results:HashMap<i32, Vec<Hexagon>> = self.get_hexagons(&origin_index_pos, &relative_pos, filter,&hex, dist);
                for (k, v) in &results {
                    let mut points = merge_results.entry(*k).or_insert(vec![]);
                    points.extend(v.iter().cloned());
                }
           }
           n+=1;

        }
        merge_results

    }
    /// グリッド座標からピクセルへ変換
    pub fn hex_to_pixel(&self, h:&Hexagon)->(f64,f64){
            let x:f64 = ((self.ori.f0 * h.q as f64 + self.ori.f1 * h.r as f64)*self.radius) as f64;
            let y:f64 = ((self.ori.f2 * h.q as f64 + self.ori.f3 * h.r as f64)*self.radius) as f64;
            (x, y)
    }
    /// ピクセルからグリッド座標へ変換
    pub fn pixel_to_hex(&self, x:f64, y:f64)-> Hexagon{
            let x_= x / self.radius;
            let y_= y / self.radius;
            let q:f64 = (self.ori.b0 * x_ + self.ori.b1 * y_) as f64;
            let r:f64 = (self.ori.b2 * x_ + self.ori.b3 * y_) as f64;
            Util::round_hex(q, r, -q - r)
    }

    // 座標をグリッドへ追加
    pub fn add_point(&mut self, hex:&Hexagon,pt:(f64,f64)){

        let  points = self.hexagon_map.entry(*hex).or_insert(vec![]);
        if points.len() == 0{
            let idxs:Vec<(Hexagon,u8)> = Util::belongs_to_center(hex);
            for it in idxs{
                let idx:Hexagon = *hex + it.0;
                let  elem = self.indexes.entry(idx).or_insert(HashMap::new());
                (*elem).insert(*hex, it.1);
            }
        }
        (*points).push(pt);
    }

    // グリッド内に記憶されている座標を取得
    pub fn get_points(&self, hex:&Hexagon)->Vec<(f64,f64)>{
        if let Some(ref points) = self.hexagon_map.get(hex){
            (*points).clone()
        }else{
            vec![]
        }
    }

    // 座標が記憶されているグリッドを取得
    pub fn get_hexagons(&self,
            origin_index_pos:&Hexagon,
            relative_pos:&Hexagon,
            filter:u8,
            current_index_pos:&Hexagon,
            distance:i32
        ) -> HashMap<i32, Vec<Hexagon>>{

        let dir:Hexagon = *current_index_pos - *origin_index_pos;
        let pattern:HashMap<i8,i8> = Pattern::get(dir.q, dir.r, dir.s);

        let hexagons:Vec<Hexagon> = self.get_list(current_index_pos,filter);
        let center_pos:Hexagon = *current_index_pos + *relative_pos;

        let rslt = Util::measure_distance(&hexagons, &center_pos, &pattern, distance);
        rslt

    }

    // インデックスが指し示すグリッドを取得
    pub fn get_list(&self, idx:&Hexagon,filter:u8)->Vec<Hexagon>{
        let mut rslt:Vec<Hexagon> = Vec::new();
        if let Some(ref map) = self.indexes.get(idx){
            for (h, f) in map.iter() {
                if (filter & *f) != 0 {
                    rslt.push(*h)
                };
            }
        }
        rslt
    }
}
