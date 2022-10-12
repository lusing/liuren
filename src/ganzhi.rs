use crate::dizhi::DiZhi;
use crate::tiangan::TianGan;
use crate::xing::Xing;

//#[derive(Clone, Copy, Debug)]
pub struct GanZhi {
    pub gan: u8,
    pub zhi: u8,
}

impl GanZhi {
    pub fn new(self) {}
}

pub enum GZ {
    Tg(TianGan),
    Dz(DiZhi)
}

impl GZ{
    pub fn ke(self,other:GZ) -> bool{
        match (self, other){
            (GZ::Tg(a), GZ::Tg(b)) => {
                if let Some(v) = a.get_xing() {
                    if let Some(v2) = b.get_xing() {
                        println!("{}克{}", a.get_name(), b.get_name());
                        return v.ke(v2);
                    }
                }
            },
            (GZ::Tg(a2), GZ::Dz(b2)) => {
                if let Some(v) = a2.get_xing() {
                    if let Some(v2) = b2.get_xing() {
                        println!("{}克{}", a2.get_name(), b2.get_name());
                        return v.ke(v2);
                    }
                }
            },
            (GZ::Dz(a3), GZ::Tg(b3)) => {
                if let Some(v) = a3.get_xing() {
                    if let Some(v2) = b3.get_xing() {
                        println!("{}克{}", a3.get_name(), b3.get_name());
                        return v.ke(v2);
                    }
                }
            },
            (GZ::Dz(a4), GZ::Dz(b4)) => {
                if let Some(v) = a4.get_xing() {
                    if let Some(v2) = b4.get_xing() {
                        println!("{}克{}", a4.get_name(), b4.get_name());
                        return v.ke(v2);
                    }
                }
            }
        }
        return false;
    }
}
