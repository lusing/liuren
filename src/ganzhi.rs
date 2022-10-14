use std::sync::Arc;
use crate::dizhi::DiZhi;
use crate::tiangan::TianGan;
use crate::xing::{WuXing, Xing};

//#[derive(Clone, Copy, Debug)]
pub struct GanZhi {
    pub gan: u8,
    pub zhi: u8,
}

impl GanZhi {
    pub fn new(self) {}
}

#[derive(Clone, Copy)]
pub enum GZ {
    Tg(TianGan),
    Dz(DiZhi),
}


impl GZ {
    pub fn is_yang(&self) -> bool {
        match self {
            GZ::Tg(t) => { t.tgid % 2 == 0 }
            GZ::Dz(d) => { d.dzid % 2 == 0 }
        }
    }
    pub fn is_yin(&self) -> bool {
        return !self.is_yang();
    }

    // 判断当前干支是否与日干比和
    pub fn is_bi_he(&self, ri_gan: u8) -> bool {
        return if ri_gan % 2 == 0u8 {
            self.is_yang()
        } else {
            self.is_yin()
        };
    }
    pub fn get_name(&self) -> &'static str {
        match self {
            GZ::Tg(t) => { t.get_name() }
            GZ::Dz(d) => { d.get_name() }
        }
    }

    pub fn cmp(a: Box<dyn WuXing>, b: Box<dyn WuXing>) -> bool {
        if let Some(v) = a.get_xing() {
            if let Some(v2) = b.get_xing() {
                let bool1 = v.ke(v2);
                if bool1 {
                    println!("{}克{}", a.get_origin_name(), b.get_origin_name());
                }
                return bool1;
            }
        }
        return false;
    }
    pub fn ke(self, other: GZ) -> bool {
        match (self, other) {
            (GZ::Tg(a), GZ::Tg(b)) => {
                return GZ::cmp(Box::new(a), Box::new(b));
            }
            (GZ::Tg(a2), GZ::Dz(b2)) => {
                return GZ::cmp(Box::new(a2), Box::new(b2));
            }
            (GZ::Dz(a3), GZ::Tg(b3)) => {
                return GZ::cmp(Box::new(a3), Box::new(b3));
            }
            (GZ::Dz(a4), GZ::Dz(b4)) => {
                return GZ::cmp(Box::new(a4), Box::new(b4));
            }
        }
        return false;
    }
}
