// pwd.rs
// 本文件包含 pwd 命令模块。

/*
	模块重要资源列表

本模块仅定义了公共函数__pwd。

*/

use std::path::PathBuf;
use super::{cd, ls,};

pub fn __pwd(tmp: bool) -> String
{
	let tmp1: PathBuf = cd::current_dir_check();
	if tmp
	{
		cd::path_to_string(&tmp1)
	}
	else
	{
		ls::clean_vec_path_0(&vec![tmp1], false)
	}
}