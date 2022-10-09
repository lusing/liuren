use crate::DZ;

#[derive(Clone, Copy, Debug)]
pub struct TianGan {
    pub tgid: u8,
}

pub enum TG {
    Jia = 0,
    Yi,
    Bing,
    Ding,
    Wu,
    Ji,
    Geng,
    Xin,
    Ren,
    Gui,
}

impl TianGan {
    pub fn get_name(self) -> &'static str {
        let dizhi_name: [&str; 10] = ["甲", "乙", "丙", "丁", "戊", "己", "庚", "辛", "壬", "癸"];
        return dizhi_name[self.tgid as usize];
    }

    /**
     * 求天干的寄宫：
     *
     * 甲课寅上乙课辰
     * 丙戌在巳不须论
     * 丁己在未庚申位
     * 辛戌壬亥定其真
     * 癸课由来丑上坐
     * 分明不用四正辰
     */
    pub fn get_ji_gong(tgid : u8) -> u8 {
        let ji_gong:[DZ; 10] = [DZ::Yin, DZ::Chen, DZ::Si, DZ::Wei, DZ::Si, DZ::Wei, DZ::Shen, DZ::Xu, DZ::Hai, DZ::Chou];
        return ji_gong[tgid as usize] as u8;
    }

    pub fn name_to_id(name: &str) -> Option<u8> {
        return match name {
            "甲" => Some(0u8),
            "乙" => Some(1u8),
            "丙" => Some(2u8),
            "丁" => Some(3u8),
            "戊" => Some(4u8),
            "己" => Some(5u8),
            "庚" => Some(6u8),
            "辛" => Some(7u8),
            "壬" => Some(8u8),
            "癸" => Some(9u8),
            _ => None,
        };
    }
}
