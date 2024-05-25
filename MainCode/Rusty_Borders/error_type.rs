// error_type.rs
// 本文件包含错误处理模块。

/*
	模块重要资源列表

	*列表顺序按首字母顺序排列（不包含结构体成员顺序）。

#==============================#
	本模块定义的结构体有：
行号	是否公有	名称				包含的成员（名称 ：类型）
----    --------    ----				----------
		pub			CustomError			num		:	i32,
										lvl		:	i32,
										error	:	String,

#==============================#
	本模块实现的类型有：
行号	是否公有	类型		名称			包含的函数
----    --------	----		----			----------
		pub			struct		CustomError		new
												from
												num
												lvl
												error

#==============================#
	本模块实现的特征Trait有：
行号	名称				实现特性				包含的函数
----	----				--------				----------
		CustomError			fmt::Display			fmt
		CustomError			std::error::Error		description

#==============================#
	本模块定义的函数有：
行号	是否公有	名称					参数			返回值
----    --------    ----					----			------
		pub			error_lvl				i32				String
		pub			error_message			i32, String		CustomError
		pub			error_message_string	i32, String		String

#==============================#
	本模块定义的宏有：
行号	是否公有	名称
----    --------    ----
		export		error_message

#==============================#
	本模块定义的别名有：
行号	是否公有	名称			别名
----    --------    ----			----
		pub			GenResult<T>	result::Result<T, CustomError>

*/

use std::fmt;
use std::result;

pub fn error_lvl(tmp: i32) -> String
{
	match tmp
	{
		0 => "INFO".to_string(),
		1 => "[Trivial]".to_string(),
		2 => "[Minor]".to_string(),
		3 => "[Normal]".to_string(),
		4 => "[MAJOR]".to_string(),
		5 => "[CRITICAL]".to_string(),
		_ => "NULL".to_string(),
	}
}

pub fn error_message_string(tmp1: i32, tmp2: String) -> String
{
	match tmp1
	{
		1  => "无法解析，索引超出范围。".to_string(),
		2  => "不支持在本系统上运行。".to_string(),
		3  => "未知路径。".to_string(),
		4  => "未知盘符。".to_string(),
		5  => "权限不足。".to_string(),
		6  => "base 不是 self 的前缀。".to_string(),
		7  => "目标不存在。".to_string(),
		8  => format!("无法转化为{tmp2}。"),
		9  => "请输入目标路径。 使用 help 命令获取更多帮助".to_string(),
		10 => "无法初始化。".to_string(),
		11 => format!("关键操作失败：{tmp2}"),
		12 => "参数错误，使用 help 命令获取更多帮助".to_string(),
		13 => "无法提取文件信息。".to_string(),
		14 => format!("{tmp2}不是个文件。"),
		15 => format!("{tmp2}不是个目录。"),
		16 => "可创建文件数量已达上线。".to_string(),
		17 => "可创建目录数量已达上线。".to_string(),
		18 => "此命令不支持删除环境文件。\n若的确需要修改环境文件，可使用wipe命令。".to_string(),
		19 => "目标已存在。".to_string(),
		20 => format!("{tmp2}是个目录。"),
		21 => format!("{tmp2}是个文件。"),
		_  => "未知错误。".to_string(),
	}
}

pub fn error_message(tmp: i32, tmp2: String) -> CustomError
{
	match tmp
	{
		2|3|4|9|12					=> CustomError::from(tmp, 1, error_message_string(tmp, tmp2)),
		1|6|14|15|16|17|18|20|21	=> CustomError::from(tmp, 2, error_message_string(tmp, tmp2)),
		5|7|8|10|19					=> CustomError::from(tmp, 3, error_message_string(tmp, tmp2)),
		11|13						=> CustomError::from(tmp, 4, error_message_string(tmp, tmp2)),
		_							=> CustomError::from(tmp, 5, error_message_string(tmp, tmp2)),
	}
}

#[macro_export]
macro_rules! error_message
{
	($tmp1: expr, $tmp2: expr) =>
	{
		error_message($tmp1, $tmp2)
	};
	($tmp1: expr) =>
	{
		error_message($tmp1, "".to_string())
	};
}

#[derive(Debug, Clone, PartialEq)]
pub struct CustomError
{
	num: i32,
	lvl: i32,
	error: String,
}

impl CustomError
{
	#[allow(dead_code)]
	pub fn new() -> CustomError
	{
		Self::from(0, 0, "".to_string())
	}

	pub fn from(tmp1: i32, tmp2: i32, tmp3: String) -> CustomError
	{
		CustomError
		{
			num: tmp1,
			lvl: tmp2,
			error: tmp3,
		}
	}

	pub fn num(&self) -> i32
	{
		self.num
	}

	pub fn lvl(&self) -> i32
	{
		self.lvl
	}

	pub fn error(&self) -> String
	{
		self.error.clone()
	}
}

impl fmt::Display for CustomError
{
	fn fmt(&self, tmp1: &mut fmt::Formatter) -> Result<(), fmt::Error>
	{
		write!(tmp1, "{}号错误：\n  等级：{}_{}\n  详细信息：{}",
			self.num,
			self.lvl,
			error_lvl(self.lvl),
			self.error)
	}
}

impl std::error::Error for CustomError
{
	fn description(&self) -> &str
	{
		&self.error
	}
}

pub type GenResult<T> = result::Result<T, CustomError>;

#[test]
fn test_error_type()
{
	assert_eq!(error_message!(5), error_message(5, "".to_string()));
	assert_eq!(error_message!(5), CustomError{
		num: 5,
		lvl: 3,
		error: "请求无法完成，权限不足。".to_string()
	});
	assert_eq!(error_message!(11, "关键操作失败。".to_string()), CustomError{
		num: 11,
		lvl: 5,
		error: "关键操作失败：关键操作失败。".to_string()
	});
	assert_eq!(error_message!(11, "关键操作失败。".to_string()), CustomError::from(11, 5, "关键操作失败：关键操作失败。".to_string()));
}