// cls.rs
// 本文件包含 cls 命令模块。

/*
	模块重要资源列表

本模块仅定义了公共函数__cls。

*/

use std::process::Command;

pub fn __cls() -> String
{
	let _ = Command::new("PowerShell").arg("-Command").arg("cls").output();
	"clear_OK".to_string()
}