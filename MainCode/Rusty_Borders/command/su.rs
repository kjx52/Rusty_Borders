// su.rs
// 本文件包含 cd 命令模块。
// ＊此命令仅能在测试中由软件开发者使用。

/*
	模块重要资源列表

	*列表顺序按首字母顺序排列（不包含结构体成员顺序）。

#==============================#
	本模块定义的常量有：
行号	是否公有	名称						类型
----    --------    ----						----
		private		ONLY_MEMBER					&'static str
		private		CORE_KEY_PASSWORD			&'static str

#==============================#
	本模块定义的函数有：
行号	是否公有	名称		参数			返回值
----    --------    ----		----			------
		pub			__su		&'a str			GenResult<(bool, bool, i32)>
								i32
								bool
								bool

*/

// 核心Hash：
// 默认Hash明文均为“Peace_through_Power”
const ONLY_MEMBER: &'static str = "3F4609B2E282B2C3";
const CORE_KEY_PASSWORD: &'static str = "3F4609B2E282B2C3";

use getch_rs::{Getch, Key,};
use std::io::{self, Write,};
use crate::error_message;
use crate::error_type::GenResult;
use crate::misc::hash_get;

pub fn __su<'a>(tmp1: &'a str, user_key: i32, only_member_bool: bool, user_permission: bool) -> GenResult<(bool, bool, i32)>
{
	if user_key != 0
	{
		println!("su：用户不在sudo特权组中，此事件将被报告。");
		return Err(error_message!(5));
	}
	if ! user_permission
	&& &format!("{:X}", hash_get(" RyB: 请输入管理者密码。\n : ", false)) != ONLY_MEMBER
	{
		println!("su：密码错误。");
		return Err(error_message!(5));
	}

	let tmp = if only_member_bool { 0 }
		else if user_permission { 1 }
		else { 9 };
	match tmp1
	{
		"root" => Ok((false, true, tmp)),
		"kali" => Ok((false, false, tmp)),
		""	=> if only_member_bool
				{
					println!("su：你已经拥有最高权限了。\n想开一个子Shell？试试“rbsh”命令吧。");
					return Err(error_message!(7));
				}
				else
				{
					if ! user_permission
					{
						println!("su：权限不足，请提升至root。");
						return Err(error_message!(7));
					}

					print!("\x1b[1;30m Arcub: 请输入顶点密钥。\n : \x1b[1;30m");

					let mut counter_01: usize = 0;
					let mut ac_command: [char; 60] = ['\0'; 60];
					let tmp1: Getch = Getch::new();
					let _ = io::stdout().flush();
					loop
					{
						if counter_01 > 50
						{
							break;
						}
						match tmp1.getch()
						{
							Ok(Key::Char(tmp2)) =>
							{
								if tmp2 == ' '
								|| tmp2 == '\r'
								|| tmp2 == '\t'
								|| tmp2 == 13_u8 as char
								{
									break;
								}
								else
								{
									ac_command[counter_01] = tmp2;
									counter_01 += 1;
									continue;
								}
							},
							_ => continue,
						}
					}

					let ac_command: String = ac_command[.. counter_01]
						.iter()
						.collect::<String>();
					if &format!("{:X}", hash_get(&ac_command, true)) == CORE_KEY_PASSWORD
					{
						println!("\n\t\x1b[1;5;31m欢迎回来，ＯＮＬＹ MEMBER。\x1b[0m");
						Ok((true, true, tmp))
					}
					else
					{
						println!("su：核心序列比对失败。");
						return Err(error_message!(7));
					}
				},
		_	=>
			{
				println!("su：用户未找到。");
				return Err(error_message!(7));
			}
	}
}