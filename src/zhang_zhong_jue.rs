//六壬掌中诀
//询问并获取用户输入是否继续查看掌中诀内容

//use crate::public::{GongWei, JieChiKou, JieDaAn, JieKongWang, JieLiuLian, JieSuXi, JieXiaoJi};
use public::{GongWei, JieChiKou, JieDaAn, JieKongWang, JieLiuLian, JieSuXi, JieXiaoJi};
use std::io;
use std::process::exit;

pub struct ZhangZhongJue {
    zzj: GongWei,
}

pub(crate) fn ask_zzj(z: u32) -> () {
    println!("输入m查看掌中诀对应宫位 + 四十句金口定");
    let mut _number = z;
    let mut ask_input = String::new(); //获取用户输入
    match io::stdin().read_line(&mut ask_input) {
        Ok(_n) => println!("{}", ask_input),
        Err(error) => {
            println!("error: {} 现在退出...", error);
        }
    }
    let m = String::from("m"); //对比字符串
    let lower_ask_input = ask_input.to_lowercase(); //转换输入为小写字母
                                                    //println!("l input ={}", lower_ask_input);

    if lower_ask_input.trim() == m {
        //掌中诀
        info_zzj(_number)
    } else {
        println!("输入错误现在退出...");
        exit(0);
    }
}

//掌中诀实现方法
impl JieDaAn for ZhangZhongJue {
    fn jie_daan(&self) -> String {
        format!("六壬掌中诀: {}", self.zzj.da_an)
    }
}
impl JieLiuLian for ZhangZhongJue {
    fn jie_liulian(&self) -> String {
        format!("六壬掌中诀: {}", self.zzj.liu_lian)
    }
}
impl JieSuXi for ZhangZhongJue {
    fn jie_suxi(&self) -> String {
        format!("六壬掌中诀: {}", self.zzj.su_xi)
    }
}
impl JieChiKou for ZhangZhongJue {
    fn jie_chikou(&self) -> String {
        format!("六壬掌中诀: {}", self.zzj.chi_kou)
    }
}
impl JieXiaoJi for ZhangZhongJue {
    fn jie_xiaoji(&self) -> String {
        format!("六壬掌中诀: {}", self.zzj.xiao_ji)
    }
}
impl JieKongWang for ZhangZhongJue {
    fn jie_kongwang(&self) -> String {
        format!("六壬掌中诀: {}", self.zzj.kong_wang)
    }
}

//掌中诀方法的实现
pub fn info_zzj(number: u32) {
    println!("::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::");
    println!("六壬掌中月日行，时上定宫来分清。十二属相对卦明，马前六星吉凶应。");
    println!("::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::");
    let zhang_zj = ZhangZhongJue {
        zzj: GongWei {
            da_an: String::from(
                "
大安青龙事事昌，贵人求财在坤方。寻人失物去不远，风水宅舍保安康。\n
路马行人身未动，病者离魂主无妨。丢物失盗近向前，正西西南可寻见。\n
商贾安居为吉日，唯有出行一般祥。求访拜贵莫远行，病患再凶也吉祥。\n
久病必是缓中强，近病无碍寿必良。寻人一五七日见，孤身门庭屋内现。\n
口舌官司贵人消，路见虎猴莫相交。求才八分旺中求，不犯破败稳中收。\n
春占防虎夏防猴，秋龙冬虎犯败口。",
            ),
            liu_lian: String::from(
                "
留连玄武卦无阳，静中求才十分量。失物寻人东南向，婚喜占日雪雨降。\n
家运人口得平祥，鼠马占宫才必强。失盗必要急去寻，贼人转手避人群。\n
出行小人破财伤，合伙谋划事不张。此日病者必不祥，不逢二八莫见强。\n
外人求借莫开口，若然开口永无收。商贾安居莫占宫，百日之内盗宅空。\n
卯时占宫莫向东，破财官非必不通。育子生产反呈吉，天道文曲落命奇。\n
春占防兔夏防鸡，秋鼠冬马必惹气。",
            ),
            su_xi: String::from(
                "
速喜朱雀吉行方，求才去南贵人帮。失物速寻未午申，路上逢女细追寻。\n
官司贵人得高强，近病晦气速转吉。旧病也可扔药箱，行人音信转回乡。\n
家宅风水福安良，口舌自消无凶祸。小人是非身边过，交易买卖财路阔。\n
仕途必升官位坐，商贾安居占此宫。百事大吉喜冲冲，婚事占宫龙子通。\n
合伙求谋必成功，唯怕邪财必败空。贵人为女占南东，狗马羊兔有鸡龙。\n
春防辰龙夏防狗，秋牛冬羊破财口。",
            ),
            chi_kou: String::from(
                "
白虎赤口血口伤，是非官讼须紧防。失物东南西南藏，行人路马有惊慌。\n
狐狸鸡犬多作怪，病者无救入西方。须防小人暗诅咒，寻物莫听女人言。\n
申酉两时合局宫，西向大道得论通。寻人谋事犯口舌，远离车马犯骨折。\n
拜贵午未有良策，纵然凶罪也能赦。求药寻医六阳时，落宫占阴寻人迟。\n
切莫小人去会本，合伙经营半路分。路马行程口莫问，不然盗贼财宝屯。\n
春天防蛇夏防猪，秋兔冬鸡必犯法。",
            ),
            xiao_ji: String::from(
                "
小吉滕蛇最吉昌，贵人路上好商量。禄马阴人来报喜，失物必是在坤方。\n
辰巳午未不出门，丑辰寅卯西北奔。路遇童子可追问，此处必定抓贼人。\n
若问行人随时见，走失也能路上现。出行可得满口财，四通八达无有灾。\n
只是不能去借贷，不然必是长久债。合伙经营忌狗马，财路中途必犯法。\n
生育逢此得天成，平安命良福禄增。风水六合财旺生，官府词讼必得成。\n
春天防鼠夏防马，秋兔冬马必犯法。",
            ),
            kong_wang: String::from(
                "
空亡勾陈事不长，鬼怪阴人无主张。求才失盗心内慌，行人路马有灾殃。\n
寻找事物永不见，官法词讼有刑伤。病者遇邪鬼嚣张，久病不吉命不祥。\n
失盗物品贼人收，三六九月还防偷。此时丧人犯人收，妨害六亲妨四周。\n
求才只得两分利，劝君在乡莫远离。四七十月防铁器，不然必犯血伤忌。\n
莫与牛羊去恩报，中途必受铁马拷。押运之年不的逃，马前遇此神难保。\n
春天防牛夏防羊，秋龙冬狗记得牢。",
            ),
        },
    };
    //依据第三宫落宫解课
    match number {
        1 | 7 | 13 | 19 | 25 | 31 | 37 | 43 => println!("info:{}", zhang_zj.jie_daan()),
        2 | 8 | 14 | 20 | 26 | 32 | 38 | 44 => println!("info:{}", zhang_zj.jie_liulian()),
        3 | 9 | 15 | 21 | 27 | 33 | 39 | 45 => println!("info:{}", zhang_zj.jie_suxi()),
        4 | 10 | 16 | 22 | 28 | 34 | 40 | 46 => println!("info:{}", zhang_zj.jie_chikou()),
        5 | 11 | 17 | 23 | 29 | 35 | 41 | 47 => println!("info:{}", zhang_zj.jie_xiaoji()),
        0 | 6 | 12 | 18 | 24 | 30 | 36 | 42 | 48 => println!("info:{}", zhang_zj.jie_kongwang()),
        _ => println!("落宫数字异常无法解卦..."),
    }
}
