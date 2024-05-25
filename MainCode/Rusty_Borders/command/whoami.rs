// whoami.rs
// 本文件包含 pwd 命令模块。

/*
	模块重要资源列表

本模块仅定义了公共函数__whoami。

*/

use username::get_user_name;

pub fn __whoami(tmp: bool) -> String
{
	if tmp
	{
		"Jessarin".to_string()
	}
	else
	{
		get_user_name().unwrap_or("None".to_string())
	}
}