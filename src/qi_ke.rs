//卜法卷可卜算内容列表
/*
use crate::jie_gua::info_fen_lei_juan;
use crate::select::get_input;
use crate::shi_wu_jue::ask_swj;
use crate::xiao_diao_qiao::diao_qiao;
use crate::zhang_zhong_jue::ask_zzj;
*/
use jie_gua::info_fen_lei_juan;
use select::get_input;
use shi_wu_jue::ask_swj;
use xiao_diao_qiao::diao_qiao;
use zhang_zhong_jue::ask_zzj;
use std::process::exit;

//按照数字起课　根据输入选择对应占卜信息
pub fn value_in_info(n: u32) {
    let mut _n1 = n;
    if _n1 == 1 {
        println!("选择{}: 占求才", _n1);
        println!("起课 --> 输入1到9任意三个自然数字");
        qi_ke(n);
    }
    if _n1 == 2 {
        println!("选择{}: 占行人", _n1);
        println!("起课 --> 输入1到9任意三个自然数字");
        qi_ke(n);
    }
    if _n1 == 3 {
        println!("选择{}:占寻人", _n1);
        println!("起课 --> 输入1到9任意三个自然数字");
        qi_ke(n);
    }
    if _n1 == 4 {
        println!("选择{}: 占家先", _n1);
        println!("起课 --> 输入1到9任意三个自然数字");
        qi_ke(n);
    }
    if _n1 == 5 {
        println!("选择{}: 占失物", _n1);
        println!("起课 --> 输入1到9任意三个自然数字");
        qi_ke(n);
    }
    if _n1 == 6 {
        println!("选择{}: 占病人", _n1);
        println!("起课 --> 输入1到9任意三个自然数字");
        qi_ke(n);
    }
    if _n1 == 7 {
        println!("选择{}: 占走失", _n1);
        println!("起课 --> 输入1到9任意三个自然数字");
        qi_ke(n);
    }
    if _n1 == 8 {
        println!("选择{}: 占访贵", _n1);
        println!("起课 --> 输入1到9任意三个自然数字");
        qi_ke(n);
    }
    if _n1 == 9 {
        println!("选择{}: 占合伙", _n1);
        println!("起课 --> 输入1到9任意三个自然数字");
        qi_ke(n);
    }
    if _n1 == 10 {
        println!("选择{}: 占婚姻", _n1);
        println!("起课 --> 输入1到9任意三个自然数字");
        qi_ke(n);
    }
    if _n1 == 11 {
        println!("选择{}: 占埋葬", _n1);
        println!("起课 --> 输入1到9任意三个自然数字");
        qi_ke(n);
    }
    if _n1 == 12 {
        println!("选择{}: 占修方", _n1);
        println!("起课 --> 输入1到9任意三个自然数字");
        qi_ke(n);
    }
    if _n1 == 13 {
        println!("选择{}: 占送方", _n1);
        println!("起课 --> 输入1到9任意三个自然数字");
        qi_ke(n);
    }
    if _n1 == 14 {
        println!("选择{}: 占预兆", _n1);
        println!("起课 --> 输入1到9任意三个自然数字");
        qi_ke(n);
    }
    if _n1 == 15 {
        println!("选择{}: 占官非", _n1);
        println!("起课 --> 输入1到9任意三个自然数字");
        qi_ke(n);
    }
    if _n1 == 16 {
        println!("选择{}: 占风水", _n1);
        println!("起课 --> 输入1到9任意三个自然数字");
        qi_ke(n);
    }
    if _n1 == 17 {
        println!("选择{}: 占虚实", _n1);
        println!("起课 --> 输入1到9任意三个自然数字");
        qi_ke(n);
    }
    if _n1 == 18 {
        println!("选择{}: 占其他", _n1);
        println!("起课 --> 输入1到9任意三个自然数字");
        qi_ke(n);
    }

    if _n1 < 1 || _n1 > 18 {
        println!("问卜已超范围　退出...");
        exit(1);
    }
}

//按照时辰起课 根据输入选择对应占卜信息
pub fn value_in_info_time(n: u32) {
    let mut _n1 = n;
    if _n1 == 1 {
        println!("选择{}: 占求才", _n1);
        println!("起课 --> 输入农历的月　日　时");
        qi_ke_time(n);
    }
    if _n1 == 2 {
        println!("选择{}: 占行人", _n1);
        println!("起课 --> 输入农历的月　日　时");
        qi_ke_time(n);
    }
    if _n1 == 3 {
        println!("选择{}:占寻人", _n1);
        println!("起课 --> 输入农历的月　日　时");
        qi_ke_time(n);
    }
    if _n1 == 4 {
        println!("选择{}: 占家先", _n1);
        println!("起课 --> 输入农历的月　日　时");
        qi_ke_time(n);
    }
    if _n1 == 5 {
        println!("选择{}: 占失物", _n1);
        println!("起课 --> 输入农历的月　日　时");
        qi_ke_time(n);
    }
    if _n1 == 6 {
        println!("选择{}: 占病人", _n1);
        println!("起课 --> 输入农历的月　日　时");
        qi_ke_time(n);
    }
    if _n1 == 7 {
        println!("选择{}: 占走失", _n1);
        println!("起课 --> 输入农历的月　日　时");
        qi_ke_time(n);
    }
    if _n1 == 8 {
        println!("选择{}: 占访贵", _n1);
        println!("起课 --> 输入农历的月　日　时");
        qi_ke_time(n);
    }
    if _n1 == 9 {
        println!("选择{}: 占合伙", _n1);
        println!("起课 --> 输入农历的月　日　时");
        qi_ke_time(n);
    }
    if _n1 == 10 {
        println!("选择{}: 占婚姻", _n1);
        println!("起课 --> 输入农历的月　日　时");
        qi_ke_time(n);
    }
    if _n1 == 11 {
        println!("选择{}: 占埋葬", _n1);
        println!("起课 --> 输入农历的月　日　时");
        qi_ke_time(n);
    }
    if _n1 == 12 {
        println!("选择{}: 占修方", _n1);
        println!("起课 --> 输入农历的月　日　时");
        qi_ke_time(n);
    }
    if _n1 == 13 {
        println!("选择{}: 占送方", _n1);
        println!("起课 --> 输入农历的月　日　时");
        qi_ke_time(n);
    }
    if _n1 == 14 {
        println!("选择{}: 占预兆", _n1);
        println!("起课 --> 输入农历的月　日　时");
        qi_ke_time(n);
    }
    if _n1 == 15 {
        println!("选择{}: 占官非", _n1);
        println!("起课 --> 输入农历的月　日　时");
        qi_ke_time(n);
    }
    if _n1 == 16 {
        println!("选择{}: 占风水", _n1);
        println!("起课 --> 输入农历的月　日　时");
        qi_ke_time(n);
    }
    if _n1 == 17 {
        println!("选择{}: 占虚实", _n1);
        println!("起课 --> 输入农历的月　日　时");
        qi_ke_time(n);
    }
    if _n1 == 18 {
        println!("选择{}: 占其他", _n1);
        println!("起课 --> 输入农历的月　日　时");
        qi_ke_time(n);
    }

    if _n1 < 1 || _n1 > 18 {
        println!("问卜已超范围　退出...");
        exit(1);
    }
}

//按照数字起课-->分类断部分
pub fn qi_ke(select_number: u32) {
    //获取用户起课数字
    let one = get_input();
    let two = get_input();
    let three = get_input();
    if (one > 9 || one < 1) || (two > 9 || two < 1) || (three > 9 || three < 1) {
        println!("输入数字不在起课范围内 退出...");
        exit(1);
    }
    println!("依据以上数字起课为:");

    //根据确定落宫数字来定宫
    //定一宫
    let yigong: u32 = one % 6;
    //println!("数字{}落{}宫", one, yigong);
    match yigong {
        1 => println!("大安"),
        2 => println!("留连"),
        3 => println!("速喜"),
        4 => println!("赤口"),
        5 => println!("小吉"),
        0 => println!("空亡"),
        _ => println!("一宫什么鬼?"),
    }
    //定二 三宫
    //二宫数字大于六的情况
    if two > 6 {
        let ergong: u32 = yigong + two - 7;
        //println!("数字{}落{}宫", two, ergong);
        match ergong {
            1 | 7 | 13 | 19 | 25 | 31 | 37 => println!("大安"),
            2 | 8 | 14 | 20 | 26 | 32 | 38 | 44 => println!("留连"),
            3 | 9 | 15 | 21 | 27 | 33 | 39 | 45 => println!("速喜"),
            4 | 10 | 16 | 22 | 28 | 34 | 40 | 46 => println!("赤口"),
            5 | 11 | 17 | 23 | 29 | 35 | 41 | 47 => println!("小吉"),
            0 | 6 | 12 | 18 | 24 | 30 | 36 | 42 | 48 => println!("空亡"),
            _ => println!("二宫什么鬼?"),
        }
        //如果三宫数字大于６－１　小于６－１例如:(三宫数字 1　９　１)
        if three > 6 {
            let sangong: u32 = ergong + three - 7;
            //println!("数字{}落{}宫", three, sangong);
            match sangong {
                1 | 7 | 13 | 19 | 25 | 31 | 37 | 43 => println!("大安"),
                2 | 8 | 14 | 20 | 26 | 32 | 38 | 44 => println!("留连"),
                3 | 9 | 15 | 21 | 27 | 33 | 39 | 45 => println!("速喜"),
                4 | 10 | 16 | 22 | 28 | 34 | 40 | 46 => println!("赤口"),
                5 | 11 | 17 | 23 | 29 | 35 | 41 | 47 => println!("小吉"),
                0 | 6 | 12 | 18 | 24 | 30 | 36 | 42 | 48 => println!("空亡"),
                _ => println!("三宫什么鬼?"),
            }
            //画一个界限线
            println!("::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::");
            //根据落宫起课

            //分类断
            info_fen_lei_juan(select_number);

            //失物诀
            //ask_swj(sangong, three);

            //小掉桥部分
            diao_qiao(sangong);

            //掌中诀部分
            ask_zzj(sangong);
        } else if three <= 6 {
            let sangong: u32 = ergong + three - 1;
            //println!("数字{}落{}宫", three, sangong);
            match sangong {
                1 | 7 | 13 | 19 | 25 | 31 | 37 | 43 => println!("大安"),
                2 | 8 | 14 | 20 | 26 | 32 | 38 | 44 => println!("留连"),
                3 | 9 | 15 | 21 | 27 | 33 | 39 | 45 => println!("速喜"),
                4 | 10 | 16 | 22 | 28 | 34 | 40 | 46 => println!("赤口"),
                5 | 11 | 17 | 23 | 29 | 35 | 41 | 47 => println!("小吉"),
                0 | 6 | 12 | 18 | 24 | 30 | 36 | 42 | 48 => println!("空亡"),
                _ => println!("三宫什么鬼?"),
            }
            //画一个界限线
            println!("::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::");
            //根据落宫起课

            //分类断
            info_fen_lei_juan(select_number);

            //失物诀
            //ask_swj(sangong, three);

            //小掉桥部分
            diao_qiao(sangong);

            //掌中诀部分
            ask_zzj(sangong);
        }
    //二宫起课数字小于等于６的情况
    } else if two <= 6 {
        let ergong: u32 = yigong + two - 1;
        // println!("数字{}落{}宫", two, ergong);
        match ergong {
            1 | 7 | 13 | 19 | 25 | 31 | 37 | 43 => println!("大安"),
            2 | 8 | 14 | 20 | 26 | 32 | 38 | 44 => println!("留连"),
            3 | 9 | 15 | 21 | 27 | 33 | 39 | 45 => println!("速喜"),
            4 | 10 | 16 | 22 | 28 | 34 | 40 | 46 => println!("赤口"),
            5 | 11 | 17 | 23 | 29 | 35 | 41 | 47 => println!("小吉"),
            0 | 6 | 12 | 18 | 24 | 30 | 36 | 42 | 48 => println!("空亡"),
            _ => println!("二宫什么鬼?"),
        }
        let sangong: u32 = ergong + three - 1;
        //println!("数字{}落{}宫", three, sangong);
        match sangong {
            1 | 7 | 13 | 19 | 25 | 31 | 37 | 43 => println!("大安"),
            2 | 8 | 14 | 20 | 26 | 32 | 38 | 44 => println!("留连"),
            3 | 9 | 15 | 21 | 27 | 33 | 39 | 45 => println!("速喜"),
            4 | 10 | 16 | 22 | 28 | 34 | 40 | 46 => println!("赤口"),
            5 | 11 | 17 | 23 | 29 | 35 | 41 | 47 => println!("小吉"),
            0 | 6 | 12 | 18 | 24 | 30 | 36 | 42 | 48 => println!("空亡"),
            _ => println!("三宫什么鬼?"),
        }
        //画一个界限线
        println!("::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::");
        //根据落宫起课

        //分类断
        info_fen_lei_juan(select_number);

        //失物诀
        //ask_swj(sangong, three);

        //小掉桥部分
        diao_qiao(sangong);

        //掌中诀部分
        ask_zzj(sangong);
    }
}

//按照数字起课-->失物诀
pub fn qi_ke_swj() {
    //获取用户起课数字
    let one = get_input();
    let two = get_input();
    let three = get_input();
    if (one > 9 || one < 1) || (two > 9 || two < 1) || (three > 9 || three < 1) {
        println!("输入数字不在起课范围内 退出...");
        exit(1);
    }
    println!("依据以上数字起课为:");
    //根据确定落宫数字来定宫
    //定一宫
    let yigong: u32 = one % 6;
    //println!("数字{}落{}宫", one, yigong);
    match yigong {
        1 => println!("大安"),
        2 => println!("留连"),
        3 => println!("速喜"),
        4 => println!("赤口"),
        5 => println!("小吉"),
        0 => println!("空亡"),
        _ => println!("一宫什么鬼?"),
    }
    //定二 三宫
    //二宫数字大于六的情况
    if two > 6 {
        let ergong: u32 = yigong + two - 7;
        //println!("数字{}落{}宫", two, ergong);
        match ergong {
            1 | 7 | 13 | 19 | 25 | 31 | 37 => println!("大安"),
            2 | 8 | 14 | 20 | 26 | 32 | 38 | 44 => println!("留连"),
            3 | 9 | 15 | 21 | 27 | 33 | 39 | 45 => println!("速喜"),
            4 | 10 | 16 | 22 | 28 | 34 | 40 | 46 => println!("赤口"),
            5 | 11 | 17 | 23 | 29 | 35 | 41 | 47 => println!("小吉"),
            0 | 6 | 12 | 18 | 24 | 30 | 36 | 42 | 48 => println!("空亡"),
            _ => println!("二宫什么鬼?"),
        }
        //如果三宫数字大于６－１　小于６－１例如:(三宫数字 1　９　１)
        if three > 6 {
            let sangong: u32 = ergong + three - 7;
            //println!("数字{}落{}宫", three, sangong);
            match sangong {
                1 | 7 | 13 | 19 | 25 | 31 | 37 | 43 => println!("大安"),
                2 | 8 | 14 | 20 | 26 | 32 | 38 | 44 => println!("留连"),
                3 | 9 | 15 | 21 | 27 | 33 | 39 | 45 => println!("速喜"),
                4 | 10 | 16 | 22 | 28 | 34 | 40 | 46 => println!("赤口"),
                5 | 11 | 17 | 23 | 29 | 35 | 41 | 47 => println!("小吉"),
                0 | 6 | 12 | 18 | 24 | 30 | 36 | 42 | 48 => println!("空亡"),
                _ => println!("三宫什么鬼?"),
            }
            //画一个界限线
            println!("::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::");
            //根据落宫起课

            //分类断
            //info_fen_lei_juan(select_number);

            //失物诀
            ask_swj(sangong, three);

            //小掉桥部分
            diao_qiao(sangong);

            //掌中诀部分
            ask_zzj(sangong);
        } else if three <= 6 {
            let sangong: u32 = ergong + three - 1;
            //println!("数字{}落{}宫", three, sangong);
            match sangong {
                1 | 7 | 13 | 19 | 25 | 31 | 37 | 43 => println!("大安"),
                2 | 8 | 14 | 20 | 26 | 32 | 38 | 44 => println!("留连"),
                3 | 9 | 15 | 21 | 27 | 33 | 39 | 45 => println!("速喜"),
                4 | 10 | 16 | 22 | 28 | 34 | 40 | 46 => println!("赤口"),
                5 | 11 | 17 | 23 | 29 | 35 | 41 | 47 => println!("小吉"),
                0 | 6 | 12 | 18 | 24 | 30 | 36 | 42 | 48 => println!("空亡"),
                _ => println!("三宫什么鬼?"),
            }
            //画一个界限线
            println!("::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::");
            //根据落宫起课

            //分类断
            //info_fen_lei_juan(select_number);

            //失物诀
            ask_swj(sangong, three);

            //小掉桥部分
            diao_qiao(sangong);

            //掌中诀部分
            ask_zzj(sangong);
        }
    //二宫起课数字小于等于６的情况
    } else if two <= 6 {
        let ergong: u32 = yigong + two - 1;
        // println!("数字{}落{}宫", two, ergong);
        match ergong {
            1 | 7 | 13 | 19 | 25 | 31 | 37 | 43 => println!("大安"),
            2 | 8 | 14 | 20 | 26 | 32 | 38 | 44 => println!("留连"),
            3 | 9 | 15 | 21 | 27 | 33 | 39 | 45 => println!("速喜"),
            4 | 10 | 16 | 22 | 28 | 34 | 40 | 46 => println!("赤口"),
            5 | 11 | 17 | 23 | 29 | 35 | 41 | 47 => println!("小吉"),
            0 | 6 | 12 | 18 | 24 | 30 | 36 | 42 | 48 => println!("空亡"),
            _ => println!("二宫什么鬼?"),
        }
        let sangong: u32 = ergong + three - 1;
        //println!("数字{}落{}宫", three, sangong);
        match sangong {
            1 | 7 | 13 | 19 | 25 | 31 | 37 | 43 => println!("大安"),
            2 | 8 | 14 | 20 | 26 | 32 | 38 | 44 => println!("留连"),
            3 | 9 | 15 | 21 | 27 | 33 | 39 | 45 => println!("速喜"),
            4 | 10 | 16 | 22 | 28 | 34 | 40 | 46 => println!("赤口"),
            5 | 11 | 17 | 23 | 29 | 35 | 41 | 47 => println!("小吉"),
            0 | 6 | 12 | 18 | 24 | 30 | 36 | 42 | 48 => println!("空亡"),
            _ => println!("三宫什么鬼?"),
        }
        //画一个界限线
        println!("::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::");
        //根据落宫起课

        //分类断
        //info_fen_lei_juan(select_number);

        //失物诀
        ask_swj(sangong, three);

        //小掉桥部分
        diao_qiao(sangong);

        //掌中诀部分
        ask_zzj(sangong);
    }
}

//按照时辰起课 --分类卷
pub fn qi_ke_time(select_number: u32) {
    println!("分类断--> 按农历时间起课");
    println!(
        "        1->子时(23:00~1:00)　2->丑时(1:00~3:00) 3->寅时(3:00~5:00) \n
        4->卯时(5:00~7:00) 5->辰时(7:00~9:00) 6->巳时(9:00~11:00)\n
        7->午时(11:00~13:00) 8->未时(13:00~15:00) 9->申时(15:00~17:00) \n
        10->酉时(17:00~19:00) 11->戌时(19:00~21:00) 12亥时(21:00~23:00)"
    );
    println!("起课 --> 输入农历的月　日　时　时辰参考以上数字");
    println!("例: 4月26辰时 输入数字: 4  26  5");
    //获取用户起课数字
    let one = get_input();
    let two = get_input();
    let three = get_input();
    if (one > 30 || one < 1) || (two > 30 || two < 1) || (three > 30 || three < 1) {
        println!("输入的　月　日　时　不在起课范围内 退出...");
        exit(1);
    }
    println!("依据以上数字起课为:");

    //根据确定落宫数字来定宫
    //定一宫
    let yigong: u32 = one % 6;
    //println!("数字{}落{}宫", one, yigong);
    match yigong {
        1 => println!("大安"),
        2 => println!("留连"),
        3 => println!("速喜"),
        4 => println!("赤口"),
        5 => println!("小吉"),
        0 => println!("空亡"),
        _ => println!("一宫什么鬼?"),
    }
    //定二 三宫
    //二宫数字大于六的情况
    if two > 6 {
        let ergong: u32 = yigong + two - 7;
        //println!("数字{}落{}宫", two, ergong);
        match ergong {
            1 | 7 | 13 | 19 | 25 | 31 | 37 => println!("大安"),
            2 | 8 | 14 | 20 | 26 | 32 | 38 | 44 => println!("留连"),
            3 | 9 | 15 | 21 | 27 | 33 | 39 | 45 => println!("速喜"),
            4 | 10 | 16 | 22 | 28 | 34 | 40 | 46 => println!("赤口"),
            5 | 11 | 17 | 23 | 29 | 35 | 41 | 47 => println!("小吉"),
            0 | 6 | 12 | 18 | 24 | 30 | 36 | 42 | 48 => println!("空亡"),
            _ => println!("二宫什么鬼?"),
        }
        //如果三宫数字大于６－１　小于６－１例如:(三宫数字 1　９　１)
        if three > 6 {
            let sangong: u32 = ergong + three - 7;
            //println!("数字{}落{}宫", three, sangong);
            match sangong {
                1 | 7 | 13 | 19 | 25 | 31 | 37 | 43 => println!("大安"),
                2 | 8 | 14 | 20 | 26 | 32 | 38 | 44 => println!("留连"),
                3 | 9 | 15 | 21 | 27 | 33 | 39 | 45 => println!("速喜"),
                4 | 10 | 16 | 22 | 28 | 34 | 40 | 46 => println!("赤口"),
                5 | 11 | 17 | 23 | 29 | 35 | 41 | 47 => println!("小吉"),
                0 | 6 | 12 | 18 | 24 | 30 | 36 | 42 | 48 => println!("空亡"),
                _ => println!("三宫什么鬼?"),
            }
            //画一个界限线
            println!("::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::");
            //根据落宫起课

            //分类断
            info_fen_lei_juan(select_number);

            //失物诀
            //ask_swj(sangong, three);

            //小掉桥部分
            diao_qiao(sangong);

            //掌中诀部分
            ask_zzj(sangong);
        } else if three <= 6 {
            let sangong: u32 = ergong + three - 1;
            //println!("数字{}落{}宫", three, sangong);
            match sangong {
                1 | 7 | 13 | 19 | 25 | 31 | 37 | 43 => println!("大安"),
                2 | 8 | 14 | 20 | 26 | 32 | 38 | 44 => println!("留连"),
                3 | 9 | 15 | 21 | 27 | 33 | 39 | 45 => println!("速喜"),
                4 | 10 | 16 | 22 | 28 | 34 | 40 | 46 => println!("赤口"),
                5 | 11 | 17 | 23 | 29 | 35 | 41 | 47 => println!("小吉"),
                0 | 6 | 12 | 18 | 24 | 30 | 36 | 42 | 48 => println!("空亡"),
                _ => println!("三宫什么鬼?"),
            }
            //画一个界限线
            println!("::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::");
            //根据落宫起课

            //分类断
            info_fen_lei_juan(select_number);

            //失物诀
            //ask_swj(sangong, three);

            //小掉桥部分
            diao_qiao(sangong);

            //掌中诀部分
            ask_zzj(sangong);
        }
    //二宫起课数字小于等于６的情况
    } else if two <= 6 {
        let ergong: u32 = yigong + two - 1;
        // println!("数字{}落{}宫", two, ergong);
        match ergong {
            1 | 7 | 13 | 19 | 25 | 31 | 37 | 43 => println!("大安"),
            2 | 8 | 14 | 20 | 26 | 32 | 38 | 44 => println!("留连"),
            3 | 9 | 15 | 21 | 27 | 33 | 39 | 45 => println!("速喜"),
            4 | 10 | 16 | 22 | 28 | 34 | 40 | 46 => println!("赤口"),
            5 | 11 | 17 | 23 | 29 | 35 | 41 | 47 => println!("小吉"),
            0 | 6 | 12 | 18 | 24 | 30 | 36 | 42 | 48 => println!("空亡"),
            _ => println!("二宫什么鬼?"),
        }
        let sangong: u32 = ergong + three - 1;
        //println!("数字{}落{}宫", three, sangong);
        match sangong {
            1 | 7 | 13 | 19 | 25 | 31 | 37 | 43 => println!("大安"),
            2 | 8 | 14 | 20 | 26 | 32 | 38 | 44 => println!("留连"),
            3 | 9 | 15 | 21 | 27 | 33 | 39 | 45 => println!("速喜"),
            4 | 10 | 16 | 22 | 28 | 34 | 40 | 46 => println!("赤口"),
            5 | 11 | 17 | 23 | 29 | 35 | 41 | 47 => println!("小吉"),
            0 | 6 | 12 | 18 | 24 | 30 | 36 | 42 | 48 => println!("空亡"),
            _ => println!("三宫什么鬼?"),
        }
        //画一个界限线
        println!("::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::");
        //根据落宫起课

        //分类断
        info_fen_lei_juan(select_number);

        //失物诀
        //ask_swj(sangong, three);

        //小掉桥部分
        diao_qiao(sangong);

        //掌中诀部分
        ask_zzj(sangong);
    }
}

//按照时辰起课　失物诀
pub fn qi_ke_swj_time() {
    println!("失物诀--> 按农历时间起课");
    println!(
        "        1->子时(23:00~1:00)　2->丑时(1:00~3:00) 3->寅时(3:00~5:00) \n
        4->卯时(5:00~7:00) 5->辰时(7:00~9:00) 6->巳时(9:00~11:00)\n
        7->午时(11:00~13:00) 8->未时(13:00~15:00) 9->申时(15:00~17:00) \n
        10->酉时(17:00~19:00) 11->戌时(19:00~21:00) 12亥时(21:00~23:00)"
    );
    println!("起课 --> 输入农历的月　日　时　时辰参考以上数字");
    println!("例: 4月26辰时 输入数字: 4  26  5");
    //获取用户起课数字
    let one = get_input();
    let two = get_input();
    let three = get_input();
    if (one > 30 || one < 1) || (two > 30 || two < 1) || (three > 30 || three < 1) {
        println!("输入的　月　日　时　不在起课范围内 退出...");
        exit(1);
    }
    println!("依据以上数字起课为:");
    //根据确定落宫数字来定宫
    //定一宫
    let yigong: u32 = one % 6;
    //println!("数字{}落{}宫", one, yigong);
    match yigong {
        1 => println!("大安"),
        2 => println!("留连"),
        3 => println!("速喜"),
        4 => println!("赤口"),
        5 => println!("小吉"),
        0 => println!("空亡"),
        _ => println!("一宫什么鬼?"),
    }
    //定二 三宫
    //二宫数字大于六的情况
    if two > 6 {
        let ergong: u32 = yigong + two - 7;
        //println!("数字{}落{}宫", two, ergong);
        match ergong {
            1 | 7 | 13 | 19 | 25 | 31 | 37 => println!("大安"),
            2 | 8 | 14 | 20 | 26 | 32 | 38 | 44 => println!("留连"),
            3 | 9 | 15 | 21 | 27 | 33 | 39 | 45 => println!("速喜"),
            4 | 10 | 16 | 22 | 28 | 34 | 40 | 46 => println!("赤口"),
            5 | 11 | 17 | 23 | 29 | 35 | 41 | 47 => println!("小吉"),
            0 | 6 | 12 | 18 | 24 | 30 | 36 | 42 | 48 => println!("空亡"),
            _ => println!("二宫什么鬼?"),
        }
        //如果三宫数字大于６－１　小于６－１例如:(三宫数字 1　９　１)
        if three > 6 {
            let sangong: u32 = ergong + three - 7;
            //println!("数字{}落{}宫", three, sangong);
            match sangong {
                1 | 7 | 13 | 19 | 25 | 31 | 37 | 43 => println!("大安"),
                2 | 8 | 14 | 20 | 26 | 32 | 38 | 44 => println!("留连"),
                3 | 9 | 15 | 21 | 27 | 33 | 39 | 45 => println!("速喜"),
                4 | 10 | 16 | 22 | 28 | 34 | 40 | 46 => println!("赤口"),
                5 | 11 | 17 | 23 | 29 | 35 | 41 | 47 => println!("小吉"),
                0 | 6 | 12 | 18 | 24 | 30 | 36 | 42 | 48 => println!("空亡"),
                _ => println!("三宫什么鬼?"),
            }
            //画一个界限线
            println!("::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::");
            //根据落宫起课

            //分类断
            //info_fen_lei_juan(select_number);

            //失物诀
            ask_swj(sangong, three);

            //小掉桥部分
            diao_qiao(sangong);

            //掌中诀部分
            ask_zzj(sangong);
        } else if three <= 6 {
            let sangong: u32 = ergong + three - 1;
            //println!("数字{}落{}宫", three, sangong);
            match sangong {
                1 | 7 | 13 | 19 | 25 | 31 | 37 | 43 => println!("大安"),
                2 | 8 | 14 | 20 | 26 | 32 | 38 | 44 => println!("留连"),
                3 | 9 | 15 | 21 | 27 | 33 | 39 | 45 => println!("速喜"),
                4 | 10 | 16 | 22 | 28 | 34 | 40 | 46 => println!("赤口"),
                5 | 11 | 17 | 23 | 29 | 35 | 41 | 47 => println!("小吉"),
                0 | 6 | 12 | 18 | 24 | 30 | 36 | 42 | 48 => println!("空亡"),
                _ => println!("三宫什么鬼?"),
            }
            //画一个界限线
            println!("::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::");
            //根据落宫起课

            //分类断
            //info_fen_lei_juan(select_number);

            //失物诀
            ask_swj(sangong, three);

            //小掉桥部分
            diao_qiao(sangong);

            //掌中诀部分
            ask_zzj(sangong);
        }
    //二宫起课数字小于等于６的情况
    } else if two <= 6 {
        let ergong: u32 = yigong + two - 1;
        // println!("数字{}落{}宫", two, ergong);
        match ergong {
            1 | 7 | 13 | 19 | 25 | 31 | 37 | 43 => println!("大安"),
            2 | 8 | 14 | 20 | 26 | 32 | 38 | 44 => println!("留连"),
            3 | 9 | 15 | 21 | 27 | 33 | 39 | 45 => println!("速喜"),
            4 | 10 | 16 | 22 | 28 | 34 | 40 | 46 => println!("赤口"),
            5 | 11 | 17 | 23 | 29 | 35 | 41 | 47 => println!("小吉"),
            0 | 6 | 12 | 18 | 24 | 30 | 36 | 42 | 48 => println!("空亡"),
            _ => println!("二宫什么鬼?"),
        }
        let sangong: u32 = ergong + three - 1;
        //println!("数字{}落{}宫", three, sangong);
        match sangong {
            1 | 7 | 13 | 19 | 25 | 31 | 37 | 43 => println!("大安"),
            2 | 8 | 14 | 20 | 26 | 32 | 38 | 44 => println!("留连"),
            3 | 9 | 15 | 21 | 27 | 33 | 39 | 45 => println!("速喜"),
            4 | 10 | 16 | 22 | 28 | 34 | 40 | 46 => println!("赤口"),
            5 | 11 | 17 | 23 | 29 | 35 | 41 | 47 => println!("小吉"),
            0 | 6 | 12 | 18 | 24 | 30 | 36 | 42 | 48 => println!("空亡"),
            _ => println!("三宫什么鬼?"),
        }
        //画一个界限线
        println!("::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::");
        //
        //根据落宫起课

        //分类断
        //info_fen_lei_juan(select_number);

        //失物诀
        ask_swj(sangong, three);

        //小掉桥部分
        diao_qiao(sangong);

        //掌中诀部分
        ask_zzj(sangong);
    }
}
