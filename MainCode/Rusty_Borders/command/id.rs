// id.rs
// 本文件包含 id 命令模块。

/*
	模块重要资源列表

本模块仅定义了公共函数__id。

*/

use super::whoami::__whoami;

pub fn __id(user_key: i32, only_member_bool: bool, user_permission: bool) -> String
{
	let (tmp1, tmp2, tmp3) =
		match (user_key, only_member_bool, user_permission)
		{
			(0, true, _)		=> (0, 0, 0),
			(0, false, true)	=> (1000, 1000, 0),
			(0, false, false)	=> (4000, 4000, 0),
			(1, _, _)			=> (197609, 4000, 1000),
			(9, _, _)			=> (197609, 197609, 197609),
			_					=> (999999, 999999, 999999),
		};

	format!("uid={tmp1}({}) gid={tmp2} groups={tmp3}", __whoami(only_member_bool))
}