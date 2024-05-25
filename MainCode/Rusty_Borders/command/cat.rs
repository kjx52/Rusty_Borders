// cat.rs
// 本文件包含 cat 命令模块。

/*
	模块重要资源列表

本模块仅定义了公共函数__cat。

*/

use super::*;
use crate::error_type::{CustomError, GenResult,};

pub fn __cat<'a>(tmp1: &'a str) -> GenResult<(String, String)>
{
	let tmp_path_01: PathBuf = cd::clean_path_2(tmp1, true)?.0;
	if ! tmp_path_01.is_file()
	{
		return Err(CustomError::from(13,
			3,
			format!("ERROR: 请求无法完成，读取对象是个目录")));
	}

	let tmp_path_01: String = cd::path_to_string(&tmp_path_01);
	let tmp2: Vec<String> = BufReader::new(
		File::open(&tmp_path_01)
		.expect("关键操作失败03"))
	.lines()
	.filter_map(|line| line.ok())
	.collect::<Vec<String>>();

	let mut tmp1: String = "".to_string();
	tmp2
		.iter()
		.for_each(
			|tmp|
			{
				tmp1 += &format!("{tmp}\n");
			});
	println!("{tmp1}");

	Ok((tmp_path_01, tmp1))
}