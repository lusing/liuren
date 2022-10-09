#[derive(Clone,Copy,Debug)]
pub struct TianGan{
    pub tgid: u8
}

pub enum TG{
    Jia=0,
    Yi,
    Bing,
    Ding,
    Wu,
    Ji,
    Geng,
    Xin,
    Ren,
    Gui
}

impl TianGan{
    pub fn get_name(self) -> &'static str{
        let dizhi_name:[&str;10] = ["甲","乙","丙","丁","戊","己","庚","辛","壬","癸"];
        return dizhi_name[self.tgid as usize];
    }
}