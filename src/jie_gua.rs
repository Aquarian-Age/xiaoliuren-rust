//每个解法是一个结构体

//use crate::public::{GongWei, JieChiKou, JieDaAn, JieKongWang, JieLiuLian, JieSuXi, JieXiaoJi};
use public::{GongWei, JieChiKou, JieDaAn, JieKongWang, JieLiuLian, JieSuXi, JieXiaoJi};
use std::process::exit;

//1:占求财
pub struct ZhanQiuCai {
    qiu_cai: GongWei,
}
//2:占行人
pub struct ZhanXingRen {
    xing_ren: GongWei,
}
//3:占寻人
pub struct ZhanXunRen {
    xun_ren: GongWei,
}
//4:占家先
pub struct ZhanJiaXian {
    jia_xian: GongWei,
}
//5:占失物
pub struct ZhanShiWu {
    shi_wu: GongWei,
}
//6:占病人
pub struct ZhanBingRen {
    bing_ren: GongWei,
}
//7:占走失
pub struct ZhanZouShi {
    zou_shi: GongWei,
}
//8:占访贵
pub struct ZhanFangGui {
    fang_gui: GongWei,
}
//9:占合伙
pub struct ZhanHeHuo {
    he_huo: GongWei,
}
//10:占婚姻
pub struct ZhanHunYin {
    hun_yin: GongWei,
}
//11:占埋葬
pub struct ZhanMaiZang {
    mai_zang: GongWei,
}
//12:占修方
pub struct ZhanXiuFang {
    xiu_fang: GongWei,
}
//13:占送方
pub struct ZhanSongFang {
    song_fang: GongWei,
}
//14:占预兆
pub struct ZhanYuZhao {
    yu_zhao: GongWei,
}
//15:占官非
pub struct ZhanGuanFei {
    guan_fei: GongWei,
}
//16:占风水
pub struct ZhanFengShui {
    feng_shui: GongWei,
}
//17:占虚实
pub struct ZhanXuShi {
    xu_shi: GongWei,
}

//18占其他
pub struct ZhanQiTa {
    qi_ta: GongWei,
}

//依据落宫解卦
//1 占求财
impl JieDaAn for ZhanQiuCai {
    fn jie_daan(&self) -> String {
        format!("占求财-大安:{}", self.qiu_cai.da_an)
    }
}
impl JieLiuLian for ZhanQiuCai {
    fn jie_liulian(&self) -> String {
        format!("占求财-留连:{}", self.qiu_cai.liu_lian)
    }
}
impl JieSuXi for ZhanQiuCai {
    fn jie_suxi(&self) -> String {
        format!("占求财-速喜:{}", self.qiu_cai.su_xi)
    }
}
impl JieChiKou for ZhanQiuCai {
    fn jie_chikou(&self) -> String {
        format!("占求财-赤口:{}", self.qiu_cai.chi_kou)
    }
}
impl JieXiaoJi for ZhanQiuCai {
    fn jie_xiaoji(&self) -> String {
        format!("占求财-小吉:{}", self.qiu_cai.xiao_ji)
    }
}
impl JieKongWang for ZhanQiuCai {
    fn jie_kongwang(&self) -> String {
        format!("占求财-空亡:{}", self.qiu_cai.kong_wang)
    }
}

//2 占行人
impl JieDaAn for ZhanXingRen {
    fn jie_daan(&self) -> String {
        format!("占行人-大安:{}", self.xing_ren.da_an)
    }
}
impl JieLiuLian for ZhanXingRen {
    fn jie_liulian(&self) -> String {
        format!("占行人-留连:{}", self.xing_ren.liu_lian)
    }
}
impl JieSuXi for ZhanXingRen {
    fn jie_suxi(&self) -> String {
        format!("占行人-留连:{}", self.xing_ren.su_xi)
    }
}
impl JieChiKou for ZhanXingRen {
    fn jie_chikou(&self) -> String {
        format!("占行人-留连:{}", self.xing_ren.chi_kou)
    }
}
impl JieXiaoJi for ZhanXingRen {
    fn jie_xiaoji(&self) -> String {
        format!("占行人-留连:{}", self.xing_ren.xiao_ji)
    }
}
impl JieKongWang for ZhanXingRen {
    fn jie_kongwang(&self) -> String {
        format!("占行人-留连:{}", self.xing_ren.kong_wang)
    }
}
//3:占寻人
impl JieDaAn for ZhanXunRen {
    fn jie_daan(&self) -> String {
        format!("占寻人-大安:{}", self.xun_ren.da_an)
    }
}
impl JieLiuLian for ZhanXunRen {
    fn jie_liulian(&self) -> String {
        format!("占寻人-留连:{}", self.xun_ren.liu_lian)
    }
}
impl JieSuXi for ZhanXunRen {
    fn jie_suxi(&self) -> String {
        format!("占寻人-速喜:{}", self.xun_ren.su_xi)
    }
}
impl JieChiKou for ZhanXunRen {
    fn jie_chikou(&self) -> String {
        format!("占寻人-赤口:{}", self.xun_ren.chi_kou)
    }
}
impl JieXiaoJi for ZhanXunRen {
    fn jie_xiaoji(&self) -> String {
        format!("占寻人-小吉:{}", self.xun_ren.xiao_ji)
    }
}
impl JieKongWang for ZhanXunRen {
    fn jie_kongwang(&self) -> String {
        format!("占寻人-空亡:{}", self.xun_ren.kong_wang)
    }
}
//4:占家先
impl JieDaAn for ZhanJiaXian {
    fn jie_daan(&self) -> String {
        format!("占家先-大安:{}", self.jia_xian.da_an)
    }
}
impl JieLiuLian for ZhanJiaXian {
    fn jie_liulian(&self) -> String {
        format!("占家先-留连:{}", self.jia_xian.liu_lian)
    }
}
impl JieSuXi for ZhanJiaXian {
    fn jie_suxi(&self) -> String {
        format!("占家先-速喜:{}", self.jia_xian.su_xi)
    }
}
impl JieChiKou for ZhanJiaXian {
    fn jie_chikou(&self) -> String {
        format!("占家先:-赤口:{}", self.jia_xian.chi_kou)
    }
}
impl JieXiaoJi for ZhanJiaXian {
    fn jie_xiaoji(&self) -> String {
        format!("占家先-小吉:{}", self.jia_xian.xiao_ji)
    }
}
impl JieKongWang for ZhanJiaXian {
    fn jie_kongwang(&self) -> String {
        format!("占家先-空亡:{}", self.jia_xian.kong_wang)
    }
}

//5:占失物
impl JieDaAn for ZhanShiWu {
    fn jie_daan(&self) -> String {
        format!("占失物-大安:{}", self.shi_wu.da_an)
    }
}
impl JieLiuLian for ZhanShiWu {
    fn jie_liulian(&self) -> String {
        format!("占失物-留连:{}", self.shi_wu.liu_lian)
    }
}
impl JieSuXi for ZhanShiWu {
    fn jie_suxi(&self) -> String {
        format!("占失物-速喜:{}", self.shi_wu.su_xi)
    }
}
impl JieChiKou for ZhanShiWu {
    fn jie_chikou(&self) -> String {
        format!("占失物-赤口:{}", self.shi_wu.chi_kou)
    }
}
impl JieXiaoJi for ZhanShiWu {
    fn jie_xiaoji(&self) -> String {
        format!("占失物-小吉:{}", self.shi_wu.xiao_ji)
    }
}
impl JieKongWang for ZhanShiWu {
    fn jie_kongwang(&self) -> String {
        format!("占失物-空亡:{}", self.shi_wu.kong_wang)
    }
}
//6:占病人
impl JieDaAn for ZhanBingRen {
    fn jie_daan(&self) -> String {
        format!("占病人-大安:{}", self.bing_ren.da_an)
    }
}
impl JieLiuLian for ZhanBingRen {
    fn jie_liulian(&self) -> String {
        format!("占病人-留连:{}", self.bing_ren.liu_lian)
    }
}
impl JieSuXi for ZhanBingRen {
    fn jie_suxi(&self) -> String {
        format!("占病人-速喜:{}", self.bing_ren.su_xi)
    }
}
impl JieChiKou for ZhanBingRen {
    fn jie_chikou(&self) -> String {
        format!("占病人-赤口:{}", self.bing_ren.chi_kou)
    }
}
impl JieXiaoJi for ZhanBingRen {
    fn jie_xiaoji(&self) -> String {
        format!("占病人-小吉:{}", self.bing_ren.xiao_ji)
    }
}
impl JieKongWang for ZhanBingRen {
    fn jie_kongwang(&self) -> String {
        format!("占病人-空亡:{}", self.bing_ren.kong_wang)
    }
}
//7:占走失
impl JieDaAn for ZhanZouShi {
    fn jie_daan(&self) -> String {
        format!("占走失人-大安:{}", self.zou_shi.da_an)
    }
}
impl JieLiuLian for ZhanZouShi {
    fn jie_liulian(&self) -> String {
        format!("占走失-留连:{}", self.zou_shi.liu_lian)
    }
}
impl JieSuXi for ZhanZouShi {
    fn jie_suxi(&self) -> String {
        format!("占走失-速喜:{}", self.zou_shi.su_xi)
    }
}
impl JieChiKou for ZhanZouShi {
    fn jie_chikou(&self) -> String {
        format!("占走失-赤口:{}", self.zou_shi.chi_kou)
    }
}
impl JieXiaoJi for ZhanZouShi {
    fn jie_xiaoji(&self) -> String {
        format!("占走失-小吉:{}", self.zou_shi.xiao_ji)
    }
}
impl JieKongWang for ZhanZouShi {
    fn jie_kongwang(&self) -> String {
        format!("占走失-空亡:{}", self.zou_shi.kong_wang)
    }
}
//8:占访贵
impl JieDaAn for ZhanFangGui {
    fn jie_daan(&self) -> String {
        format!("占访贵-大安:{}", self.fang_gui.da_an)
    }
}
impl JieLiuLian for ZhanFangGui {
    fn jie_liulian(&self) -> String {
        format!("占访贵-留连:{}", self.fang_gui.liu_lian)
    }
}
impl JieSuXi for ZhanFangGui {
    fn jie_suxi(&self) -> String {
        format!("占访贵-速喜:{}", self.fang_gui.su_xi)
    }
}
impl JieChiKou for ZhanFangGui {
    fn jie_chikou(&self) -> String {
        format!("占访贵-赤口:{}", self.fang_gui.chi_kou)
    }
}
impl JieXiaoJi for ZhanFangGui {
    fn jie_xiaoji(&self) -> String {
        format!("占访贵-小吉:{}", self.fang_gui.xiao_ji)
    }
}
impl JieKongWang for ZhanFangGui {
    fn jie_kongwang(&self) -> String {
        format!("占访贵-空亡:{}", self.fang_gui.kong_wang)
    }
}
//9:占合伙
impl JieDaAn for ZhanHeHuo {
    fn jie_daan(&self) -> String {
        format!("占合伙-大安:{}", self.he_huo.da_an)
    }
}
impl JieLiuLian for ZhanHeHuo {
    fn jie_liulian(&self) -> String {
        format!("占合伙-留连:{}", self.he_huo.liu_lian)
    }
}
impl JieSuXi for ZhanHeHuo {
    fn jie_suxi(&self) -> String {
        format!("占合伙-速喜:{}", self.he_huo.su_xi)
    }
}
impl JieChiKou for ZhanHeHuo {
    fn jie_chikou(&self) -> String {
        format!("占合伙-赤口:{}", self.he_huo.chi_kou)
    }
}
impl JieXiaoJi for ZhanHeHuo {
    fn jie_xiaoji(&self) -> String {
        format!("占合伙-小吉:{}", self.he_huo.xiao_ji)
    }
}
impl JieKongWang for ZhanHeHuo {
    fn jie_kongwang(&self) -> String {
        format!("占合伙-空亡:{}", self.he_huo.kong_wang)
    }
}
//10:占婚姻
impl JieDaAn for ZhanHunYin {
    fn jie_daan(&self) -> String {
        format!("占婚姻-大安:{}", self.hun_yin.da_an)
    }
}
impl JieLiuLian for ZhanHunYin {
    fn jie_liulian(&self) -> String {
        format!("占婚姻-留连:{}", self.hun_yin.liu_lian)
    }
}
impl JieSuXi for ZhanHunYin {
    fn jie_suxi(&self) -> String {
        format!("占婚姻-速喜:{}", self.hun_yin.su_xi)
    }
}
impl JieChiKou for ZhanHunYin {
    fn jie_chikou(&self) -> String {
        format!("占婚姻-赤口:{}", self.hun_yin.chi_kou)
    }
}
impl JieXiaoJi for ZhanHunYin {
    fn jie_xiaoji(&self) -> String {
        format!("占婚姻-小吉:{}", self.hun_yin.xiao_ji)
    }
}
impl JieKongWang for ZhanHunYin {
    fn jie_kongwang(&self) -> String {
        format!("占婚姻-空亡:{}", self.hun_yin.kong_wang)
    }
}
//11 占埋葬
impl JieDaAn for ZhanMaiZang {
    fn jie_daan(&self) -> String {
        format!("占埋葬-大安:{}", self.mai_zang.da_an)
    }
}
impl JieLiuLian for ZhanMaiZang {
    fn jie_liulian(&self) -> String {
        format!("占埋葬-留连:{}", self.mai_zang.liu_lian)
    }
}
impl JieSuXi for ZhanMaiZang {
    fn jie_suxi(&self) -> String {
        format!("占埋葬-速喜:{}", self.mai_zang.su_xi)
    }
}
impl JieChiKou for ZhanMaiZang {
    fn jie_chikou(&self) -> String {
        format!("占埋葬-赤口:{}", self.mai_zang.chi_kou)
    }
}
impl JieXiaoJi for ZhanMaiZang {
    fn jie_xiaoji(&self) -> String {
        format!("占埋葬-小吉:{}", self.mai_zang.xiao_ji)
    }
}
impl JieKongWang for ZhanMaiZang {
    fn jie_kongwang(&self) -> String {
        format!("占埋葬-空亡:{}", self.mai_zang.kong_wang)
    }
}
//12:占修方
impl JieDaAn for ZhanXiuFang {
    fn jie_daan(&self) -> String {
        format!("占修方-大安:{}", self.xiu_fang.da_an)
    }
}
impl JieLiuLian for ZhanXiuFang {
    fn jie_liulian(&self) -> String {
        format!("占修方-留连:{}", self.xiu_fang.liu_lian)
    }
}
impl JieSuXi for ZhanXiuFang {
    fn jie_suxi(&self) -> String {
        format!("占修方-速喜:{}", self.xiu_fang.su_xi)
    }
}
impl JieChiKou for ZhanXiuFang {
    fn jie_chikou(&self) -> String {
        format!("占修方-赤口:{}", self.xiu_fang.chi_kou)
    }
}
impl JieXiaoJi for ZhanXiuFang {
    fn jie_xiaoji(&self) -> String {
        format!("占修方-小吉:{}", self.xiu_fang.xiao_ji)
    }
}
impl JieKongWang for ZhanXiuFang {
    fn jie_kongwang(&self) -> String {
        format!("占修方-空亡:{}", self.xiu_fang.kong_wang)
    }
}
//13:占送方
impl JieDaAn for ZhanSongFang {
    fn jie_daan(&self) -> String {
        format!("占送方-大安:{}", self.song_fang.da_an)
    }
}
impl JieLiuLian for ZhanSongFang {
    fn jie_liulian(&self) -> String {
        format!("占送方-留连:{}", self.song_fang.liu_lian)
    }
}
impl JieSuXi for ZhanSongFang {
    fn jie_suxi(&self) -> String {
        format!("占送方-速喜:{}", self.song_fang.su_xi)
    }
}
impl JieChiKou for ZhanSongFang {
    fn jie_chikou(&self) -> String {
        format!("占送方-赤口:{}", self.song_fang.chi_kou)
    }
}
impl JieXiaoJi for ZhanSongFang {
    fn jie_xiaoji(&self) -> String {
        format!("占送方-小吉:{}", self.song_fang.xiao_ji)
    }
}
impl JieKongWang for ZhanSongFang {
    fn jie_kongwang(&self) -> String {
        format!("占送方-空亡:{}", self.song_fang.kong_wang)
    }
}
//14:占预兆
impl JieDaAn for ZhanYuZhao {
    fn jie_daan(&self) -> String {
        format!("占预兆-大安:{}", self.yu_zhao.da_an)
    }
}
impl JieLiuLian for ZhanYuZhao {
    fn jie_liulian(&self) -> String {
        format!("占预兆-留连:{}", self.yu_zhao.liu_lian)
    }
}
impl JieSuXi for ZhanYuZhao {
    fn jie_suxi(&self) -> String {
        format!("占预兆-速喜:{}", self.yu_zhao.su_xi)
    }
}
impl JieChiKou for ZhanYuZhao {
    fn jie_chikou(&self) -> String {
        format!("占预兆-赤口:{}", self.yu_zhao.chi_kou)
    }
}
impl JieXiaoJi for ZhanYuZhao {
    fn jie_xiaoji(&self) -> String {
        format!("占预兆-小吉:{}", self.yu_zhao.xiao_ji)
    }
}
impl JieKongWang for ZhanYuZhao {
    fn jie_kongwang(&self) -> String {
        format!("占预兆-空亡:{}", self.yu_zhao.kong_wang)
    }
}
//15:占管非
impl JieDaAn for ZhanGuanFei {
    fn jie_daan(&self) -> String {
        format!("占管非-大安:{}", self.guan_fei.da_an)
    }
}
impl JieLiuLian for ZhanGuanFei {
    fn jie_liulian(&self) -> String {
        format!("占管非-留连:{}", self.guan_fei.liu_lian)
    }
}
impl JieSuXi for ZhanGuanFei {
    fn jie_suxi(&self) -> String {
        format!("占管非-速喜:{}", self.guan_fei.su_xi)
    }
}
impl JieChiKou for ZhanGuanFei {
    fn jie_chikou(&self) -> String {
        format!("占管非-赤口:{}", self.guan_fei.chi_kou)
    }
}
impl JieXiaoJi for ZhanGuanFei {
    fn jie_xiaoji(&self) -> String {
        format!("占管非-小吉:{}", self.guan_fei.xiao_ji)
    }
}
impl JieKongWang for ZhanGuanFei {
    fn jie_kongwang(&self) -> String {
        format!("占管非-空亡:{}", self.guan_fei.kong_wang)
    }
}
//16:占风水
impl JieDaAn for ZhanFengShui {
    fn jie_daan(&self) -> String {
        format!("占风水-大安:{}", self.feng_shui.da_an)
    }
}
impl JieLiuLian for ZhanFengShui {
    fn jie_liulian(&self) -> String {
        format!("占风水-留连:{}", self.feng_shui.liu_lian)
    }
}
impl JieSuXi for ZhanFengShui {
    fn jie_suxi(&self) -> String {
        format!("占风水-速喜:{}", self.feng_shui.su_xi)
    }
}
impl JieChiKou for ZhanFengShui {
    fn jie_chikou(&self) -> String {
        format!("占风水-赤口:{}", self.feng_shui.chi_kou)
    }
}
impl JieXiaoJi for ZhanFengShui {
    fn jie_xiaoji(&self) -> String {
        format!("占风水-小吉:{}", self.feng_shui.xiao_ji)
    }
}
impl JieKongWang for ZhanFengShui {
    fn jie_kongwang(&self) -> String {
        format!("占风水-空亡:{}", self.feng_shui.kong_wang)
    }
}
//17:占虚实
impl JieDaAn for ZhanXuShi {
    fn jie_daan(&self) -> String {
        format!("占虚实-大安:{}", self.xu_shi.da_an)
    }
}
impl JieLiuLian for ZhanXuShi {
    fn jie_liulian(&self) -> String {
        format!("占虚实-留连:{}", self.xu_shi.liu_lian)
    }
}
impl JieSuXi for ZhanXuShi {
    fn jie_suxi(&self) -> String {
        format!("占虚实-速喜:{}", self.xu_shi.su_xi)
    }
}
impl JieChiKou for ZhanXuShi {
    fn jie_chikou(&self) -> String {
        format!("占虚实-赤口:{}", self.xu_shi.chi_kou)
    }
}
impl JieXiaoJi for ZhanXuShi {
    fn jie_xiaoji(&self) -> String {
        format!("占虚实-小吉:{}", self.xu_shi.xiao_ji)
    }
}
impl JieKongWang for ZhanXuShi {
    fn jie_kongwang(&self) -> String {
        format!("占虚实-空亡:{}", self.xu_shi.kong_wang)
    }
}
//18:占其他
impl JieDaAn for ZhanQiTa {
    fn jie_daan(&self) -> String {
        format!("占其他-大安:{}", self.qi_ta.da_an)
    }
}
impl JieLiuLian for ZhanQiTa {
    fn jie_liulian(&self) -> String {
        format!("占其他-留连:{}", self.qi_ta.liu_lian)
    }
}
impl JieSuXi for ZhanQiTa {
    fn jie_suxi(&self) -> String {
        format!("占其他-速喜:{}", self.qi_ta.su_xi)
    }
}
impl JieChiKou for ZhanQiTa {
    fn jie_chikou(&self) -> String {
        format!("占其他-赤口:{}", self.qi_ta.chi_kou)
    }
}
impl JieXiaoJi for ZhanQiTa {
    fn jie_xiaoji(&self) -> String {
        format!("占其他-小吉:{}", self.qi_ta.xiao_ji)
    }
}
impl JieKongWang for ZhanQiTa {
    fn jie_kongwang(&self) -> String {
        format!("占其他-空亡:{}", self.qi_ta.kong_wang)
    }
}

//打印分类断的解卦信息　_n1为选择占卜内容　即: value_in_info(n: u32)
pub fn info_fen_lei_juan(_n1: u32) {
    if _n1 == 1 {
        info_zhan_qiu_cai(_n1); //1:占求财
    }
    if _n1 == 2 {
        info_zhan_xing_ren(_n1); //2:占行人
    }
    if _n1 == 3 {
        info_zhan_xun_ren(_n1); //3:占寻人
    }
    if _n1 == 4 {
        info_zhan_jia_xian(_n1); //4:占家先
    }
    if _n1 == 5 {
        info_zhan_shi_wu(_n1); //5:占失物
    }
    if _n1 == 6 {
        info_zhan_bing_ren(_n1); //6:占病人
    }
    if _n1 == 7 {
        info_zhan_zou_shi(_n1); //7:占走失
    }
    if _n1 == 8 {
        info_zhan_fang_gui(_n1); //8:占访贵
    }
    if _n1 == 9 {
        info_zhan_he_huo(_n1); //9:占合伙
    }
    if _n1 == 10 {
        info_zhan_hun_yin(_n1); //10:占婚姻
    }
    if _n1 == 11 {
        info_zhan_mai_zang(_n1); //11:占埋葬
    }
    if _n1 == 12 {
        info_zhan_xiu_fang(_n1); //4:占修方
    }
    if _n1 == 13 {
        info_zhan_song_fang(_n1); //13:占送方
    }
    if _n1 == 14 {
        info_zhan_yu_zhao(_n1); //14:占预兆
    }
    if _n1 == 15 {
        info_zhan_guan_fei(_n1); //15:占官非
    }
    if _n1 == 16 {
        info_zhan_feng_shui(_n1); //16:占风水
    }
    if _n1 == 17 {
        info_zhan_xu_shi(_n1); //17:占虚实
    }
    if _n1 == 18 {
        info_zhan_qi_ta(_n1); //18:占其他
    }
    if _n1 < 1 || _n1 > 18 {
        println!("解卦已超范围　退出...");
        exit(1);
    }
}
//1:占求财
pub fn info_zhan_qiu_cai(number: u32) {
    let zhanqiucai = ZhanQiuCai {
        qiu_cai: GongWei {
            da_an: String::from("大安来不得"),
            liu_lian: String::from("流连本无收"),
            su_xi: String::from("速喜加倍利"),
            chi_kou: String::from("白虎支才多"),
            xiao_ji: String::from("小吉本有利"),
            kong_wang: String::from("空亡本无收"),
        },
    };
    //依据第三宫落宫解课
    match number {
        1 | 7 | 13 | 19 | 25 | 31 | 37 | 43 => println!("info:{}", zhanqiucai.jie_daan()),
        2 | 8 | 14 | 20 | 26 | 32 | 38 | 44 => println!("info:{}", zhanqiucai.jie_liulian()),
        3 | 9 | 15 | 21 | 27 | 33 | 39 | 45 => println!("info:{}", zhanqiucai.jie_suxi()),
        4 | 10 | 16 | 22 | 28 | 34 | 40 | 46 => println!("info:{}", zhanqiucai.jie_chikou()),
        5 | 11 | 17 | 23 | 29 | 35 | 41 | 47 => println!("info:{}", zhanqiucai.jie_xiaoji()),
        0 | 6 | 12 | 18 | 24 | 30 | 36 | 42 | 48 => println!("info:{}", zhanqiucai.jie_kongwang()),
        _ => println!("落宫数字异常无法解卦..."),
    }
}
//2:占行人 number 为落到第三宫的数字
pub fn info_zhan_xing_ren(number: u32) {
    let zhanxingren = ZhanXingRen {
        xing_ren: GongWei {
            da_an: String::from("大安身未动"),
            liu_lian: String::from("流连人未见"),
            su_xi: String::from("速喜立时见"),
            chi_kou: String::from("白虎在他乡 "),
            xiao_ji: String::from("小吉终须到"),
            kong_wang: String::from("空亡即世亡"),
        },
    };
    //依据第三宫落宫解课
    match number {
        1 | 7 | 13 | 19 | 25 | 31 | 37 | 43 => println!("info:{}", zhanxingren.jie_daan()),
        2 | 8 | 14 | 20 | 26 | 32 | 38 | 44 => println!("info:{}", zhanxingren.jie_liulian()),
        3 | 9 | 15 | 21 | 27 | 33 | 39 | 45 => println!("info:{}", zhanxingren.jie_suxi()),
        4 | 10 | 16 | 22 | 28 | 34 | 40 | 46 => println!("info:{}", zhanxingren.jie_chikou()),
        5 | 11 | 17 | 23 | 29 | 35 | 41 | 47 => println!("info:{}", zhanxingren.jie_xiaoji()),
        0 | 6 | 12 | 18 | 24 | 30 | 36 | 42 | 48 => println!("info:{}", zhanxingren.jie_kongwang()),
        _ => println!("落宫数字异常无法解卦..."),
    }
}
//3:占寻人
pub fn info_zhan_xun_ren(number: u32) {
    let zhanxunren = ZhanXunRen {
        xun_ren: GongWei {
            da_an: String::from("大安人回来"),
            liu_lian: String::from("流连在家中"),
            su_xi: String::from("速喜须相见"),
            chi_kou: String::from("白虎在他乡 "),
            xiao_ji: String::from("小吉在路中"),
            kong_wang: String::from("空亡出远方"),
        },
    };
    //依据第三宫落宫解课
    match number {
        1 | 7 | 13 | 19 | 25 | 31 | 37 | 43 => println!("info:{}", zhanxunren.jie_daan()),
        2 | 8 | 14 | 20 | 26 | 32 | 38 | 44 => println!("info:{}", zhanxunren.jie_liulian()),
        3 | 9 | 15 | 21 | 27 | 33 | 39 | 45 => println!("info:{}", zhanxunren.jie_suxi()),
        4 | 10 | 16 | 22 | 28 | 34 | 40 | 46 => println!("info:{}", zhanxunren.jie_chikou()),
        5 | 11 | 17 | 23 | 29 | 35 | 41 | 47 => println!("info:{}", zhanxunren.jie_xiaoji()),
        0 | 6 | 12 | 18 | 24 | 30 | 36 | 42 | 48 => println!("info:{}", zhanxunren.jie_kongwang()),
        _ => println!("落宫数字异常无法解卦..."),
    }
}
//4:占家先
pub fn info_zhan_jia_xian(number: u32) {
    let zhanjiaxian = ZhanJiaXian {
        jia_xian: GongWei {
            da_an: String::from("大安家先神"),
            liu_lian: String::from("流连孝服衰"),
            su_xi: String::from("速喜男人病"),
            chi_kou: String::from("白虎有官方"),
            xiao_ji: String::from("小吉庙中神"),
            kong_wang: String::from("空亡万事犹"),
        },
    };
    //依据第三宫落宫解课
    match number {
        1 | 7 | 13 | 19 | 25 | 31 | 37 | 43 => println!("info:{}", zhanjiaxian.jie_daan()),
        2 | 8 | 14 | 20 | 26 | 32 | 38 | 44 => println!("info:{}", zhanjiaxian.jie_liulian()),
        3 | 9 | 15 | 21 | 27 | 33 | 39 | 45 => println!("info:{}", zhanjiaxian.jie_suxi()),
        4 | 10 | 16 | 22 | 28 | 34 | 40 | 46 => println!("info:{}", zhanjiaxian.jie_chikou()),
        5 | 11 | 17 | 23 | 29 | 35 | 41 | 47 => println!("info:{}", zhanjiaxian.jie_xiaoji()),
        0 | 6 | 12 | 18 | 24 | 30 | 36 | 42 | 48 => println!("info:{}", zhanjiaxian.jie_kongwang()),
        _ => println!("落宫数字异常无法解卦..."),
    }
}
//5:占失物
pub fn info_zhan_shi_wu(number: u32) {
    let zhanshiwu = ZhanShiWu {
        shi_wu: GongWei {
            da_an: String::from("大安物不见"),
            liu_lian: String::from("流连在眼前"),
            su_xi: String::from("速喜门庭见"),
            chi_kou: String::from("白虎被人收"),
            xiao_ji: String::from("小吉有人报"),
            kong_wang: String::from("空亡不见回"),
        },
    };
    //依据第三宫落宫解课
    match number {
        1 | 7 | 13 | 19 | 25 | 31 | 37 | 43 => println!("info:{}", zhanshiwu.jie_daan()),
        2 | 8 | 14 | 20 | 26 | 32 | 38 | 44 => println!("info:{}", zhanshiwu.jie_liulian()),
        3 | 9 | 15 | 21 | 27 | 33 | 39 | 45 => println!("info:{}", zhanshiwu.jie_suxi()),
        4 | 10 | 16 | 22 | 28 | 34 | 40 | 46 => println!("info:{}", zhanshiwu.jie_chikou()),
        5 | 11 | 17 | 23 | 29 | 35 | 41 | 47 => println!("info:{}", zhanshiwu.jie_xiaoji()),
        0 | 6 | 12 | 18 | 24 | 30 | 36 | 42 | 48 => println!("info:{}", zhanshiwu.jie_kongwang()),
        _ => println!("落宫数字异常无法解卦..."),
    }
}
//6:占病人
pub fn info_zhan_bing_ren(number: u32) {
    let zhanbingren = ZhanBingRen {
        bing_ren: GongWei {
            da_an: String::from("大安不久病离床"),
            liu_lian: String::from("流连不久病难当"),
            su_xi: String::from("速喜男轻女便重"),
            chi_kou: String::from("白虎知时是重衰"),
            xiao_ji: String::from("小吉时中为吉利"),
            kong_wang: String::from("空亡到底是空亡"),
        },
    };
    //依据第三宫落宫解课
    match number {
        1 | 7 | 13 | 19 | 25 | 31 | 37 | 43 => println!("info:{}", zhanbingren.jie_daan()),
        2 | 8 | 14 | 20 | 26 | 32 | 38 | 44 => println!("info:{}", zhanbingren.jie_liulian()),
        3 | 9 | 15 | 21 | 27 | 33 | 39 | 45 => println!("info:{}", zhanbingren.jie_suxi()),
        4 | 10 | 16 | 22 | 28 | 34 | 40 | 46 => println!("info:{}", zhanbingren.jie_chikou()),
        5 | 11 | 17 | 23 | 29 | 35 | 41 | 47 => println!("info:{}", zhanbingren.jie_xiaoji()),
        0 | 6 | 12 | 18 | 24 | 30 | 36 | 42 | 48 => println!("info:{}", zhanbingren.jie_kongwang()),
        _ => println!("落宫数字异常无法解卦..."),
    }
}
//7:占走失
pub fn info_zhan_zou_shi(number: u32) {
    let zhanzoushi = ZhanZouShi {
        zou_shi: GongWei {
            da_an: String::from("大安去远在东方"),
            liu_lian: String::from("流连眼前在北方"),
            su_xi: String::from("速喜之时在南方"),
            chi_kou: String::from("白虎人收在西方"),
            xiao_ji: String::from("小吉在山林东方"),
            kong_wang: String::from("空亡在路旁中央"),
        },
    };
    /*依据第三宫落宫解课  */
    match number {
        1 | 7 | 13 | 19 | 25 | 31 | 37 | 43 => println!("info:{}", zhanzoushi.jie_daan()),
        2 | 8 | 14 | 20 | 26 | 32 | 38 | 44 => println!("info:{}", zhanzoushi.jie_liulian()),
        3 | 9 | 15 | 21 | 27 | 33 | 39 | 45 => println!("info:{}", zhanzoushi.jie_suxi()),
        4 | 10 | 16 | 22 | 28 | 34 | 40 | 46 => println!("info:{}", zhanzoushi.jie_chikou()),
        5 | 11 | 17 | 23 | 29 | 35 | 41 | 47 => println!("info:{}", zhanzoushi.jie_xiaoji()),
        0 | 6 | 12 | 18 | 24 | 30 | 36 | 42 | 48 => println!("info:{}", zhanzoushi.jie_kongwang()),
        _ => println!("落宫数字异常无法解卦..."),
    }
}
//8:占访贵
pub fn info_zhan_fang_gui(number: u32) {
    let zhanfanggui = ZhanFangGui {
        fang_gui: GongWei {
            da_an: String::from("大安西南贵人帮"),
            liu_lian: String::from("流连西北必吉祥"),
            su_xi: String::from("速喜求贵居坎上"),
            chi_kou: String::from("白虎遂意艮北方"),
            xiao_ji: String::from("小吉东南大吉昌"),
            kong_wang: String::from("空亡东西西南当"),
        },
    };
    /*依据第三宫落宫解课
     */
    match number {
        1 | 7 | 13 | 19 | 25 | 31 | 37 | 43 => println!("info:{}", zhanfanggui.jie_daan()),
        2 | 8 | 14 | 20 | 26 | 32 | 38 | 44 => println!("info:{}", zhanfanggui.jie_liulian()),
        3 | 9 | 15 | 21 | 27 | 33 | 39 | 45 => println!("info:{}", zhanfanggui.jie_suxi()),
        4 | 10 | 16 | 22 | 28 | 34 | 40 | 46 => println!("info:{}", zhanfanggui.jie_chikou()),
        5 | 11 | 17 | 23 | 29 | 35 | 41 | 47 => println!("info:{}", zhanfanggui.jie_xiaoji()),
        0 | 6 | 12 | 18 | 24 | 30 | 36 | 42 | 48 => println!("info:{}", zhanfanggui.jie_kongwang()),
        _ => println!("落宫数字异常无法解卦..."),
    }
}
//9:占合伙
pub fn info_zhan_he_huo(number: u32) {
    let zhanhehuo = ZhanHeHuo {
        he_huo: GongWei {
            da_an: String::from("大安小吉喜稳妥"),
            liu_lian: String::from("流连空亡必散伙"),
            su_xi: String::from("速喜短合久有克 "),
            chi_kou: String::from("白虎多忧口角多"),
            xiao_ji: String::from("大安小吉喜稳妥"),
            kong_wang: String::from("流连空亡必散伙"),
        },
    };
    /*依据第三宫落宫解课
     */
    match number {
        1 | 7 | 13 | 19 | 25 | 31 | 37 | 43 => println!("info:{}", zhanhehuo.jie_daan()),
        2 | 8 | 14 | 20 | 26 | 32 | 38 | 44 => println!("info:{}", zhanhehuo.jie_liulian()),
        3 | 9 | 15 | 21 | 27 | 33 | 39 | 45 => println!("info:{}", zhanhehuo.jie_suxi()),
        4 | 10 | 16 | 22 | 28 | 34 | 40 | 46 => println!("info:{}", zhanhehuo.jie_chikou()),
        5 | 11 | 17 | 23 | 29 | 35 | 41 | 47 => println!("info:{}", zhanhehuo.jie_xiaoji()),
        0 | 6 | 12 | 18 | 24 | 30 | 36 | 42 | 48 => println!("info:{}", zhanhehuo.jie_kongwang()),
        _ => println!("落宫数字异常无法解卦..."),
    }
}
//10:占婚姻
pub fn info_zhan_hun_yin(number: u32) {
    let zhanhunyin = ZhanHunYin {
        hun_yin: GongWei {
            da_an: String::from("大安同福喜多扬"),
            liu_lian: String::from("流连聚散两别乡"),
            su_xi: String::from("速喜旺财事百强"),
            chi_kou: String::from("白虎占婚切莫当 是男是女皆主伤"),
            xiao_ji: String::from("小吉六合定吉祥"),
            kong_wang: String::from("卦推空亡必不良 铜盆疑墆半路殃"),
        },
    };
    /* 依据第三宫落宫解课   */
    match number {
        1 | 7 | 13 | 19 | 25 | 31 | 37 | 43 => println!("info:{}", zhanhunyin.jie_daan()),
        2 | 8 | 14 | 20 | 26 | 32 | 38 | 44 => println!("info:{}", zhanhunyin.jie_liulian()),
        3 | 9 | 15 | 21 | 27 | 33 | 39 | 45 => println!("info:{}", zhanhunyin.jie_suxi()),
        4 | 10 | 16 | 22 | 28 | 34 | 40 | 46 => println!("info:{}", zhanhunyin.jie_chikou()),
        5 | 11 | 17 | 23 | 29 | 35 | 41 | 47 => println!("info:{}", zhanhunyin.jie_xiaoji()),
        0 | 6 | 12 | 18 | 24 | 30 | 36 | 42 | 48 => println!("info:{}", zhanhunyin.jie_kongwang()),
        _ => println!("落宫数字异常无法解卦..."),
    }
}
//11:占埋葬
pub fn info_zhan_mai_zang(number: u32) {
    let zhanmaizang = ZhanMaiZang {
        mai_zang: GongWei {
            da_an: String::from("大安小吉喜名堂 后代子孙福禄长"),
            liu_lian: String::from("流连下葬定不祥 百日之内藏祸殃"),
            su_xi: String::from("此课无内容(作者注解)"),
            chi_kou: String::from("白虎大凶叔少亡"),
            xiao_ji: String::from("大安小吉喜名堂 后代子孙福禄长"),
            kong_wang: String::from("空亡不吉换三房"),
        },
    };
    /* 依据第三宫落宫解课   */
    match number {
        1 | 7 | 13 | 19 | 25 | 31 | 37 | 43 => println!("info:{}", zhanmaizang.jie_daan()),
        2 | 8 | 14 | 20 | 26 | 32 | 38 | 44 => println!("info:{}", zhanmaizang.jie_liulian()),
        3 | 9 | 15 | 21 | 27 | 33 | 39 | 45 => println!("info:{}", zhanmaizang.jie_suxi()),
        4 | 10 | 16 | 22 | 28 | 34 | 40 | 46 => println!("info:{}", zhanmaizang.jie_chikou()),
        5 | 11 | 17 | 23 | 29 | 35 | 41 | 47 => println!("info:{}", zhanmaizang.jie_xiaoji()),
        0 | 6 | 12 | 18 | 24 | 30 | 36 | 42 | 48 => println!("info:{}", zhanmaizang.jie_kongwang()),
        _ => println!("落宫数字异常无法解卦..."),
    }
}
//4:占修方
pub fn info_zhan_xiu_fang(number: u32) {
    let zhanxiufang = ZhanXiuFang {
        xiu_fang: GongWei {
            da_an: String::from("大安三八定禄方"),
            liu_lian: String::from("流连一六财福旺"),
            su_xi: String::from("速喜二七把身伤"),
            chi_kou: String::from("白虎四九犯贼光"),
            xiao_ji: String::from("此课无内容(作者注解)"),
            kong_wang: String::from("空亡五零宅不康 修家依次来探方"),
        },
    };
    /* 依据第三宫落宫解课 大安三八定禄方 白虎四九犯贼光 流连一六财福旺 速喜二七把身伤
    空亡五零宅不康 修家依次来探方  */
    match number {
        1 | 7 | 13 | 19 | 25 | 31 | 37 | 43 => println!("info:{}", zhanxiufang.jie_daan()),
        2 | 8 | 14 | 20 | 26 | 32 | 38 | 44 => println!("info:{}", zhanxiufang.jie_liulian()),
        3 | 9 | 15 | 21 | 27 | 33 | 39 | 45 => println!("info:{}", zhanxiufang.jie_suxi()),
        4 | 10 | 16 | 22 | 28 | 34 | 40 | 46 => println!("info:{}", zhanxiufang.jie_chikou()),
        5 | 11 | 17 | 23 | 29 | 35 | 41 | 47 => println!("info:{}", zhanxiufang.jie_xiaoji()),
        0 | 6 | 12 | 18 | 24 | 30 | 36 | 42 | 48 => println!("info:{}", zhanxiufang.jie_kongwang()),
        _ => println!("落宫数字异常无法解卦..."),
    }
}
//13:占送方
pub fn info_zhan_song_fang(number: u32) {
    let zhansongfang = ZhanSongFang {
        song_fang: GongWei {
            da_an: String::from("时辰落宫排五行 须同日上共论清 日上定我何方兴 金木水火土分明"),
            liu_lian: String::from("时辰落宫排五行 须同日上共论清 日上定我何方兴 金木水火土分明"),
            su_xi: String::from("时辰落宫排五行 须同日上共论清 日上定我何方兴 金木水火土分明"),
            chi_kou: String::from("时辰落宫排五行 须同日上共论清 日上定我何方兴 金木水火土分明"),
            xiao_ji: String::from("时辰落宫排五行 须同日上共论清 日上定我何方兴 金木水火土分明"),
            kong_wang: String::from("时辰落宫排五行 须同日上共论清 日上定我何方兴 金木水火土分明"),
        },
    };
    /* 依据第三宫落宫解课 时辰落宫排五行 须同日上共论清 日上定我何方兴 金木水火土分明 */
    match number {
        1 | 7 | 13 | 19 | 25 | 31 | 37 | 43 => println!("info:{}", zhansongfang.jie_daan()),
        2 | 8 | 14 | 20 | 26 | 32 | 38 | 44 => println!("info:{}", zhansongfang.jie_liulian()),
        3 | 9 | 15 | 21 | 27 | 33 | 39 | 45 => println!("info:{}", zhansongfang.jie_suxi()),
        4 | 10 | 16 | 22 | 28 | 34 | 40 | 46 => println!("info:{}", zhansongfang.jie_chikou()),
        5 | 11 | 17 | 23 | 29 | 35 | 41 | 47 => println!("info:{}", zhansongfang.jie_xiaoji()),
        0 | 6 | 12 | 18 | 24 | 30 | 36 | 42 | 48 => {
            println!("info:{}", zhansongfang.jie_kongwang())
        }
        _ => println!("落宫数字异常无法解卦..."),
    }
}
//14:占预兆
pub fn info_zhan_yu_zhao(number: u32) {
    let zhanyuzhao = ZhanYuZhao {
        yu_zhao: GongWei {
            da_an: String::from(
                "
            大安小吉左眼跳 七日之内有喜报
            大安小吉右眼跳 申时钱财犯争吵 
            大安临时心惊跳 四日车前切躲绕",
            ),
            liu_lian: String::from(
                "
            流连白虎左眼跳 午时出行遇断桥
            流连白虎右眼跳 旧友相约吃面肴 
            流连临时心惊跳 五日须防贼来撬",
            ),
            su_xi: String::from(
                "
            速喜空亡左眼跳 三日男人犯口角
            速喜空亡右眼跳 九日口舌跑不了
            速喜临时心惊跳 六日气翻体不消",
            ),
            chi_kou: String::from("白虎临时心惊跳 六亲老人主挂孝"),
            xiao_ji: String::from("小吉临时心惊跳 八日破财有损耗"),
            kong_wang: String::from("空亡临时心惊跳 九日之内坏书到明"),
        },
    };
    /* 依据第三宫落宫解课 */
    match number {
        1 | 7 | 13 | 19 | 25 | 31 | 37 | 43 => println!("info:{}", zhanyuzhao.jie_daan()),
        2 | 8 | 14 | 20 | 26 | 32 | 38 | 44 => println!("info:{}", zhanyuzhao.jie_liulian()),
        3 | 9 | 15 | 21 | 27 | 33 | 39 | 45 => println!("info:{}", zhanyuzhao.jie_suxi()),
        4 | 10 | 16 | 22 | 28 | 34 | 40 | 46 => println!("info:{}", zhanyuzhao.jie_chikou()),
        5 | 11 | 17 | 23 | 29 | 35 | 41 | 47 => println!("info:{}", zhanyuzhao.jie_xiaoji()),
        0 | 6 | 12 | 18 | 24 | 30 | 36 | 42 | 48 => println!("info:{}", zhanyuzhao.jie_kongwang()),
        _ => println!("落宫数字异常无法解卦..."),
    }
}
//15:占官非
pub fn info_zhan_guan_fei(number: u32) {
    let zhanguanfei = ZhanGuanFei {
        guan_fei: GongWei {
            da_an: String::from("大安是不成"),
            liu_lian: String::from("流连实是忧"),
            su_xi: String::from("速喜人和散"),
            chi_kou: String::from("白虎见官方"),
            xiao_ji: String::from("小吉须消散"),
            kong_wang: String::from("空亡即不妨"),
        },
    };
    /* 依据第三宫落宫解课 */
    match number {
        1 | 7 | 13 | 19 | 25 | 31 | 37 | 43 => println!("info:{}", zhanguanfei.jie_daan()),
        2 | 8 | 14 | 20 | 26 | 32 | 38 | 44 => println!("info:{}", zhanguanfei.jie_liulian()),
        3 | 9 | 15 | 21 | 27 | 33 | 39 | 45 => println!("info:{}", zhanguanfei.jie_suxi()),
        4 | 10 | 16 | 22 | 28 | 34 | 40 | 46 => println!("info:{}", zhanguanfei.jie_chikou()),
        5 | 11 | 17 | 23 | 29 | 35 | 41 | 47 => println!("info:{}", zhanguanfei.jie_xiaoji()),
        0 | 6 | 12 | 18 | 24 | 30 | 36 | 42 | 48 => println!("info:{}", zhanguanfei.jie_kongwang()),
        _ => println!("落宫数字异常无法解卦..."),
    }
}
//16:占风水
pub fn info_zhan_feng_shui(number: u32) {
    let zhanfengshui = ZhanFengShui {
        feng_shui: GongWei {
            da_an: String::from(
                "
            时辰隔住长子当
            顺数落宫定四房
            青龙白虎顺排方
            朱雀玄武紧跟上
            何房见煞何房伤
            何房见贵是吉祥
            旋掌再推风水向
            白虎小吉同分讲
            白虎东起逆下量
            小吉东起顺中央",
            ),
            liu_lian: String::from(
                "
            时辰隔住长子当
            顺数落宫定四房
            青龙白虎顺排方
            朱雀玄武紧跟上
            何房见煞何房伤
            何房见贵是吉祥
            旋掌再推风水向
            白虎小吉同分讲
            白虎东起逆下量
            小吉东起顺中央",
            ),
            su_xi: String::from(
                "
            时辰隔住长子当
            顺数落宫定四房
            青龙白虎顺排方
            朱雀玄武紧跟上
            何房见煞何房伤
            何房见贵是吉祥
            旋掌再推风水向
            白虎小吉同分讲
            白虎东起逆下量
            小吉东起顺中央",
            ),
            chi_kou: String::from(
                "
            时辰隔住长子当
            顺数落宫定四房
            青龙白虎顺排方
            朱雀玄武紧跟上
            何房见煞何房伤
            何房见贵是吉祥
            旋掌再推风水向
            白虎小吉同分讲
            白虎东起逆下量
            小吉东起顺中央",
            ),
            xiao_ji: String::from(
                "            
            时辰隔住长子当
            顺数落宫定四房
            青龙白虎顺排方
            朱雀玄武紧跟上
            何房见煞何房伤
            何房见贵是吉祥
            旋掌再推风水向
            白虎小吉同分讲
            白虎东起逆下量
            小吉东起顺中央",
            ),
            kong_wang: String::from(
                "            
            时辰隔住长子当
            顺数落宫定四房
            青龙白虎顺排方
            朱雀玄武紧跟上
            何房见煞何房伤
            何房见贵是吉祥
            旋掌再推风水向
            白虎小吉同分讲
            白虎东起逆下量
            小吉东起顺中央",
            ),
        },
    };
    /* 依据第三宫落宫解课  */
    match number {
        1 | 7 | 13 | 19 | 25 | 31 | 37 | 43 => println!("info:{}", zhanfengshui.jie_daan()),
        2 | 8 | 14 | 20 | 26 | 32 | 38 | 44 => println!("info:{}", zhanfengshui.jie_liulian()),
        3 | 9 | 15 | 21 | 27 | 33 | 39 | 45 => println!("info:{}", zhanfengshui.jie_suxi()),
        4 | 10 | 16 | 22 | 28 | 34 | 40 | 46 => println!("info:{}", zhanfengshui.jie_chikou()),
        5 | 11 | 17 | 23 | 29 | 35 | 41 | 47 => println!("info:{}", zhanfengshui.jie_xiaoji()),
        0 | 6 | 12 | 18 | 24 | 30 | 36 | 42 | 48 => {
            println!("info:{}", zhanfengshui.jie_kongwang())
        }
        _ => println!("落宫数字异常无法解卦..."),
    }
}
//17:占虚实
pub fn info_zhan_xu_shi(number: u32) {
    let zhanxushi = ZhanXuShi {
        xu_shi: GongWei {
            da_an: String::from("速喜大安话语实"),
            liu_lian: String::from("小吉流连言必失"),
            su_xi: String::from("速喜大安话语实"),
            chi_kou: String::from("白虎多谋假意指"),
            xiao_ji: String::from("小吉流连言必失"),
            kong_wang: String::from("万语虚尽空亡时"),
        },
    };
    /* 依据第三宫落宫解课 速喜大安话语实 小吉流连言必失 白虎多谋假意指 万语虚尽空亡时  */
    match number {
        1 | 7 | 13 | 19 | 25 | 31 | 37 | 43 => println!("info:{}", zhanxushi.jie_daan()),
        2 | 8 | 14 | 20 | 26 | 32 | 38 | 44 => println!("info:{}", zhanxushi.jie_liulian()),
        3 | 9 | 15 | 21 | 27 | 33 | 39 | 45 => println!("info:{}", zhanxushi.jie_suxi()),
        4 | 10 | 16 | 22 | 28 | 34 | 40 | 46 => println!("info:{}", zhanxushi.jie_chikou()),
        5 | 11 | 17 | 23 | 29 | 35 | 41 | 47 => println!("info:{}", zhanxushi.jie_xiaoji()),
        0 | 6 | 12 | 18 | 24 | 30 | 36 | 42 | 48 => println!("info:{}", zhanxushi.jie_kongwang()),
        _ => println!("落宫数字异常无法解卦..."),
    }
}
//18:占其他
pub fn info_zhan_qi_ta(number: u32) {
    let zhanqita = ZhanQiTa {
        qi_ta: GongWei {
            da_an: String::from("等待添加..."),
            liu_lian: String::from("等待添加..."),
            su_xi: String::from(" 等待添加..."),
            chi_kou: String::from("等待添加..."),
            xiao_ji: String::from("等待添加..."),
            kong_wang: String::from("等待添加..."),
        },
    };
    /* 依据第三宫落宫解课 速喜大安话语实 小吉流连言必失 白虎多谋假意指 万语虚尽空亡时  */
    match number {
        1 | 7 | 13 | 19 | 25 | 31 | 37 | 43 => println!("info:{}", zhanqita.jie_daan()),
        2 | 8 | 14 | 20 | 26 | 32 | 38 | 44 => println!("info:{}", zhanqita.jie_liulian()),
        3 | 9 | 15 | 21 | 27 | 33 | 39 | 45 => println!("info:{}", zhanqita.jie_suxi()),
        4 | 10 | 16 | 22 | 28 | 34 | 40 | 46 => println!("info:{}", zhanqita.jie_chikou()),
        5 | 11 | 17 | 23 | 29 | 35 | 41 | 47 => println!("info:{}", zhanqita.jie_xiaoji()),
        0 | 6 | 12 | 18 | 24 | 30 | 36 | 42 | 48 => println!("info:{}", zhanqita.jie_kongwang()),
        _ => println!("落宫数字异常无法解卦..."),
    }
}
