use crate::dizhi::DZ;
use crate::liuren::LiuRen;

mod liuren;
mod dizhi;
mod ganzhi;
mod tiangan;
mod xing;

fn main() {
    println!("地盘");
    println!("天盘");
    println!("四课");
    println!("三传");
    
    let dz = dizhi::DiZhi { dzid: 0u8};
    println!("{:?}",dz.get_name());

    let lr = liuren::LiuRen{
        month: 0u8,
        time: 0u8,
        gan: 0u8,
        zhi: 0u8,
        jiang: 12u8
    };
    LiuRen::format_pan(0);

    let lr1 = liuren::LiuRen::init_with_jiang("卯", "亥").unwrap();

    println!("{}",(DZ::Zi as usize));
    println!("{}",((1i8-2i8)%12i8));
}
