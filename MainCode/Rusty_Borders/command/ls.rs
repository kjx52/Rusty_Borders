// ls.rs
// 本文件包含 ls 命令模块。

/*
	模块重要资源列表

	*列表顺序按首字母顺序排列（不包含结构体成员顺序）。

#==============================#
	本模块定义的函数有：
行号	是否公有	名称						参数						返回值
----    --------    ----						----						------
		private		datain_dirs					&PathBuf, i32				GenResult<Vec<Vec<PathBuf>>>
		private		format_vec_string			&Vec<String>, bool			String
		private		filename_to_string			&PathBuf					GenResult<String>
		private		lgdatain_dir2				&Vec<PathBuf>				GenResult<Vec<String>>
												i32
												bool
		private		print_vec_string			&Vec<Vec<String>>, bool
		pub			clean_vec_path_0			&Vec<PathBuf>, bool			String
		pub			dir_is_empty				&PathBuf					bool
		pub			__ls						&'a str, Vec<char>			GenResult<(String, i32)>

*/

use chrono::prelude::*;
use std::fs;
use std::path::PathBuf;
use std::str::FromStr;
use crate::error_message;
use crate::error_type::*;
use crate::misc::*;
use crate::ryb_config::HOME_PATH_03;
use super::cd;

pub fn filename_to_string(tmp: &PathBuf) -> GenResult<String>
{
	Ok(match tmp.file_name().ok_or(error_message!(13))?.to_str().ok_or(error_message!(8, "str".to_string()))?
		{
			"Shell" => "/",
			tmp => tmp,
		}.to_string())
}

pub fn dir_is_empty(tmp: &PathBuf) -> bool
{
	if let Ok(mut t) = tmp.read_dir()
	{
		t.next().is_none()
	}
	else
	{
		false
	}
}

pub fn clean_vec_path_0(tmp1: &Vec<PathBuf>, key: bool) -> String
{
	match tmp1[0].strip_prefix(HOME_PATH_03)
	{
		Ok(t) =>
		{
			let mut tmp: String = parse_pair(Vec::with_capacity(5),
					&cd::path_to_string(&t.to_path_buf()),
					'\\')
				.into_iter()
				.map(|tmp|
					{
						"/".to_string() + &tmp
					})
				.collect::<String>();
			if key
			&& t == cd::return_current_path()
			{
				tmp = ".".to_string();
			}

			tmp
		},
		Err(_) => panic_now(7),
	}
}

fn lgdatain_dir2(tmp1: &Vec<PathBuf>, mode: i32, key: bool) -> GenResult<Vec<String>>
{
	/*
		以下是关于Windows文件权限的研究结果：
		* [X]表示近期版本不会考虑加入此项权限的检测
		·l（链接）[X]
		·d（目录）
		·a（存档）
		·r（只读）
		·h（隐藏）
		·s（系统）[X]

		darh--

		128   200   文件 索引文件内容 ------
		1     1     文件 索引文件内容读取 --r---
		2     2     文件 索引文件内容隐藏 ---h--
		40    40    文件 索引文件内容可执行（存档文件） -a----
		2042  4000  文件 索引文件内容压缩数据 ------

		8192  20000 文件 ------
		8193  20001 文件 读取 --r---
		8194  20002 文件 隐藏 ---h--
		8224  20040 文件 可执行（存档文件） -a----
		10240 24000 文件 压缩数据 ------

		缺省  220   文件夹 索引文件内容 d-----
		缺省  21    文件夹 索引文件内容读取 d-r---
		缺省  22    文件夹 索引文件内容隐藏 d--h--
		缺省  60    文件夹 索引文件内容可执行（存档文件） da----
		缺省  4020  文件夹 索引文件内容压缩数据 d-----

		缺省  20020 文件夹 d-----
		缺省  20021 文件夹 读取 d-r---
		缺省  20022 文件夹 隐藏 d--h--
		缺省  20060 文件夹 可执行（存档文件） da----
		缺省  24020 文件夹 压缩数据 d-----

		由于上述属性中，仅有“只读”，“隐藏”和“存档”影响显
		式权限标识符，故仅检测这三个属性。
	*/

	let mut tmp2: Vec<String> = Vec::with_capacity(10);
	tmp2.push(clean_vec_path_0(&tmp1, key));
	
	if ! (tmp1.len() > 1)
	&& tmp1[0].is_dir()
	{
		return Ok(vec![]);
	}

	for tmp in &tmp1[1..]
	{
		let dkey: bool;
		let mut tmp4: String = "".to_string();

		match (tmp.is_file(), tmp.is_dir())
		{
			(true, false) =>
			{
				tmp4 += "-";
				dkey = false;
			},
			(false, true) =>
			{
				tmp4 += "d";
				dkey = true;
			},
			_ => return Err(error_message!(99)),
		};

		let tmp6 = fs::metadata(tmp.clone()).ok().ok_or(error_message!(13))?;
		let tmp5: Vec<String> = parse_pair(Vec::with_capacity(6), &format!("{:?}", tmp6.permissions()), ' ');
		let tmp5: i32 = i32::from_str(&tmp5[3]).ok().ok_or(error_message!(8, "i32".to_string()))?;
		let mut tmp5: i32 = i32::from_str(&format!("{tmp5:o}")).ok().ok_or(error_message!(8, "i32".to_string()))?;
		if dkey
		{
			tmp5 -= 20;
		}
		if ! (tmp5 < 20000)
		{
			tmp5 -= 20000;
		}
		if ! (tmp5 < 4000)
		{
			tmp5 -= 4000;
		}
		match tmp5
		{
			0  => tmp4 += "-----\t\t",
			1  => tmp4 += "-r---\t\t",
			2  => tmp4 += "--h--\t\t",
			3  => tmp4 += "-rh--\t\t",
			40 => tmp4 += "a----\t\t",
			41 => tmp4 += "ar---\t\t",
			42 => tmp4 += "a-h--\t\t",
			43 => tmp4 += "arh--\t\t",
			_  => return Err(error_message!(99)),
		}
		let tmp5: DateTime<Local> = DateTime::from(tmp6.modified().ok().ok_or(error_message!(8, "SystemTime".to_string()))?);
		tmp4 += &tmp5.format("%Y/%m/%d     %H:%M").to_string();
		let tmp7: u64 = tmp6.len();
		if dkey
		{
			tmp4 += "\t\t\t";
		}
		else
		{
			if ! (mode - 3 < 0)
			{
				let mut key: bool = true;
				if key
				&& tmp7 > 1073741824
				{
					tmp4 += &format!("\t\t{:.1}GB\t", tmp7 / 1073741824);
					key = false;
				}
				if key
				&& tmp7 > 1048576
				{
					tmp4 += &format!("\t\t{:.1}MB\t", tmp7 / 1048576);
					key = false;
				}
				if key
				&& tmp7 > 1024
				{
					tmp4 += &format!("\t\t{:.1}KB\t", tmp7 / 1024);
					key = false;
				}
				if key
				{
					tmp4 += &format!("\t\t{}B\t", tmp7);
				}
			}
			else
			{
				tmp4 += &format!("\t\t{tmp7}\t");
			}
		}
		let mut tmp3 = filename_to_string(tmp)?;
		if tmp.is_dir()
		{
			tmp3 = format!("\x1b[34m{tmp3}/\x1b[0m");
		}
		tmp4 += &tmp3;
		tmp2.push(tmp4);
	}
	Ok(tmp2)
}

fn datain_dirs(tmp1: &PathBuf, mode: i32) -> GenResult<Vec<Vec<PathBuf>>>
{
	let mut tmp2: Vec<Vec<PathBuf>> = Vec::with_capacity(10);
	let mut tmp3: Vec<PathBuf> = Vec::with_capacity(10);
	tmp3.push(tmp1.clone());
	if tmp1.is_dir()
	{
		for entry in fs::read_dir(tmp1)
						.ok()
						.ok_or(error_message!(5))?
		{
			let entry = entry
						.ok()
						.ok_or(error_message!(8, "Path".to_string()))?
						.path()
						.to_path_buf();
			if ! (mode - 8 < 0)
			{
				tmp3.push(entry.clone());
				if entry.is_dir()
				&& ! dir_is_empty(&entry)
				{
					datain_dirs(&entry, mode)?
						.into_iter()
						.for_each(
					|tmp|
					{
						tmp2.push(tmp);
					});
				}
			}
			else if ! (mode - 4 < 0)
			{
				if entry.is_dir()
				{
					tmp3.push(entry);
				}
			}
			else
			{
				tmp3.push(entry);
			}
		}
	}
	else
	{
		tmp3.push(tmp1.to_path_buf());
	}

	tmp2.push(tmp3);

	Ok(tmp2)
}

fn format_vec_string(tmp1: &Vec<String>, mode: bool) -> String
{
	let mut tmp3: i32 = 1;
	if tmp1.len() == 0
	|| tmp1.len() == 1
	{
		return "".to_string();
	}
	let mut tmp2: String = "".to_string();
	match mode
	{
		true => &tmp1[1..]
			.into_iter()
			.for_each(|tmp|
			{
				tmp2 += &format!(" {tmp}");
				if tmp3 % 5 == 0
				{
					tmp2 += "\n";
				}
				tmp3 += 1;
			}),
		false =>
		{
			tmp2 += &format!("{}:\n", tmp1[0]);
			&tmp1[1..]
				.into_iter()
				.for_each(|tmp|
				{
					tmp2 += &format!(" {tmp}\n");
				})
		},
	};
	tmp2 += "\n";

	tmp2
}

fn print_vec_string(tmp1: &Vec<Vec<String>>, mode: bool)
{
	let mut tmp2: String = "".to_string();
	tmp1
		.iter()
		.for_each(
			|tmp|
			{
				tmp2 += &format_vec_string(tmp, mode);
			});

	println!("{}", tmp2);
}

pub fn __ls<'a>(tmp1: &'a str, tmp3: Vec<char>) -> GenResult<(String, i32)>
{
	let mut mode: i32 = 0;
	let mut tmp_path_01: PathBuf = cd::current_dir_check();
	if tmp1.len() > 0
	{
		tmp_path_01 = cd::clean_path_2(&tmp1, true)?.0;
	}
	let mut key2: bool = false;

	if PathBuf::from(tmp1) == PathBuf::from("./")
	{
		key2 = true;
	}
	tmp3
	.iter()
	.for_each(
		|tmp|
		{
			match tmp
			{
				'l' => mode += 1,
				'h' => mode += 2,
				'd' => mode += 4,
				'R' => mode += 8,
				_ => (),
			}
		});

	let tmp2: Vec<Vec<PathBuf>> = datain_dirs(&tmp_path_01, mode)?;
	let mode_res = mode;
	if ! (mode < 8)
	{
		mode -= 8;
	}
	if ! (mode < 4)
	{
		mode -= 4;
	}

	let (tmp2, tmp3) = 
		if ! (mode - 1 < 0)
		&& mode != 2
		{
			let mut tmp3: Vec<Vec<String>> = Vec::with_capacity(8);
			for tmp in tmp2.iter().rev()
			{
				tmp3.push(lgdatain_dir2(&tmp, mode, key2)?);
			}
			(tmp3, false)
		}
		else
		{
			(tmp2
			.iter()
			.rev()
			.map(
				|tmp|
				{
					if ! (mode_res < 8)
					{
						let mut tmp2: Vec<String> = Vec::with_capacity(5);
						tmp2.push("".to_string());
						tmp2.push(format!("{}\n{}:\n", 8_u8 as char, clean_vec_path_0(tmp, key2)));
						tmp[1..]
						.iter()
						.for_each(
							|tmp|
							{
								let mut tmp3 = filename_to_string(tmp)
									.unwrap_or("None".to_string());
								if tmp.is_dir()
								{
									tmp3 = format!("\x1b[34m{}/\x1b[0m", tmp3);
								}
								tmp2.push(tmp3);
							});
						tmp2
					}
					else
					{
						tmp
						.iter()
						.map(
							|tmp|
							{
								let tmp3 = filename_to_string(tmp)
									.unwrap_or("None".to_string());
								if tmp.is_dir()
								{
									format!("\x1b[34m{}/\x1b[0m", tmp3)
								}
								else
								{
									tmp3
								}
							}).collect::<Vec<String>>()
					}
				}).collect::<Vec<Vec<String>>>()
			, true)
		};

	print_vec_string(&tmp2, tmp3);

	Ok((clean_vec_path_0(&vec![tmp_path_01], false), mode_res))
}