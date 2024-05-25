// rm.rs
// 本文件包含 rm 命令模块。

/*
	模块重要资源列表

本模块仅定义了公共函数__rm。

*/

use std::fs;
use std::path::PathBuf;
use crate::error_message;
use crate::error_type::GenResult;
use super::ls;

pub fn __rm<'a>(tmp_path_01: &PathBuf, tmp2: bool) -> GenResult<PathBuf>
{
	if tmp2
	{
		if tmp_path_01.is_dir()
		{
			if let Err(e) = fs::remove_dir_all(&tmp_path_01)
			{
				return Err(error_message!(11, e.to_string()));
			}

			return Ok(tmp_path_01.to_path_buf());
		}
	}
	if tmp_path_01.is_dir()
	{
		return Err(error_message!(20, ls::filename_to_string(&tmp_path_01)?));
	}

	if let Err(e) = fs::remove_file(&tmp_path_01)
	{
		return Err(error_message!(11, e.to_string()));
	}

	Ok(tmp_path_01.to_path_buf())
}