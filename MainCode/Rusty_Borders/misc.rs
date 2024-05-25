// misc.rs
// 杂项函数

/*
	模块重要资源列表

	*列表顺序按首字母顺序排列（不包含结构体成员顺序）。

#==============================#
	本模块定义的常量有：
行号	是否公有	名称						类型
----    --------    ----						----
		private		FORBIDDEN_STRS				&'static str
		private		PERMITTED_STRS				&'static str

#==============================#
	本模块定义的特征Trait有：
行号	名称					加入特性			包含的函数
----    ----					--------			----------
		CharExtraTrait01		self				unprint
													isprint
													is_letter
		char					CharExtraTrait01
		CharExtraTrait02		self				new
		[char; 28]				CharExtraTrait02
		[char; 67]				CharExtraTrait02

#==============================#
	本模块定义的函数有：
行号	是否公有	名称							参数						返回值
----    --------    ----							----						------
		pub			compare_char					&char, &char				bool
		pub			del_index_path					&mut Vec<PathBuf>			usize
													usize
													&PathBuf
		pub			hash_get						&'a str, bool				u64
		pub			match_path						&Vec<PathBuf>, &PathBuf		bool
		pub			mid_to_string					&'a str, usize				String
		pub			panic_now						i32							!
		pub			parse_pair						Vec<String>					Vec<String>
													&str
													char
		pub			str_to_vec_char					&'a str						Vec<char>
		pub			write_a							&'a str						Result<File, std::io::Error>
		pub			detect_forbidden_characters		&char						bool
		pub			detect_empty_characters			&char						bool

#==============================#
	本模块定义的宏有：
行号	是否公有	名称
----    --------    ----
		export		match_path_with_root_path

*/

const FORBIDDEN_STRS: &'static str = r####"!"#$%&'()*+,;<=>?@[\]^_`{|}~"####;
const PERMITTED_STRS: &'static str = r####" -./0123456789:ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz"####;

use std::fs::{File, OpenOptions,};
use std::hash::DefaultHasher;
use std::hash::Hasher;
use std::io::{self, Write,};
use std::path::PathBuf;
use super::mark_file;
use super::ryb_config::SYSTEM_INFO;

pub fn panic_now(tmp: i32) -> !
{
	mark_file!("SYSTEM_INFO", SYSTEM_INFO, &format!("【崩溃】 Rusty Borders 伪终端诧异：{tmp}。"), '5');
	panic!("\n\n\x1b[;5;31m
		######################################
		#              %%警告%%              #
		#        %%RyB 核心进程受阻%%        #
		#          %%！紧急终止！%%          #
		#          Critical Code 0{}          #
		######################################
	\x1b[0m", tmp);
}

pub fn write_a<'a>(tmp: &'a str) -> Result<File, std::io::Error>
{
	OpenOptions::new()
		.append(true)
		.create(true)
		.open(tmp)
}

pub fn mid_to_string<'a>(tmp1: &'a str, tmp2: usize) -> String
{
	if tmp1.len() > tmp2
	{
		&tmp1[tmp2..]
	}
	else
	{
		""
	}.to_string()
}

pub fn parse_pair(mut tmp1: Vec<String>, tmp2: &str, separator: char) -> Vec<String>
{
	match tmp2.find(separator)
	{
		None =>
		{
			tmp1.push(tmp2.to_string());
			tmp1
		}
		Some(index) =>
		{
			tmp1.push(tmp2[..index].to_string());
			parse_pair(tmp1, &tmp2[index + 1..], separator)
		}
	}
}

pub fn hash_get<'a>(tmp: &'a str, tmp1: bool) -> u64
{
	let mut line: String = String::new();
	let mut hasher = DefaultHasher::new();

	if tmp1
	{
		line = tmp.to_string();
	}
	else
	{
		print!("\x1b[1;34m{tmp}\x1b[0m");
		let _ = io::stdout().flush();
		let _ = io::stdin().read_line(&mut line);
	}

	hasher.write(line.trim().as_bytes());
	hasher.finish()
}

pub fn str_to_vec_char<'a>(tmp1: &'a str) -> Vec<char>
{
	tmp1.chars().collect::<Vec<char>>()
}

pub fn compare_char(tmp1: &char, tmp2: &char) -> bool
{
	if tmp1        == tmp2
	|| *tmp1 as u8 == *tmp2 as u8 - 32
	{
		true
	}
	else
	{
		false
	}
}

pub fn del_index_path(tmp1: &mut Vec<PathBuf>, mut tmp2: usize, tmp3: &PathBuf) -> usize
{
	if tmp2 == 0
	{
		return 0;
	}
	for tmp in 0..tmp2
	{
		if tmp1[tmp].starts_with(tmp3)
		{
			tmp1.remove(tmp);
			tmp2 -= 1;
			break;
		}
	}

	tmp2
}

pub fn match_path(dirs_crate_list: &Vec<PathBuf>, tmp_path_01: &PathBuf) -> bool
{
	let mut key: bool = true;
	for tmp in dirs_crate_list
	{
		if tmp_path_01.starts_with(tmp)
		{
			key = false;
			break;
		}
	}

	key
}

pub trait CharExtraTrait01
{
    fn unprint(&self) -> bool;
	fn isprint(&self) -> bool;
	fn is_letter(&self) -> bool;
}

pub trait CharExtraTrait02
{
	fn new() -> Self;
}

impl CharExtraTrait01 for char
{
	fn isprint(&self) -> bool
	{
		let tmp = *self as u8;
		(tmp > 31
			&& tmp < 127)
		|| tmp == 13
		|| tmp == 9
	}

	fn unprint(&self) -> bool
	{
		let tmp = *self as u8;
		(tmp < 31
		&& tmp != 13
		&& tmp != 9)
		|| tmp == 127
	}

	fn is_letter(&self) -> bool
	{
		let tmp = *self as u8;
		(tmp > 64
			&& tmp < 91)
		|| (tmp > 96
			&& tmp < 123)
	}
}

impl CharExtraTrait02 for [char; 28]
{
    fn new() -> Self
	{
        let mut forbidden_chars: [char; 28] = ['\0'; 28];
		forbidden_chars.copy_from_slice(
			&FORBIDDEN_STRS.chars()
			.collect::<Box<[char]>>());
		forbidden_chars
    }
}

impl CharExtraTrait02 for [char; 67]
{
    fn new() -> Self
	{
        let mut forbidden_chars: [char; 67] = ['\0'; 67];
		forbidden_chars.copy_from_slice(
			&PERMITTED_STRS.chars()
			.collect::<Box<[char]>>());
		forbidden_chars
    }
}

pub fn detect_forbidden_characters(tmp2: &char) -> bool
{
	for tmp in <[char; 28] as CharExtraTrait02>::new()
	{
		if tmp2 == &tmp
		{
			return true;
		}
	}

	false
}

pub fn detect_empty_characters(tmp2: &char) -> bool
{
	if tmp2 == &' '
	|| tmp2 == &'\0'
	{
		return false;
	}

	for tmp in <[char; 67] as CharExtraTrait02>::new()
	{
		if tmp2 == &tmp
		{
			return true;
		}
	}

	panic_now(4);
}