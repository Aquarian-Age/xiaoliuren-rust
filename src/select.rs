//起课选择　数字或者时间　后期添加随机起课
//use crate::qi_ke::{qi_ke_swj, qi_ke_swj_time, value_in_info, value_in_info_time};
use qi_ke::{qi_ke_swj, qi_ke_swj_time, value_in_info, value_in_info_time};
use std::io;
use std::process::exit;

//获取用户输入(三宫数字)
pub fn get_input() -> u32 {
    //获取输入
    let mut input_one = String::new();
    io::stdin().read_line(&mut input_one).expect("输入错误");

    //过滤无效的输入
    let mut _number: u32 = match input_one.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("下一次请输入数字　现在程序退出...");
            exit(1);
        }
    };
    return _number;
}

//根据输入选择相应起课方法
pub fn select_time_number() {
    //选择起课方式
    println!(
        "\
         Number-->按数字起课选择 1\n\
         Time  -->按时间起课选择 2\n"
    );
    //获取用户输入
    let mut input = String::new();
    io::stdin().read_line(&mut input).ok().expect("输入错误...");
    //过滤掉用户的错误输入
    let choise: u32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("输入错误　程序退出...");
            exit(1);
        }
    };

    //按数字起课
    if choise == 1 {
        println!("Number --> 分类卷部分按1 失物诀按2");
        let mut input_number = String::new();
        io::stdin().read_line(&mut input_number).expect("输入错误");
        let mut _n: u32 = match input_number.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("下一次请输入数字　现在程序退出...");
                exit(1);
            }
        };

        //分类卷
        if _n == 1 {
            println!(
                "道家小六壬卜法卷
        测算内容：
        1:占求财 2:占行人 3:占寻人 
        4:占家先 5:占失物 6:占病人 
        7:占走失 8:占访贵 9:占合伙 
        10:占婚姻 11:占埋葬 12:占修方
        13:占送方 14:占预兆 15:占官非 
        16:占风水 17:占虚实 18:占其他"
            );
            println!("分类卷 --> 根据数字选择要测算的内容");

            let mut input = String::new();
            io::stdin().read_line(&mut input).ok().expect("输入错误...");
            //过滤掉用户的错误输入
            let flj_n: u32 = match input.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("输入错误　程序退出...");
                    exit(1);
                }
            };
            value_in_info(flj_n);
        }
        //失物诀
        if _n == 2 {
            println!("失物诀 --> 起课 --> 输入1到9任意三个自然数字");
            qi_ke_swj();
        }
    }
    //按时辰起课
    if choise == 2 {
        println!("Time --> 分类卷部分按1 失物诀按2");
        let mut input_time = String::new();
        io::stdin().read_line(&mut input_time).expect("输入错误");
        let mut _t: u32 = match input_time.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("下一次请输入数字　现在程序退出...");
                exit(1);
            }
        };
        //分类卷
        if _t == 1 {
            println!(
                "道家小六壬卜法卷
        测算内容：
        1:占求财 2:占行人 3:占寻人 
        4:占家先 5:占失物 6:占病人 
        7:占走失 8:占访贵 9:占合伙 
        10:占婚姻 11:占埋葬 12:占修方
        13:占送方 14:占预兆 15:占官非 
        16:占风水 17:占虚实 18:占其他"
            );
            println!("分类卷 --> 根据数字选择要测算的内容");

            let mut input = String::new();
            io::stdin().read_line(&mut input).ok().expect("输入错误...");
            //过滤掉用户的错误输入
            let flj_t: u32 = match input.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("输入错误　程序退出...");
                    exit(1);
                }
            };
            value_in_info_time(flj_t);
        }
        //失物诀
        if _t == 2 {
            qi_ke_swj_time();
        }
    }
}
