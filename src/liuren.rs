use crate::dizhi::DiZhi;

#[derive(Debug)]
pub struct LiuRen {
    pub month: u8,
    pub time: u8,
    pub gan: u8,
    pub zhi: u8,
    pub jiang: u8,
}

impl LiuRen {
    pub fn init_with_jiang(shi: &str, jiang: &str) -> Option<LiuRen> {
        let shi_id = DiZhi::name_to_id(&shi)?;
        let jiang_id = DiZhi::name_to_id(jiang)?;
        println!("{:?}时", shi_id);
        println!("{:?}将", jiang_id);
        return Some(LiuRen {
            month: 0,
            time: shi_id,
            gan: 0,
            zhi: 0,
            jiang: jiang_id,
        });
    }

    /*
     * 格式化地盘和天盘
     */
    pub fn format_pan(dzid: u8) {
        let mut dzs: [DiZhi; 12] = [DiZhi { dzid: 0 }; 12];
        for i in 0u8..12 {
            dzs[i as usize] = DiZhi { dzid: (i + dzid) % 12u8 };
        }
        println!("{}{}{}{}", dzs[5].get_name(), dzs[6].get_name(), dzs[7].get_name(), dzs[8].get_name());
        println!("{}    {}", dzs[4].get_name(), dzs[9].get_name());
        println!("{}    {}", dzs[3].get_name(), dzs[10].get_name());
        println!("{}{}{}{}", dzs[2].get_name(), dzs[1].get_name(), dzs[0].get_name(), dzs[11].get_name());
    }
    pub fn name_to_id(name: String) -> u8 {
        return 0;
    }

    pub fn get_yue_jiang(month: u8) -> u8 {
        return 12 - month;
    }
}