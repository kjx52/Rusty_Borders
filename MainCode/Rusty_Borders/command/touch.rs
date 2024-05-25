// touch.rs
// 本文件包含 touch 命令模块。

/*
	模块重要资源列表

本模块仅定义了公共函数__touch。

*/

use std::fs::{self, File, FileTimes, OpenOptions};
use std::path::PathBuf;
use std::time::SystemTime;
use crate::error_message;
use crate::error_type::GenResult;
use crate::misc::*;
use crate::ryb_config::HOME_PATH_03;
use super::cd;
use super::ls;

pub fn __touch<'a>(tmp1: &'a str) -> GenResult<PathBuf>
{
	let (tmp_path_01, tmp_path_vec) = cd::clean_path_2(&tmp1, false)?;
	let tmp2 = tmp_path_vec.len();
	let tmp2 = match tmp2
		{
			0 =>
			{
				if tmp_path_01.is_file()
				{
					let tmp = File::options()
						.write(true)
						.open(&tmp_path_01)
						.ok()
						.ok_or(error_message!(13))?;
					let time = FileTimes::new()
						.set_modified(SystemTime::now());
					if let Err(e) = tmp.set_times(time)
					{
						return Err(error_message!(11, e.to_string()));
					}
					return Ok(tmp_path_01);
				}
				if tmp_path_01.is_dir()
				{
					return Err(error_message!(20, ls::filename_to_string(&tmp_path_01)?));
				}
				else
				{
					return Err(error_message!(14, "目标".to_string()))
				}
			}
			_ => tmp2 - 1,
		};

	if ! tmp_path_01.is_dir()
	{
		return Err(error_message!(15, ls::filename_to_string(&tmp_path_01)?));
	}

	let tmp_path_03: PathBuf = tmp_path_01.join(&tmp_path_vec[0]);
	let mut tmp_path_02 = tmp_path_03.clone();
	if tmp2 > 0
	{
		tmp_path_vec[1..tmp2]
			.iter()
			.for_each(
				|tmp|
				{
					tmp_path_02.push(tmp);
				});
		fs::create_dir_all(&tmp_path_02)
			.ok()
			.ok_or(error_message!(11, "无法创建目录".to_string()))?;
	}

	let tmp_path_01: PathBuf =
		if let Ok(t) = tmp_path_02.strip_prefix(HOME_PATH_03)
		{
			let t = PathBuf::from("/").join(t);
			cd::clean_path_2(&cd::path_to_string(&t), false)?.0.join(&tmp_path_vec[tmp2])
		}
		else
		{
			panic_now(7)
		};
	
	match OpenOptions::new()
		.append(true)
		.create(true)
		.open(&tmp_path_01)
	{
		Ok(_) => Ok(tmp_path_03),
		Err(e) => Err(error_message!(11, e.to_string())),
	}
}