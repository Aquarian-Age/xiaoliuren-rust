//小掉桥内容

//use crate::public::{GongWei, JieChiKou, JieDaAn, JieKongWang, JieLiuLian, JieSuXi, JieXiaoJi};
use public::{GongWei, JieChiKou, JieDaAn, JieKongWang, JieLiuLian, JieSuXi, JieXiaoJi};

pub struct XiaoDiaoQiao {
    pub(crate) diao_qiao: GongWei,
}

impl JieDaAn for XiaoDiaoQiao {
    fn jie_daan(&self) -> String {
        format!("小掉桥: {}", self.diao_qiao.da_an)
    }
}
impl JieLiuLian for XiaoDiaoQiao {
    fn jie_liulian(&self) -> String {
        format!("小掉桥: {}", self.diao_qiao.liu_lian)
    }
}
impl JieSuXi for XiaoDiaoQiao {
    fn jie_suxi(&self) -> String {
        format!("小掉桥: {}", self.diao_qiao.su_xi)
    }
}
impl JieChiKou for XiaoDiaoQiao {
    fn jie_chikou(&self) -> String {
        format!("小掉桥: {}", self.diao_qiao.chi_kou)
    }
}
impl JieXiaoJi for XiaoDiaoQiao {
    fn jie_xiaoji(&self) -> String {
        format!("小掉桥: {}", self.diao_qiao.xiao_ji)
    }
}
impl JieKongWang for XiaoDiaoQiao {
    fn jie_kongwang(&self) -> String {
        format!("小掉桥: {}", self.diao_qiao.kong_wang)
    }
}

pub fn diao_qiao(s: u32) {
    let small_drop_bridge = XiaoDiaoQiao {
        diao_qiao: GongWei {
            da_an: String::from(
                "
                 大安牛羊禄马正，
                 贵人遇此百事成。
                 风水动玄为准绳，
                 百日之内福禄增。",
            ),
            liu_lian: String::from(
                "
                 留连鼠猴贵人是，
                 好运自是得天赐。
                 再至凶处也不迟，
                 弯曲之处也待直。",
            ),
            su_xi: String::from(
                "
                 速喜猪鸡吉祥卦，
                 冬日冰雪春融化。
                 灾消难解枯木发，
                 贵人卜马文星跨。",
            ),
            chi_kou: String::from(
                "
                 白虎兔蛇月堂中，
                 占卦逢此喜冲冲。
                 冰寒透骨喜融融，
                 环视九州贵人通。",
            ),
            xiao_ji: String::from(
                "
                 小吉龙鼠数合六，
                 官讼灾祸顺水流。
                 万事不须自营求，
                 贵人盘上定属牛。",
            ),
            kong_wang: String::from(
                "
                 空亡虎狗贵人合，
                 玉带相结喜乐歌。
                 山路再高不相隔，
                 唯怕带孝来相克。",
            ),
        },
    };

    //依据第三宫落宫解课
    match s {
        1 | 7 | 13 | 19 | 25 | 31 | 37 | 43 => println!("info:{}", small_drop_bridge.jie_daan()),
        2 | 8 | 14 | 20 | 26 | 32 | 38 | 44 => println!("info:{}", small_drop_bridge.jie_liulian()),
        3 | 9 | 15 | 21 | 27 | 33 | 39 | 45 => println!("info:{}", small_drop_bridge.jie_suxi()),
        4 | 10 | 16 | 22 | 28 | 34 | 40 | 46 => println!("info:{}", small_drop_bridge.jie_chikou()),
        5 | 11 | 17 | 23 | 29 | 35 | 41 | 47 => println!("info:{}", small_drop_bridge.jie_xiaoji()),
        0 | 6 | 12 | 18 | 24 | 30 | 36 | 42 | 48 => {
            println!("{}", small_drop_bridge.jie_kongwang())
        }
        _ => println!("小掉桥落宫数字异常无法解析..."),
    }
}
