use crate::xing::{WuXing, XING, Xing};

#[derive(Clone, Copy, Debug)]
pub struct DiZhi {
    pub dzid: u8,
}

#[derive(Clone, Copy, Debug)]
pub enum DZ {
    Zi = 0,
    Chou,
    Yin,
    Mao,
    Chen,
    Si,
    Wu,
    Wei,
    Shen,
    You,
    Xu,
    Hai,
}

impl WuXing for DiZhi{
    fn get_xing(&self) -> Option<Xing> {
        return match self.dzid {
            0 => Some(Xing{xingid:XING::Shui as u8}),//子水
            1 => Some(Xing{xingid:XING::Tu as u8}), //丑土
            2 => Some(Xing{xingid:XING::Mu as u8}),//寅木
            3 => Some(Xing{xingid:XING::Mu as u8}),//卯木
            4 => Some(Xing{xingid:XING::Tu as u8}),//辰土
            5 => Some(Xing{xingid:XING::Huo as u8}),//火
            6 => Some(Xing{xingid:XING::Huo as u8}),//火
            7 => Some(Xing{xingid:XING::Tu as u8}),//土
            8 => Some(Xing{xingid:XING::Jin as u8}),//金
            9 => Some(Xing{xingid:XING::Jin as u8}), //金
            10 => Some(Xing{xingid:XING::Tu as u8}), //土
            11 => Some(Xing{xingid:XING::Shui as u8}), //水
            _ => None
        }
    }

    fn get_origin_name(&self) -> &'static str {
        return self.get_name();
    }
}

impl DiZhi {
    pub fn get_name(self) -> &'static str {
        let dizhi_name: [&str; 12] = [
            "子", "丑", "寅", "卯", "辰", "巳", "午", "未", "申", "酉", "戌", "亥",
        ];
        return dizhi_name[self.dzid as usize];
    }

    /*
     * 合
     */
    pub fn he2(dz1: u8, dz2: u8) -> bool {
        return (dz1 + dz2) % 12 == 1;
    }

    /*
     * 三合
     */

    pub fn sanhe2(he1: u8, he2: u8, he3: u8) -> bool {
        return ((he1 + 4) % 12 == he2) && ((he2 + 4) % 12 == he3);
    }

    /*
     * 冲
     */

    pub fn chong2(chong1: u8, chong2: u8) -> bool {
        return (chong1 + 6u8) % 12u8 == chong2;
    }

    pub fn he(self, dz: DiZhi) -> bool {
        let ishe: bool = DiZhi::he2(self.dzid, dz.dzid);
        if ishe {
            println!("{}与{}合", self.get_name(), dz.get_name())
        }
        return ishe;
    }

    pub fn chong(self, dz: DiZhi) -> bool {
        let is_chong: bool = DiZhi::chong2(self.dzid, dz.dzid);
        if is_chong {
            println!("{}{}相冲", self.get_name(), dz.get_name())
        }
        return is_chong;
    }

    pub fn sanhe(self, dz2: DiZhi, dz3: DiZhi) -> bool {
        return DiZhi::sanhe2(self.dzid, dz2.dzid, dz3.dzid);
    }

    pub fn get_month(self) -> u8 {
        return (self.dzid - 1u8) % 12u8;
    }

    pub fn name_to_id(name: &str) -> Option<u8> {
        return match name {
            "子" => Some(0u8),
            "丑" => Some(1u8),
            "寅" => Some(2u8),
            "卯" => Some(3u8),
            "辰" => Some(4u8),
            "巳" => Some(5u8),
            "午" => Some(6u8),
            "未" => Some(7u8),
            "申" => Some(8u8),
            "酉" => Some(9u8),
            "戌" => Some(10u8),
            "亥" => Some(11u8),
            _ => None,
        };
    }
}

