use crate::dizhi::DZ;
use crate::liuren::LiuRen;

mod dizhi;
mod ganzhi;
mod liuren;
mod tiangan;
mod xing;

fn main() {
    println!("地盘");
    println!("天盘");
    println!("四课");
    println!("三传");

    let dz = dizhi::DiZhi { dzid: 0u8 };
    println!("{:?}", dz.get_name());

    let lr = liuren::LiuRen {
        month: 0u8,
        time: 0u8,
        gan: 0u8,
        zhi: 0u8,
        jiang: 12u8,
        tianpan: [0;12],
        kelow: [0;4],
        kehigh: [0;4],
        sanchuan: [0;3]
    };

    let lr1 = liuren::LiuRen::init_with_jiang("卯", "亥").unwrap();
    let lr2 = Box::new(lr1);
    lr2.all();

    println!("{}", (DZ::Zi as usize));
    println!("{}", ((1i8 - 2i8) % 12i8));
}
