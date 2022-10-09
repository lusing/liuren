#[derive(Clone, Copy, Debug)]
pub struct Xing {
    pub xingid: u8,
}

pub enum XING {
    Mu = 0,
    Huo,
    Tu,
    Jin,
    Shui,
}

impl Xing {
    pub fn get_name(self) -> &'static str {
        let xing_name: [&str; 5] = ["木", "火", "土", "金", "水"];
        return xing_name[self.xingid as usize];
    }

    pub fn sheng2(shenger: u8, shengee: u8) -> bool {
        return (shenger + 1u8) % 5u8 == shengee;
    }

    pub fn ke2(ker: u8, kee: u8) -> bool {
        return (ker + 2u8) % 5u8 == kee;
    }

    pub fn sheng(self, xing: Xing) -> bool {
        return if Xing::sheng2(self.xingid, xing.xingid) {
            println!("{}生{}", self.get_name(), xing.get_name());
            true
        } else {
            false
        };
    }

    pub fn ke(self, xing: Xing) -> bool {
        return if Xing::ke2(self.xingid, xing.xingid) {
            println!("{}克{}", self.get_name(), xing.get_name());
            true
        } else {
            false
        };
    }

    pub fn debug(self) -> &'static str {
        return self.get_name();
    }
}
