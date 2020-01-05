//马前失物诀　起课与前法略不同　月上起日　日上起时辰
/*
use crate::public::{
    GongWei, JieChiKou, JieDaAn, JieKongWang, JieLiuLian, JieSuXi, JieXiaoJi, TwelveTime,
};
*/
use public::{
    GongWei, JieChiKou, JieDaAn, JieKongWang, JieLiuLian, JieSuXi, JieXiaoJi, TwelveTime,
};

pub struct ShiWuJue {
    pub(crate) shi_wu: GongWei,
}
//方法 落宫解卦
impl JieDaAn for ShiWuJue {
    fn jie_daan(&self) -> String {
        format!("失物诀-大安: {}", self.shi_wu.da_an)
    }
}
impl JieLiuLian for ShiWuJue {
    fn jie_liulian(&self) -> String {
        format!("失物诀-留连: {}", self.shi_wu.liu_lian)
    }
}
impl JieSuXi for ShiWuJue {
    fn jie_suxi(&self) -> String {
        format!("失物诀-速喜: {}", self.shi_wu.su_xi)
    }
}
impl JieChiKou for ShiWuJue {
    fn jie_chikou(&self) -> String {
        format!("失物诀-赤口: {}", self.shi_wu.chi_kou)
    }
}
impl JieXiaoJi for ShiWuJue {
    fn jie_xiaoji(&self) -> String {
        format!("失物诀-小吉: {}", self.shi_wu.xiao_ji)
    }
}
impl JieKongWang for ShiWuJue {
    fn jie_kongwang(&self) -> String {
        format!("失物诀-空亡: {}", self.shi_wu.kong_wang)
    }
}

pub fn display_shi_wu_jue(j: u32, _san_t: u32) {
    let shi_wu_j = ShiWuJue {
        shi_wu: GongWei {
            da_an: String::from(
                "\n
                 大安辰戌丑未时 
                 易人老者不差迟 
                 子午卯酉时住陈
                 两个顽童定其真
                 寅申巳亥定何身 
                 阳者中男阴女神
                 ",
            ),
            liu_lian: String::from(
                "\n
                流连辰戌丑未推 
                中男两个高短腿 
                子午老人卯酉十 
                寅申巳亥也好推 
                三个少年跨土堆 
                必有一个犯牢罪",
            ),
            su_xi: String::from(
                "\n
                 速喜辰戌丑未看 
                 贼人可知少年汉 
                 逢午子酉女贼犯 
                 其余中男不须换 ",
            ),
            chi_kou: String::from(
                "\n
                 白虎辰戌丑未神 
                 不推就知是女人 
                 寅申巳亥少年身 
                 子午卯酉群贼真 ",
            ),
            xiao_ji: String::from(
                "\n
                 小吉子午卯酉会 
                 必是老人惯偷鬼 
                 其余可为少年推",
            ),
            kong_wang: String::from(
                "\n
                 空亡辰戌丑昧追 
                 也是少年来犯罪 
                 申未酉午女人位 
                 剩下时辰中男对",
            ),
        },
    };

    //依据第三宫落宫解课
    match j {
        1 | 7 | 13 | 19 | 25 | 31 | 37 | 43 => println!("info:{}", shi_wu_j.jie_daan()),
        2 | 8 | 14 | 20 | 26 | 32 | 38 | 44 => println!("info:{}", shi_wu_j.jie_liulian()),
        3 | 9 | 15 | 21 | 27 | 33 | 39 | 45 => println!("info:{}", shi_wu_j.jie_suxi()),
        4 | 10 | 16 | 22 | 28 | 34 | 40 | 46 => println!("info:{}", shi_wu_j.jie_chikou()),
        5 | 11 | 17 | 23 | 29 | 35 | 41 | 47 => println!("info:{}", shi_wu_j.jie_xiaoji()),
        0 | 6 | 12 | 18 | 24 | 30 | 36 | 42 | 48 => println!("{}", shi_wu_j.jie_kongwang()),
        _ => println!("失物诀落宫数字异常无法解析..."),
    }
}

pub(crate) fn ask_swj(j: u32, san_time: u32) {
    let mut _swj = j;
    display_shi_wu_jue(_swj, san_time);
    //println!("输入m查看更多位置　时间信息");
    time_suj(san_time);
    address_swj(_swj);
}

//三宫起课数字对应时辰解卦辞　子时:晚上11~1 丑时:1~3 午时:中午11~1　以此类推
pub fn time_suj(san_t: u32) {
    let twelve_time = TwelveTime {
        yin_shi: String::from("寅卯时辰落坡间"),
        mao_shi: String::from("寅卯时辰落坡间"),
        chen_shi: String::from("辰巳定是亲人见"),
        si_shi: String::from("辰巳定是亲人见"),
        wu_shi: String::from("午未室中必出观"),
        wei_shi: String::from("午未室中必出观"),
        shen_shi: String::from("申酉溜走顺道前"),
        you_shi: String::from("申酉溜走顺道前"),
        xu_shi: String::from("戌亥时辰沟窝连"),
        hai_shi: String::from("戌亥时辰沟窝连"),
        zi_shi: String::from("子丑时辰林中里"),
        chou_shi: String::from("子丑时辰林中里"),
    };
    //对应三宫输入的时辰数字
    if san_t == 3 {
        println!("info:时辰参考-寅时:{}", twelve_time.yin_shi);
    }
    if san_t == 4 {
        println!("info:时辰参考-卯时:{}", twelve_time.mao_shi);
    }
    if san_t == 5 {
        println!("info:时辰参考-辰时:{}", twelve_time.chen_shi);
    }
    if san_t == 6 {
        println!("info:时辰参考-巳时:{}", twelve_time.si_shi);
    }
    if san_t == 7 {
        println!("info:时辰参考-午时:{}", twelve_time.wu_shi);
    }
    if san_t == 8 {
        println!("info:时辰参考-未时:{}", twelve_time.wei_shi);
    }
    if san_t == 9 {
        println!("info:时辰参考-申时:{}", twelve_time.shen_shi);
    }
    if san_t == 10 {
        println!("info:时辰参考-酉时:{}", twelve_time.you_shi);
    }
    if san_t == 11 {
        println!("info:时辰参考-戌时:{}", twelve_time.xu_shi);
    }
    if san_t == 12 {
        println!("info:时辰参考-亥时:{}", twelve_time.hai_shi);
    }
    if san_t == 1 {
        println!("info:时辰参考-子时:{}", twelve_time.xu_shi);
    }
    if san_t == 2 {
        println!("info:时辰参考-丑时:{}", twelve_time.chou_shi);
    }
}

pub fn address_swj(swj: u32) {
    match swj {
        1 | 7 | 13 | 19 | 25 | 31 | 37 | 43 => println!("info:位置参考: 大安东北正东位"),
        2 | 8 | 14 | 20 | 26 | 32 | 38 | 44 => println!("info:位置参考: 流连东南方向会"),
        3 | 9 | 15 | 21 | 27 | 33 | 39 | 45 => println!("info:位置参考: 速喜必是正南回"),
        4 | 10 | 16 | 22 | 28 | 34 | 40 | 46 => println!("info:位置参考: 白虎西南破财回"),
        5 | 11 | 17 | 23 | 29 | 35 | 41 | 47 => println!("info:位置参考: 小吉方向住西北"),
        0 | 6 | 12 | 18 | 24 | 30 | 36 | 42 | 48 => {
            println!("info:位置参考： 空亡正北不返归 不论何论永难回")
        }
        _ => println!("无法返回失物诀当前宫位的位置信息..."),
    }
}
