#[derive(Clone, Copy, Debug)]
pub struct DiZhi {
    pub dzid: u8,
}

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

impl DiZhi {
    pub fn get_name(self) -> &'static str {
        let dizhi_name: [&str; 12] = ["子", "丑", "寅", "卯", "辰", "巳", "午", "未", "申", "酉", "戌", "亥"];
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

    pub fn sanhe2(he1: u8, he2: u8, he3: u8) -> bool
    {
        return ((he1 + 4) % 12 == he2) && ((he2 + 4) % 12 == he3);
    }

    /*
     * 冲
     */

    pub fn chong2(chong1: u8, chong2: u8) -> bool
    {
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
            _ => None
        };
    }
}
