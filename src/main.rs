use crate::dizhi::DZ;
use crate::liuren::LiuRen;
use crate::tiangan::TG;

mod dizhi;
mod ganzhi;
mod liuren;
mod tiangan;
mod xing;

fn main() {
    //let dz = dizhi::DiZhi { dzid: 0u8 };
    //println!("{:?}", dz.get_name());

    /*
    let lr = liuren::LiuRen {
        month: 0u8,
        time: 0u8,
        gan: 0u8,
        zhi: 0u8,
        jiang: 12u8,
        tianpan: [0;12],
        kelow: [0;4],
        kehigh: [0;4],
        sanchuan: [0;3],
        kelow2: [None;4],
        kehigh2: [None;4]
    };*/

    //let lr1 = liuren::LiuRen::init_with_jiang("卯", "亥").unwrap();
    //let lr2 = Box::new(lr1);
    //lr2.all();

    //
    if let Some(mut lr0) = LiuRen::init_with_Chinese("卯", "亥", "甲", "子") {
        //lr0.all();
    }

    // 1-1
    if let Some(mut lr1_1) = LiuRen::init_with_Chinese("午", "酉", "甲", "辰") {
        //lr1_1.all();
    }

    // 1-2
    if let Some(mut lr1_2) = LiuRen::init_with_Chinese("巳", "寅", "壬", "戌") {
        //lr1_2.all();
    }

    // 2-1
    if let Some(mut lr2_1) = LiuRen::init_with_Chinese("未", "申", "甲", "戌") {
        lr2_1.all();
    }

    // 2-2
    if let Some(mut lr2_2) = LiuRen::init_with_Chinese("亥", "卯", "辛", "亥") {
        lr2_2.all();
    }

    // 3
    if let Some(mut lr3_1) = LiuRen::init_with_Chinese("丑", "亥", "丁", "卯") {
        lr3_1.all();
    }

    // 4
    if let Some(mut lr4_1) = LiuRen::init(DZ::Yin, DZ::Hai, TG::Jia, DZ::Xu) {
        lr4_1.all();
    }

    if let Some(mut lr4_2) = LiuRen::init(DZ::Shen, DZ::Hai, TG::Geng, DZ::Xu) {
        lr4_2.all();
    }

    // 5
    if let Some(mut lr5_1) = LiuRen::init(DZ::Yin, DZ::Wu, TG::Wu, DZ::Yin) {
        lr5_1.all();
    }

    if let Some(mut lr5_2) = LiuRen::init(DZ::Zi, DZ::Hai, TG::Yi, DZ::Wei) {
        lr5_2.all();
    }


    // 6

    // 7
    if let Some(mut lr7_1) = LiuRen::init(DZ::Si, DZ::Hai, TG::Xin, DZ::Chou) {
        lr7_1.all();
    }

    if let Some(mut lr7_2) = LiuRen::init(DZ::Shen, DZ::Yin, TG::Xin, DZ::Wei) {
        lr7_2.all();
    }

    // 8
    if let Some(mut lr8_1) = LiuRen::init(DZ::Mao, DZ::Chen, TG::Wu, DZ::Wu) {
        lr8_1.all();
    }

    if let Some(mut lr8_2) = LiuRen::init(DZ::Shen, DZ::Hai, TG::Xin, DZ::Chou) {
        lr8_2.all();
    }

    // 9. 八专

    // 9-1
    if let Some(mut lr9_1) = LiuRen::init_with_Chinese("寅", "亥", "甲", "寅") {
        lr9_1.all();
    }

    // 9-2
    if let Some(mut lr9_2) = LiuRen::init_with_Chinese("申", "亥", "己", "未") {
        lr9_2.all();
    }

    // ===================

    // 1. 元首课
    if let Some(mut lr_k_01) = LiuRen::init_with_Chinese("卯", "子", "甲", "子") {
        //lr_k_01.all();
    }

    // 2. 重审课
    if let Some(mut lr_k_02) = LiuRen::init_with_Chinese("巳", "申", "丙", "戌") {
        //lr_k_02.all();
    }

    // 3. 比用课
    if let Some(mut lr_k_03) = LiuRen::init(DZ::Si, DZ::Chen, TG::Ren, DZ::Chen) {
        lr_k_03.all();
    }
}
