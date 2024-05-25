// mkdir.rs
// 本文件包含 mkdir 命令模块。

/*
	模块重要资源列表

本模块仅定义了公共函数__mkdir。

*/

use std::fs;
use std::path::PathBuf;
use crate::error_message;
use crate::error_type::GenResult;
use super::cd;
use super::ls;

pub fn __mkdir<'a>(tmp1: &'a str, tmp2: Vec<char>) -> GenResult<PathBuf>
{
	if tmp2.len() > 1
	{
		return Err(error_message!(12));
	}

	let mode: bool = if tmp2.len() == 0
		{
			false
		}
		else
		{
			match tmp2[0]
			{
				'p'	=> true,
				_	=> return Err(error_message!(12)),
			}
		};

	let (tmp_path_01, tmp_path_vec) = cd::clean_path_2(&tmp1, false)?;
	let tmp2 = tmp_path_vec.len();
	if tmp2 == 0
	{
		return Err(error_message!(19));
	}

	if ! tmp_path_01.is_dir()
	{
		return Err(error_message!(15, ls::filename_to_string(&tmp_path_01)?));
	}

	let tmp_path_03: PathBuf = tmp_path_01.join(&tmp_path_vec[0]);
	let mut tmp_path_02 = tmp_path_03.clone();
	if mode
	{
		if tmp2 > 1
		{
			tmp_path_vec[1..]
			.iter()
			.for_each(
				|tmp|
				{
					tmp_path_02.push(tmp);
				});
		}
		if let Err(e) = fs::create_dir_all(&tmp_path_02)
		{
			return Err(error_message!(11, e.to_string()));
		}

		Ok(tmp_path_03)
	}
	else
	{
		if tmp2 > 1
		{
			return Err(error_message!(7));
		}

		if let Err(e) = fs::create_dir(&tmp_path_02)
		{
			return Err(error_message!(11, e.to_string()));
		}

		Ok(tmp_path_03)
	}
}