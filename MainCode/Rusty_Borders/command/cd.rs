// cd.rs
// 本文件包含 cd 命令模块。

/*
	模块重要资源列表

	*列表顺序按首字母顺序排列（不包含结构体成员顺序）。

#==============================#
	本模块定义的函数有：
行号	是否公有	名称						参数		返回值
----    --------    ----						----		------
X		private		absolute_chdir				&'a Path	GenResult<()>
		private		clean_path					&'a str		GenResult<(bool, PathBuf)>
		private		match_dir_01_current		&'a Path	GenResult<PathBuf>
		private		match_dir_02_parent			&'a Path	GenResult<PathBuf>
		private		match_path_start			PathBuf		PathBuf
		private		match_path_with_root_path	PathBuf		bool
		private		path_to_string				&PathBuf	String
		pub			chdir						&'a str		GenResult<()>
		pub			clean_path_2				&'a str		GenResult<PathBuf>
		pub			current_dir_check						PathBuf
		pub			return_current_path						PathBuf
		pub			__cd						&'a str		GenResult<()>


#==============================#
	本模块定义的宏有：
行号	是否公有	名称
----    --------    ----
		export		match_path_with_root_path

*/

use std::env;
use std::fs;
use std::path::{Path, PathBuf,};
use crate::error_message;
use crate::error_type::{CustomError, GenResult,};
use crate::misc::*;
use crate::ryb_config::{HOME_PATH_01, HOME_PATH_03, HOME_PATH_04};

pub fn path_to_string(tmp: &PathBuf) -> String
{
	tmp.display().to_string()
}

fn match_path_start(tmp: &PathBuf) -> PathBuf
{
	if tmp.starts_with(r###"\\?\C:"###)
	{
		let tmp: String = path_to_string(&tmp);
		PathBuf::from(&tmp[4..])
	}
	else
	{
		tmp.to_path_buf()
	}
}

pub fn current_dir_check() -> PathBuf
{
	if let Ok(t) = env::current_dir()
	{
		match_path_start(&t)
	}
	else
	{
		PathBuf::from("None")
	}
}

fn match_path_with_root_path(tmp: &PathBuf) -> bool
{
	if tmp == &PathBuf::from("None")
	{
		true
	}
	else
	{
		! tmp.starts_with(HOME_PATH_03)
		&& ! tmp.starts_with(HOME_PATH_04)
	}
}

macro_rules! match_path_with_root_path
{
	($tmp1: expr) =>
	{
		match_path_with_root_path($tmp1)
	};
	() =>
	{
		match_path_with_root_path(&current_dir_check())
	};
}

pub fn return_current_path() -> PathBuf
{
	let tmp: PathBuf = current_dir_check();
	if tmp != PathBuf::from("None")
	&& tmp.starts_with(HOME_PATH_03)
	&& tmp.starts_with(HOME_PATH_04)
	{
		if let Ok(r) = tmp.strip_prefix(HOME_PATH_03)
		{
			return r.to_path_buf();
		}
	}

	panic_now(7);
}

fn match_dir_01_current<'a>(tmp1: &'a Path) -> GenResult<PathBuf>
{
	if tmp1.starts_with(".///")
	{
		if let Ok(tmp4) = tmp1.strip_prefix("./")
		{
			match_dir_01_current(tmp4)
		}
		else
		{
			return Err(error_message!(6));
		}
	}
	else
	{
		Ok(tmp1.to_path_buf())
	}
}

fn match_dir_02_parent<'a>(tmp1: &'a Path) -> GenResult<PathBuf>
{
	if tmp1.starts_with("..///")
	{
		if let Ok(tmp4) = tmp1.strip_prefix("../")
		{
			if return_current_path() != PathBuf::from("")
			{
				match env::set_current_dir(&Path::new("../"))
				{
					Ok(()) => match_dir_02_parent(tmp4),
					Err(e) => Err(error_message!(11, e.to_string())),
				}
			}
			else
			{
				match_dir_02_parent(tmp4)
			}
		}
		else
		{
			Err(error_message!(6))
		}
	}
	else
	{
		Ok(tmp1.to_path_buf())
	}
}

pub fn clean_path_2<'a>(tmp1: &'a str, tmp3: bool) -> GenResult<(PathBuf, Vec<PathBuf>)>
{
	let tmp: usize = tmp1.len();
	if ! (tmp < 21)
	{
		panic_now(6);
	}
	if tmp == 0
	{
		return Err(error_message!(9));
	}
	let mut tmp_path_01: PathBuf = PathBuf::from(tmp1);
	let mut tmp_path_vec: Vec<PathBuf> = Vec::with_capacity(3);

	if tmp_path_01.has_root()
	{
		let tmp2: Vec<char> = str_to_vec_char(tmp1);

		if tmp_path_01.is_absolute()
		|| ! tmp_path_01.starts_with("////")
		|| (tmp > 1
			&& ! (tmp2[0] == '/'
				|| tmp2[1].is_letter()))
		{
			return Err(error_message!(2));
		}

		tmp_path_01 = PathBuf::from(HOME_PATH_03);
		tmp_path_01.as_mut_os_string().push(tmp1);
	}
	else
	{
		tmp_path_01 = current_dir_check().join(tmp1);
	}
	
	tmp_path_01 = loop
	{
		if let Ok(t) = tmp_path_01.canonicalize()
		{
			break match_path_start(&t)
		}
		else
		{
			tmp_path_vec.push(tmp_path_01.file_name().ok_or(error_message!(7))?.into());
			tmp_path_01.pop();
			if tmp3
			|| match_path_with_root_path!(&tmp_path_01)
			{
				return Err(error_message!(7));
			}
		};
	};

	if match_path_with_root_path!(&tmp_path_01)
	{
		return Err(error_message!(7));
	}

	Ok((tmp_path_01, tmp_path_vec.into_iter().rev().collect::<Vec<PathBuf>>()))
}

pub fn __cd<'a>(tmp1: &'a str) -> GenResult<()>
{
	let tmp: usize = tmp1.len();
	if ! (tmp < 21)
	{
		panic_now(6);
	}
	if tmp == 0
	{
		if let Err(e) = env::set_current_dir(&PathBuf::from(HOME_PATH_01))
		{
			return Err(error_message!(11, e.to_string()));
		}
	}

	let mut tmp_path_01: PathBuf = PathBuf::from(tmp1);

	if parse_pair(Vec::with_capacity(3), tmp1, ' ').len() > 1
	{
		return Err(error_message!(12));
	}

	if tmp_path_01.has_root()
	{
		let tmp2: Vec<char> = str_to_vec_char(tmp1);

		if tmp_path_01.is_absolute()
		|| ! tmp_path_01.starts_with("////")
		|| (tmp > 1
			&& ! (tmp2[0] == '/'
				|| tmp2[1].is_letter()))
		{
			return Err(error_message!(2));
		}

		if let Err(e) = env::set_current_dir(Path::new(HOME_PATH_03))
		{
			return Err(error_message!(11, e.to_string()));
		}

		if let Ok(t) = tmp_path_01.strip_prefix("/")
		{
			tmp_path_01 = t.to_path_buf();
		}
	}

	let critical_path = |tmp1: &str| -> bool
	{
		let mut tmp = tmp1.chars();
		if let (Some(t), Some(r)) = (tmp.next(), tmp.next())
		{
			t.is_letter() && r == ':'
		}
		else
		{
			false
		}
	};

	loop
	{
		let tmp: usize = path_to_string(&tmp_path_01).len();
		if tmp > 1
		&& tmp_path_01.starts_with(".///")
		{
			tmp_path_01 = match_dir_01_current(&tmp_path_01)?;
		}
		else if tmp == 1
		&& tmp_path_01.starts_with(".")
		{
			tmp_path_01 = tmp_path_01
				.strip_prefix(".")
				.ok()
				.ok_or(error_message!(6))?
				.to_path_buf();
		}

		let tmp: usize = path_to_string(&tmp_path_01).len();
		if tmp > 2
		&& tmp_path_01.starts_with("..///")
		{
			tmp_path_01 = match_dir_02_parent(&tmp_path_01)?;
		}
		else if tmp == 2
		&& tmp_path_01.starts_with("..")
		{
			if return_current_path() != PathBuf::from("")
			{
				if let Err(e) = env::set_current_dir(&Path::new("../"))
				{
					return Err(error_message!(11, e.to_string()));
				}
			}
			tmp_path_01 = tmp_path_01
				.strip_prefix("..")
				.ok()
				.ok_or(error_message!(6))?
				.to_path_buf();
		}

		if tmp_path_01.has_root()
		{
			return Err(error_message!(2));
		}

		if tmp_path_01.starts_with("///")
		&& tmp_path_01.strip_prefix("///").is_err()
		{
			return Err(error_message!(10));
		}

		let mut tmp_path_03: PathBuf = current_dir_check();
		let tmp2: String = path_to_string(&match_path_start(&tmp_path_01));
		let tmp_path_04: Vec<String> = parse_pair(Vec::with_capacity(7),
			&tmp2,
			'/');

		let mut tmp3: usize = 0;
		for tmp in 0..tmp_path_04.len()
		{
			if critical_path(&tmp_path_04[tmp])
			{
				return Err(error_message!(4));
			}

			if tmp_path_04[tmp] != ".."
			&& tmp_path_04[tmp] != "."
			{
				if tmp_path_04[tmp].starts_with("...")
				{
					return Err(error_message!(7));
				}
				tmp_path_01 = PathBuf::from("");
				tmp_path_03.push(tmp_path_04[tmp].clone());
				tmp3 += tmp_path_04[tmp].len() + 1;
			}
			else
			{
				tmp_path_01 = PathBuf::from(mid_to_string(&tmp2, tmp3));
				break;
			}
		}
		if tmp_path_03.is_dir()
		{
			if let Err(e) = env::set_current_dir(&tmp_path_03)
			{
				return Err(error_message!(11, e.to_string()));
			}
		}
		else
		{
			match fs::metadata(&tmp_path_03)
			{
				Err(e) => return Err(CustomError::from(14, 3, e.to_string())),
				_ => return Err(CustomError::from(13, 3,
					format!("ERROR: 请求无法完成，{}是一个{}文件",
						match Path::new(tmp1).file_name()
						{
							Some(t) => t.to_str().ok_or(error_message!(8, "str".to_string()))?,
							None => tmp1,
						},
						match Path::new(tmp1).extension()
						{
							Some(t) => t.to_str().ok_or(error_message!(8, "str".to_string()))?,
							None => "",
						}))),
			}
		}

		if match_path_with_root_path!()
		{
			return Err(error_message!(7));
		}

		if tmp_path_01 == PathBuf::from("")
		{
			break;
		}
	}

	Ok(())
}