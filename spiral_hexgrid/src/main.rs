use std::collections::HashMap;
use std::cmp::{max,min};

extern crate spiral_hexgrid as hexgrid;
use hexgrid::hexagon::*;
use hexgrid::hexgrid::*;


extern crate gnuplot;
use gnuplot::*;

/// メイン関数
fn main() {
    //グリッド作成
    let mut grid = HexGrid::new(1.0, Layout::HORIZONTAL);
    // 座標の範囲
    let n:i64 = 100;
    let mut data:Vec<(i64,i64)> = Vec::new();
    let mut  q:i64 = -n;
    // 点群を作成
    while q <= n{
        let mut r:i64 = max(-n, -q - n);
        while r <= min(n, -q + n) {
            data.push((q,r));
            r+=1;
        }
        q+=1;
    }
    // 中心位置をセット
    let origin:Hexagon=Hexagon::new(40,15);
    // グリッドに点群を格納
    for v in data{
        let inp:Hexagon = Hexagon::new(v.0, v.1);
        let hex:Hexagon = origin - inp;
        let pt:(f64, f64) = grid.hex_to_pixel(&hex);
        grid.add_point(&hex,pt);
    }
    // プロット色の設定
    let color_list = vec!["red", "green", "blue", "cyan", "magenta", "orange", "brown" ];

    // 近傍探索
    let result_map:HashMap<i32, Vec<Hexagon>> = grid.neighbors(&origin,100.0);

    // 結果の出力
    let mut fg = Figure::new();
    {
        let  a =  fg.axes2d();
        // 返却リストをループ
        for (dist, idxs) in &result_map {
            let mut x:Vec<f64> = vec![];
            let mut y:Vec<f64> = vec![];
            for idx in idxs{
                // グリッドに格納されている座標を取得
                let pts:Vec<(f64,f64)> = grid.get_points(idx);
                for pt in pts{
                    x.push(pt.0);
                    y.push(pt.1);
                }
            }
            // 距離別にカラーリング
            let clr = color_list[(dist % 7) as usize];
            a.points(&x, &y, &[PointSymbol('+'), Color(clr)] );
        }
    }
    //プロット
    fg.show();
}
