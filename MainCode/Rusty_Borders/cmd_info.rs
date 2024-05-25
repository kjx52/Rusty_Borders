// cmd_info.rs
// 本文件包含Rusty Borders所包含的命令的一些信息。

pub const CMD_01_INFO: &'static str =
r###"cd 0.6.18
Copyright (TM) 2022-2026 Двигатель Ржавчина, All Rights Reserved.
更改当前Shell的工作目录。

用法：cd [目录路径 DIR]...
参数：无。


Written by Jessarin000.
"###;
pub const CMD_02_INFO: &'static str =
r###"ls 0.2.18
Copyright (TM) 2022-2026 Двигатель Ржавчина, All Rights Reserved.
列出目标路径FILE（默认为当前目录）的信息，并按照字母顺序排序。

用法：ls [参数 OPTION]... [文件路径 FILE]...
参数：
目前仅支持短选项。
    -l      以长格式显示文件和目录信息，包括权限、大小、修改时间等。
    -h      以人类可读的方式输出文件大小，如：16KB、24GB
            （前提是有大小，所以须配合-l使用）
    -R      递归遍历目标路径中的所有文件和子目录。
    -d      列出目标路径下的子目录。
    -a      显示隐藏文件，这是默认的。


Written by Jessarin000.
"###;
pub const CMD_03_INFO: &'static str =
r###"cat 0.0.6
Copyright (TM) 2022-2026 Двигатель Ржавчина, All Rights Reserved.
读取并打印目标文件的内容。

用法：cat [文件路径 FILE]...
参数：无。


Written by Jessarin000.
"###;
pub const CMD_04_INFO: &'static str =
r###"clear 0.0.2
Copyright (TM) 2022-2026 Двигатель Ржавчина, All Rights Reserved.
刷新屏幕，这没什么新奇的。

用法：clear ...
参数：无。


Written by Jessarin000.
"###;
pub const CMD_05_INFO: &'static str =
r###"pwd 0.0.2
Copyright (TM) 2022-2026 Двигатель Ржавчина, All Rights Reserved.
返回当前路径。

用法：pwd ...
参数：无。


Written by Jessarin000.
"###;
pub const CMD_06_INFO: &'static str =
r###"whoami 0.1.6
Copyright (TM) 2022-2026 Двигатель Ржавчина, All Rights Reserved.
这个命令用来检查你的身份并打印用户名，但在这个终端格式下，它似乎完全是多余的:)。

用法：whoami ...
参数：无。


Written by Jessarin000.
"###;
pub const CMD_07_INFO: &'static str =
r###"hostname 0.0.0
Copyright (TM) 2022-2026 Двигатель Ржавчина, All Rights Reserved.
返回当前主机名。

用法：hostname ...
参数：无。


Written by Jessarin000.
"###;
pub const CMD_08_INFO: &'static str =
r###"id 0.0.2
Copyright (TM) 2022-2026 Двигатель Ржавчина, All Rights Reserved.
显示当前用户的ID。

用法：id ...
参数：无。


Written by Jessarin000.
"###;
pub const CMD_09_INFO: &'static str =
r###"touch 0.1.3
Copyright (TM) 2022-2026 Двигатель Ржавчина, All Rights Reserved.
可用于更新文件。
若目标路径不存在，将创建该路径；文件存在则更新修改日期。

用法：touch [文件路径 FILE]...
参数：无。


Written by Jessarin000.
"###;
pub const CMD_10_INFO: &'static str =
r###"mkdir 0.1.3
Copyright (TM) 2022-2026 Двигатель Ржавчина, All Rights Reserved.
可用于创建目录。

用法：mkdir [参数 OPTION]... [目录路径 DIR]...
参数：
	-p		递归地创建目录。


Written by Jessarin000.
"###;
pub const CMD_11_INFO: &'static str =
r###"rm 0.1.5
Copyright (TM) 2022-2026 Двигатель Ржавчина, All Rights Reserved.
可用于删除目标文件。
仅可删除由touch、mkdir创建的文件。

用法：rm [参数 OPTION]... [目录路径 DIR]...
参数：
	-r		递归地删除目标路径中的所有文件和子目录。
	-f		忽略大部分报错。


Written by Jessarin000.
"###;
pub const CMD_12_INFO: &'static str =
r###"wipe 0.1.3
Copyright (TM) 2022-2026 Двигатель Ржавчина, All Rights Reserved.
可用于删除环境文件。

*注意，这个命令不受管制。若非在特殊情况下，请
考虑使用rm。

用法：wipe [参数 OPTION]... [目录路径 DIR]...
参数：
	-r		递归地删除目标路径中的所有文件和子目录。
	-f		忽略大部分报错。


Written by Jessarin000.
"###;
pub const CMD_13_INFO: &'static str =
r###"SU 0.1.13
Copyright (TM) 2022-2026 Двигатель Ржавчина, All Rights Reserved.
在用户拥有开发者许可证的情况下，可以使用本命令来提升权限至任意等级。
目前共三等权限：
    ONLY_MEMBER =>  ·任何命令都是可用的；
                    ·不受到Allow_CMD命令列表的管制；
                    ·可查看真实路径；
                    ·可启动OS_Shell。
    root        =>  除su命令外，其他命令都是可用的，但受到Allow_CMD
                    的约束。
    kali        =>  无法使用wipe等可修改原环境文件的命令，受到
                    Allow_CMD的约束。
	
*为保证环境可控，任何权限修改仅能影响回显类命令的操作结果，如whoami、
id、pwd等，而诸如目录变迁、文件读写等会改变环境状态的命令则均不会收
到影响，如cd、touch、ls等。

*请注意su的提权并不是在子Shell中进行的。

本命令应仅在测试阶段使用。

若要禁止su命令，仅需移除License许可文件即可。

用法：su [用户名 USER]...
参数：无。


Written by Jessarin000.
"###;
pub const CMD_14_INFO: &'static str =
r###"help 0.0.2
Copyright (TM) 2022-2026 Двигатель Ржавчина, All Rights Reserved.
可用于查看命令帮助。

用法：help [目标命令 CMD]...
参数：无。


Written by Jessarin000.
"###;