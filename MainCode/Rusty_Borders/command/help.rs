// help.rs
// 本文件包含 help 命令模块。

/*
	模块重要资源列表

本模块仅定义了公共函数__help。

*/

use crate::cmd_info::*;
use crate::ryb_config::*;

pub fn __help<'a>(tmp1: &'a str) -> String
{
	match tmp1
	{
		"" => "用法：help [目标命令 CMD]...\n详细信息请使用：help help 命令查看。".to_string(),
		CMD_01 => CMD_01_INFO.to_string(),
		CMD_02 => CMD_02_INFO.to_string(),
		CMD_03 => CMD_03_INFO.to_string(),
		CMD_04 => CMD_04_INFO.to_string(),
		CMD_05 => CMD_05_INFO.to_string(),
		CMD_06 => CMD_06_INFO.to_string(),
		CMD_07 => CMD_07_INFO.to_string(),
		CMD_08 => CMD_08_INFO.to_string(),
		CMD_09 => CMD_09_INFO.to_string(),
		CMD_10 => CMD_10_INFO.to_string(),
		CMD_11 => CMD_11_INFO.to_string(),
		CMD_12 => CMD_12_INFO.to_string(),
		CMD_13 => CMD_13_INFO.to_string(),
		CMD_14 => CMD_14_INFO.to_string(),
		_ => "help：命令未找到。".to_string(),
	}
}