use crate::dizhi::DiZhi;
use crate::tiangan::TianGan;

//#[derive(Clone,Copy,Debug)]
pub struct LiuRen {
    pub month: u8,
    pub time: u8,
    pub gan: u8,
    pub zhi: u8,
    pub jiang: u8,
    pub tianpan: [u8; 12],
    pub kelow: [u8; 4],
    pub kehigh: [u8; 4],
    pub sanchuan: [u8; 3],
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
            tianpan: [0;12],
            kelow: [0;4],
            kehigh: [0;4],
            sanchuan: [0;3]
        });
    }

    pub fn init_with_Chinese(shi: &str, jiang: &str, ri_gan: &str, ri_zhi: &str) -> Option<LiuRen> {
        let shi_id = DiZhi::name_to_id(&shi)?;
        let jiang_id = DiZhi::name_to_id(jiang)?;
        let rigan_id = TianGan::name_to_id(ri_gan)?;
        let rizhi_id = DiZhi::name_to_id(ri_zhi)?;
        println!("{:?}时", shi_id);
        println!("{:?}将", jiang_id);
        return Some(LiuRen {
            month: 0,
            time: shi_id,
            gan: rigan_id,
            zhi: rizhi_id,
            jiang: jiang_id,
            tianpan: [0;12],
            kelow: [0;4],
            kehigh: [0;4],
            sanchuan: [0;3]
        });
    }

    /*
     * 格式化地盘和天盘
     */
    pub fn format_pan(dzid: u8) {
        let mut dzs: [DiZhi; 12] = [DiZhi { dzid: 0 }; 12];
        for i in 0u8..12 {
            dzs[i as usize] = DiZhi {
                dzid: (i + dzid) % 12u8,
            };
        }
        println!(
            "{}{}{}{}",
            dzs[5].get_name(),
            dzs[6].get_name(),
            dzs[7].get_name(),
            dzs[8].get_name()
        );
        println!("{}   {}", dzs[4].get_name(), dzs[9].get_name());
        println!("{}   {}", dzs[3].get_name(), dzs[10].get_name());
        println!(
            "{}{}{}{}",
            dzs[2].get_name(),
            dzs[1].get_name(),
            dzs[0].get_name(),
            dzs[11].get_name()
        );
    }

    pub fn all(&mut self){
        self.di_pan();
        self.tian_pan();
        self.si_ke();
    }

    /*
     * 排地盘
     */
    pub fn di_pan(&self) {
        LiuRen::format_pan(0);
    }

    /*
     * 排天盘
     */
    pub fn tian_pan(&mut self) {
        let tian_start: u8 = (12 + self.jiang - self.time) % 12;
        LiuRen::format_pan(tian_start);

        for i in 0u8..12 {
            self.tianpan[i as usize] = (12+tian_start+i) % 12u8;
        }

        println!("{:?}",self.tianpan);
    }

    pub fn si_ke(&mut self){
        self.kelow[0] = self.gan;
        self.kehigh[0] = self.tianpan[TianGan::get_ji_gong(self.gan) as usize];
        self.kelow[1] = self.kehigh[0];
        self.kehigh[1] = self.tianpan[self.kelow[1] as usize];

        self.kelow[2] = self.zhi;
        self.kehigh[2] = self.tianpan[self.kelow[2] as usize];
        self.kelow[3] = self.kehigh[2];
        self.kehigh[3] = self.tianpan[self.kelow[3] as usize];

        let kehigh0 = DiZhi{dzid:self.kehigh[0]};
        let kehigh1 = DiZhi{dzid:self.kehigh[1]};
        let kehigh2 = DiZhi{dzid:self.kehigh[2]};
        let kehigh3 = DiZhi{dzid:self.kehigh[3]};

        let kelow0 = TianGan{tgid:self.kelow[0]};
        let kelow1 = DiZhi{dzid:self.kelow[1]};
        let kelow2 = DiZhi{dzid:self.kelow[2]};
        let kelow3 = DiZhi{dzid:self.kelow[3]};

        println!("{:?}",self.tianpan);
        println!("{:?}",self.kehigh);
        println!("{:?}",self.kelow);

        println!("四课：");
        println!("{}{}{}{}",kehigh3.get_name(),kehigh2.get_name(),kehigh1.get_name(),kehigh0.get_name());
        println!("{}{}{}{}",kelow3.get_name(),kelow2.get_name(),kelow1.get_name(),kelow0.get_name());
    }

    pub fn get_yue_jiang(month: u8) -> u8 {
        return 12 - month;
    }

    pub fn get_month_by_yue_jiang(yuejiang: u8) -> u8 {
        return 12 - yuejiang;
    }
}
