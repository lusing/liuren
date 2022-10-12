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

#[derive(Clone,Copy)]
pub enum GZ {
    Tg(TianGan),
    Dz(DiZhi)
}


impl GZ{
    pub fn cmp (a: Box<dyn WuXing>, b: Box<dyn WuXing>) -> bool{
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
    pub fn ke(self,other:GZ) -> bool{
        match (self, other){
            (GZ::Tg(a), GZ::Tg(b)) => {
                return GZ::cmp(Box::new(a),Box::new(b));
            },
            (GZ::Tg(a2), GZ::Dz(b2)) => {
                return GZ::cmp(Box::new(a2),Box::new(b2));
            },
            (GZ::Dz(a3), GZ::Tg(b3)) => {
                return GZ::cmp(Box::new(a3),Box::new(b3));
            },
            (GZ::Dz(a4), GZ::Dz(b4)) => {
                return GZ::cmp(Box::new(a4),Box::new(b4));
            }
        }
        return false;
    }
}
