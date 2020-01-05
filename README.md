## xiaoliuren-rust

### 道家小六壬 卜法卷 rust版本

```text
业余爱好　练手项目
分类卷部分的内容已经补充完毕 占风水部分需要另外处理

```
### 已知问题 
```text
水平有限　重复代码过多　有待优化
```

### rust版本

 - Linux
```
nightly-x86_64-unknown-linux-gnu (default)
rustc 1.41.0-nightly (5c5c8eb86 2019-12-07)

```

 - Windows
```
nightly-x86_64-pc-windows-msvc (default)
rustc 1.42.0-nightly (c5840f9d2 2020-01-03)
```

### 编译

Linux平台: rustup show 版本小于1.41.0的需要启用 //注释的部分 同时注释掉相同的部分
如下 如果编译报错交换注释即可
```
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
```

Windows平台: rustup show 版本大于等于1.42.0直接编译即可
