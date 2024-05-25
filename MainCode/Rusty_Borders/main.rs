
/*
	Rusty Borders /*危墙*/项目
	// 原 Rustic Terminus Tonic (RTT) 项目
	// 原 Rust TermX (RTX) 项目

	Jessarin000
	2024-03-19
*/

//###########################################################################
// 请自行修改 "C:\Users\Public\Control\Email_Config.cfg" 文件配置以适应环境
//###########################################################################
const LICENSE_STRS: &'static str = r####"0123456789ABCDEFGHIJKLMNOPQRSTUV"####;
static mut EINFO: EmailInfo = EmailInfo
{
	tago: String::new(),
	sosr: String::new(),
	smtp: String::new(),
	pawd: String::new(),
};

extern crate lettre;
extern crate lettre_email;

mod command;
mod powershell;
#[macro_use] pub mod error_type;
pub mod misc;
pub mod ryb_config;
pub mod cmd_info;

use chrono::prelude::*;

use getch_rs::{Getch, Key,};
use lettre_email::Email;
use lettre::smtp::authentication::Credentials;
use lettre::{SmtpClient, Transport};
use local_ip_address::local_ip;
use regex::RegexSetBuilder;
use std::env;
use std::fmt;
use std::fs::{self, File,};
use std::io::{self, Write, BufRead, BufReader};
use std::mem;
use std::net::IpAddr;
use std::path::{Path, PathBuf,};
use std::process::{exit, Command, Output,};
use std::str::FromStr;
use sysinfo::System;
use username::get_user_name;
use crate::command::cat::__cat;
use crate::command::cd::{self, __cd,};
use crate::command::cls::__cls;
use crate::command::help::__help;
use crate::command::id::__id;
use crate::command::ls::__ls;
use crate::command::mkdir::__mkdir;
use crate::command::pwd::__pwd;
use crate::command::rm::__rm;
use crate::command::su::__su;
use crate::command::touch::__touch;
use crate::command::whoami::__whoami;
use crate::command::wipe::__wipe;
use crate::error_type::*;
use crate::misc::*;
use crate::ryb_config::*;

fn get_target_ip() -> (String, String)
{
	let rule = RegexSetBuilder::new(&[
		r###"Accepted password for kali from \d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3} port \d{1,5}"###])
		.case_insensitive(true)
		.build()
		.expect("出乎意料的失策03");

	let mut line: Vec<String> = BufReader::new(
			File::open(SSH_LOG_FILE)
			.expect(&error_message_string(11, "01".to_string())))
		.lines()
		.filter_map(|tmp| tmp.ok())
		.filter(|tmp| rule.is_match(&tmp))
		.collect::<Vec<String>>();

	if let Some(t) = line.pop()
	{
		let mut res: Vec<String> = parse_pair(Vec::with_capacity(14), &t, ' ');
		return (mem::replace(&mut res[8], "NULL".to_string())
			, mem::replace(&mut res[10], "NULL".to_string()));
	}

	("NULL".to_string(), "NULL".to_string())
}

unsafe fn eget()
{
	if let Ok(line) = read_lines(ECFG)
	{
		for tmp in line
		{
			if let Ok(t) = tmp
			{
				if t.chars().next() == Some('#')
				{
					continue;
				}

				let emailvec = parse_pair(Vec::with_capacity(4), &t, '=');
				if emailvec.len() != 2
				{
					continue;
				}

				match emailvec[0].as_str()
				{
					"TAGO" => EINFO.tago = emailvec[1].to_string(),
					"SOSR" => EINFO.sosr = emailvec[1].to_string(),
					"SMTP" => EINFO.smtp = emailvec[1].to_string(),
					"PAWD" => EINFO.pawd = emailvec[1].to_string(),
					_ => continue,
				}
			}
		}
	}
}

unsafe fn e_mail_sender<'a>(tmp: &'a str, tmp1: &'a str, tmp3: i32)
{
	let email = Email::builder()
	.to(EINFO.tago.to_string())
	.from(EINFO.sosr.to_string())
	.subject("RyB警告邮件")
	.html(r###"<html>
	<head>
		<meta charset="utf-8" name="viewport" content="width=device-width, initial-scale=1.0, minimum-scale=1.0, maximum-scale=1.0, user-scalable=no" />
		<title>RyB警告邮件</title>
		<style type="text/css">
			#header_mob
			{
				height:60px;
				line-height: 60px;
				position: static;
				top: 0;
				width: 100%;
				text-align: center;
				background: black ;
				font-family: Arial;
				letter-spacing: 1px;
			}

			#footer_mob
			{
				height:80px;
				line-height: 20px;
				position: fixed;
				bottom: 0;
				width: 100%;
				text-align: center;
				background: #333;
				color: #fff;
				font-family: Arial;
				font-size: 9px;
				letter-spacing: 1px;
			}
		</style>
	</head>
	<body>
		<div id="header_mob">
			<B><I>
			<font size="7" color="red">&nbsp;Rusty Borders</font>
			</I></B>
		</div>
		<p>
			"###.to_string() + &format!("
			<font color=\"{}\">
				<h1>
					{}：{}
				</h1>
				<br>
				{}
			</font>", if tmp3 == 5
				{
					"red"
				}
				else
				{
					"black"
				}, error_lvl(tmp3), tmp, tmp1) + r###"
		</p>
		<div id="footer_mob">
			Copyright © 2022-2026&nbsp;
			<strong>10.129.136.9</strong> 
			All Rights Reserved.
			<br>
			备案号：陕ICP备21049801号-1
			<br>
			Power By <B><I>Jessarin000</I></B>
		</div>
	</body>
</html>"###)
	.build()
	.expect(&error_message_string(11, "01".to_string()));

	let tmp2 = Credentials::new(
		EINFO.sosr.to_string(),
		EINFO.pawd.to_string(),
	);

	let mut mailer = SmtpClient::new_simple(&EINFO.smtp)
	.expect(&error_message_string(11, "02".to_string()))
	.credentials(tmp2)
	.transport();

	let result = mailer.send(email.into());

	if result.is_err()
	{
		panic_now(8);
	}

	mailer.close();
}

#[test]
fn test_e_mail_sender()
{
	e_mail_sender("SYSTEM_INFO文件写入失败", "[2024/4/23 12:18:48] 目标： 警告原因：", 2);
	let local: DateTime<Local> = Local::now();
	let cur_time = local.format("%Y-%m-%d %H:%M:%S").to_string();
	let (ip, port) = get_target_ip();
	e_mail_sender("SYSTEM_INFO文件写入失败", &format!("[{}] 目标：{}:{} 警告原因：NULL", cur_time, ip, port), 3);
	e_mail_sender("程序退出node_01循环。", &format!("RyB于[{}]退出loop 'node_01，链接目标：{}:{}", cur_time, ip, port), 5);
}

fn mark_file<'a>(tmp2: &'a str, tmp1: &'static str, tmp: &'a str, tmp3: char)
{
	let local: DateTime<Local> = Local::now();
	let cur_time =
		if tmp2 == "SYSTEM_INFO"
		{
			local.format("%Y-%m-%d %H:%M:%S").to_string()
		}
		else
		{
			local.format("%H:%M:%S").to_string()
		};

	let tmp: String = match tmp3
	{
		'0' => r##"<font color="blue">"##.to_string() + tmp + "</font>",
		'1' => r##"<B><font color="blue">"##.to_string() + tmp + "</font></B>",
		'2' => r##"<font color="yellow">"##.to_string() + tmp + "</font>",
		'3' => r##"<B><font color="yellow">"##.to_string() + tmp + "</font></B>",
		'4' => r##"<font color="red">"##.to_string() + tmp + "</font>",
		'5' => r##"<I><B><font color="red">"##.to_string() + tmp + "</font></B></I>",
		_ => tmp.to_string(),
	};
	match write_a(tmp1)
	{
		Ok(mut fs) => if let Err(e) = fs.write_all(format!("> <font color=\"gray\">[{}]</font>\t# {}\n", cur_time, tmp).as_bytes())
		{
			let (ip, port) = get_target_ip();
			unsafe { e_mail_sender(&format!("{}文件写入失败", tmp2), &format!("[{}] 目标：{}:{} 警告原因：{:?}", cur_time, ip, port, e), 3) };
		},
		Err(e) =>
		{
			let (ip, port) = get_target_ip();
			unsafe { e_mail_sender(&format!("{}文件打开失败", tmp2), &format!("[{}] 目标：{}:{} 警告原因：{:?}", cur_time, ip, port, e), 3) };
		},
	}
}

#[macro_export]
macro_rules! mark_file
{
	($tmp1: expr, $tmp2: expr, $tmp3: expr, $tmp4: expr) =>
	{
		mark_file($tmp1, $tmp2, $tmp3, $tmp4)
	};
	($tmp1: expr, $tmp2: expr, $tmp3: expr) =>
	{
		mark_file($tmp1, $tmp2, $tmp3, ' ')
	};
}

fn user_info() -> (String, bool)
{
	match get_user_name()
	{
		Ok(t) =>
		{
			let command_string_01: String =
				r###"[bool]((whoami /groups) -match "S-1-16-12288")"###
				.to_string();
			let output: Output = Command::new("PowerShell")
								  .arg("-Command")
								  .arg(&command_string_01)
								  .output()
								  .expect("出乎意料的失策02");

			if let Ok(r) = String::from_utf8(output.stdout)
			{
				if r.trim() == "True"
				{
					return (t, true);
				}
			}

			(t, false)
		}
		_ => (format!("None"), false),
	}
}

fn target<'a>(tmp1: &[char; 25], tmp2: &[char]) -> bool
{
	let mut tmp_flag_01: usize = 0;

	let compare_str = |tmp3: &usize| -> bool
	{
		for tmp in 0 .. tmp2.len()
		{
			if ! compare_char(&tmp1[*tmp3 + tmp], &tmp2[tmp])
			{
				return true;
			}
		}
		false
	};

	loop
	{
		if tmp_flag_01 == 21
		{
			return false;
		}

		if compare_char(&tmp1[tmp_flag_01], &tmp2[0])
		{
			if tmp_flag_01 > 0
			&& detect_empty_characters(&tmp1[tmp_flag_01 - 1])
			{
				tmp_flag_01 += 1;
				continue;
			}

			if compare_str(&tmp_flag_01)
			{
				tmp_flag_01 += 1;
			}
			else
			{
				if tmp_flag_01 < 21
				&& detect_empty_characters(&tmp1[tmp_flag_01 + tmp2.len()])
				{
					tmp_flag_01 += 1;
					continue;
				}

				return true;
			}
		}
		else
		{
			tmp_flag_01 += 1;
		}
	}
}

fn read_lines<T>(filename: T) -> io::Result<io::Lines<io::BufReader<File>>>
where T: AsRef<Path>,
{
	let file = File::open(filename)?;
	Ok(io::BufReader::new(file).lines())
}

fn extraction_data<'a>(tmp1: &'a str, tmp2: char, tmp3: usize) -> Vec<String>
{
	if let Ok(mut line) = read_lines(tmp1)
	{
		let tmp = line.nth(tmp3);
		if let Some(Ok(t)) = &tmp
		{
			parse_pair(Vec::with_capacity(7), &t, tmp2)
		}
		else
		{
			vec![]
		}
	}
	else
	{
		vec![]
	}
}

fn get_date_code(tmp1: &u8) -> GenResult<usize>
{
	match LICENSE_STRS.find(*tmp1 as char)
	{
		Some(index) => Ok(index),
		None =>	Err(error_message!(1)),
	}
}

fn verification_permission<'a>(tmp1: &'a str) -> (Vec<String>, i32)
{
	let mut tmp2: Vec<String> = extraction_data(tmp1, '-', 0);
	if tmp2.len() != 7
	{
		return (vec!["License Not Found".to_string()], 99)
	}

	let (tmp3, tmp4) = match tmp2[0].as_str()
	{
		"ASTLL" => ("Never_Expires".to_string(), true),
		"BSTLL" => ("Valid_for_One_Year".to_string(), false),
		"CSTLL" => ("Valid_for_Three_Month".to_string(), false),
		"ASR"	=> ("Temporary_License".to_string(), true),
		_		=> return (vec!["No License".to_string()], 99),
	};
	{
		tmp2.push(tmp3);
		if (tmp4 && &tmp2[1] != "0000")
		|| (!tmp4 && &tmp2[1] == "0000")
		{
			return (vec!["No License".to_string()], 99);
		}
		if ! tmp4
		{
			let tmp3: &[u8] = &tmp2[1].as_bytes();
			let tmp3: Vec<usize> = tmp3.iter().map(|tmp| { match get_date_code(tmp)
				{
					Ok(t) => t,
					Err(_e) => 0,
				}
			}).collect::<Vec<usize>>();
			if tmp3.len() != 4
			{
				return (vec!["No License".to_string()], 99);
			}
			tmp2.push(format!("20{}{}-{:>02}-{:>02}", tmp3[0], tmp3[1], tmp3[2], tmp3[3]));
		}
		else
		{
			tmp2.push("0000".to_string());
		}
	}
	let tmp3: i32;
	tmp2.push(match tmp2[5].as_str()
	{
		"92DSC" =>
			{
				tmp3 = 0;
				"Project_Developer".to_string()
			},
		"92DBC" =>
			{
				tmp3 = 1;
				"Beta_Participants".to_string()
			},
		"00000" =>
			{
				tmp3 = 9;
				"User".to_string()
			},
		_		=> return (vec!["No License".to_string()], 99),
	});

	(tmp2, tmp3)
}

#[derive(Debug, Clone, PartialEq)]
struct AllowCmd
{
	cmd1: Vec<bool>,
	cmd2: Vec<bool>,
	cmd3: Vec<bool>,
	cmd4: Vec<bool>,
	cmd5: Vec<bool>,
	error: bool,
}

impl AllowCmd
{
	fn new() -> AllowCmd
	{
		AllowCmd
		{
			cmd1: vec![false, false, false],
			cmd2: vec![false, false, false],
			cmd3: vec![false, false, false],
			cmd4: vec![false, false, false],
			cmd5: vec![false, false, false],
			error: false,
		}
	}
}

impl fmt::Display for AllowCmd
{
	fn fmt(&self, tmp1: &mut fmt::Formatter) -> Result<(), fmt::Error>
	{
		write!(tmp1, "AllowCmd:
{{
	cmd1: {:?},
	cmd2: {:?},
	cmd3: {:?},
	cmd4: {:?},
	cmd5: {:?},
	error: {},
}}", self.cmd1, self.cmd2, self.cmd3, self.cmd4, self.cmd5, self.error)
	}
}

fn str_to_t<T: FromStr>(s: &str) -> Option<T>
{
	match T::from_str(&s)
	{
		Ok(l) => Some(l),
		_ => None
	}
}

fn choose_cmd(gen_allow_cmd: &AllowCmd, warn_allow_cmd: &AllowCmd) -> (bool, AllowCmd, AllowCmd)
{
	let str_to_alcmd_bool = |tmp: &str, tmp1: &mut AllowCmd, tmp2: usize|
	{
		match tmp
		{
			"" => (),
			"0" => tmp1.cmd1[tmp2] = true,
			"1" => tmp1.cmd2[tmp2] = true,
			"2" => tmp1.cmd3[tmp2] = true,
			"3" => tmp1.cmd4[tmp2] = true,
			"4" => tmp1.cmd5[tmp2] = true,
			_ => tmp1.error = true,
		};
	};

	let mut tmp1: AllowCmd = AllowCmd::new();
	let mut tmp2: AllowCmd = AllowCmd::new();

	let mut tmp3: Vec<String> = extraction_data(CMD_CONTROL_FILE, 'g', 0);
	let mut tmp4: Vec<String> = extraction_data(CMD_CONTROL_FILE, 'w', 1);

	let _ = tmp3.iter_mut().for_each(
		|tmp|
		{
			let mut tmp5: usize = 0;
			if let Some(t) = str_to_t::<usize>(tmp)
			{
				if t > 4
				{
					*tmp = (t - 5).to_string();
					tmp5 += 1;
				}
			}
			str_to_alcmd_bool(tmp, &mut tmp1, tmp5);
		});
	let _ = tmp4.iter_mut().for_each(
		|tmp|
		{
			let mut tmp5: usize = 0;
			if let Some(t) = str_to_t::<usize>(tmp)
			{
				if t > 4
				{
					*tmp = (t - 5).to_string();
					tmp5 += 1;
				}
			}
			str_to_alcmd_bool(tmp, &mut tmp2, tmp5);
		});

	if gen_allow_cmd == &tmp1
	&& warn_allow_cmd == &tmp2
	{
		return (false, AllowCmd::new(), AllowCmd::new());
	}

	(true, tmp1, tmp2)
}




fn main()
{
	unsafe { eget() };

	let mut only_member_bool: bool = false;
	let mut info_count: u32 = 0;
	let mut warn_count: u32 = 0;
	let mut error_count: u32 = 0;

	let _ = io::stdout().flush();
	let _ = Command::new("PowerShell").arg("-Command").arg("cls").output();
	let (license, user_key) = verification_permission(LICENSE);
	let license2 = extraction_data(LICENSE, '-', 0);
	mark_file!("SYSTEM_INFO", SYSTEM_INFO,
		&format!("【信息】 已激活 Rusty Borders 伪终端。\n 许可证号：{}", license2.into_iter().collect::<String>()), '1');
	info_count += 1;
	let (ip, port) = get_target_ip();
	mark_file!("SYSTEM_INFO", SYSTEM_INFO,
		&format!("【信息】已连接至 {}:{}", ip, port), '1');
	info_count += 1;

	let home: PathBuf = PathBuf::from(HOME_PATH_01);
	if let Err(e) = fs::metadata(&home)
	{
		println!("\n\n\x1b[34m[\x1b[0m\x1b[1;31mCRITICAL\x1b[0m\x1b[34m]\x1b[0m\x1b[1;31m 关键操作失败：{}\x1b[0m", e);
		panic_now(1);
	}

	let _ = env::set_current_dir(HOME_PATH_01).expect(
		"\n\n\x1b[;5;31m
			######################################
			#              %%警告%%              #
			#        %%RyB 核心进程受阻%%        #
			#          %%！紧急终止！%%          #
			#          Critical Code 03          #
			######################################
		\x1b[0m");

	/*
	pub enum Key {
		EOF,				// Null byte.
		Backspace,			// Backspace.
		Delete,				// Delete key.
		Esc,				// Esc key.
		Up,					// Up arrow.
		Down,				// Down arrow.
		Right,				// Right arrow.
		Left,				// Left arrow.
		End,				// End key.
		Home,				// Home key.
		BackTab,			// Backward Tab key.
		Insert,				// Insert key.
		PageUp,				// Page Up key.
		PageDown,			// Page Down key.
		F(u8),				// Function keys.
							// Only function keys 1 through 12 are supported.
		Char(char),			// Normal character.
		Alt(char),			// Alt modified character.
		Ctrl(char),			// Ctrl modified character.
							// Note that certain keys may not be modifiable with ctrl, due to limitations of terminals.
		Ctrl + Backspace,	// CtrlBackspace,
		Other(Vec<u8>),		// Other key.
	}
	*/

	#[allow(unused_assignments)]
	let mut ac_command: [char; 25] = ['\0'; 25];
	let reset = |tmp2: usize|
	{
		for _tmp in 0 .. tmp2
		{
			print!("{}", 8_u8 as char);
		}
		for _tmp in 0 .. 30
		{
			print!(" ");
		}
		for _tmp in 0 .. 30
		{
			print!("{}", 8_u8 as char);
		}
	};

	let check_result = |tmp1: Option<String>| -> String
	{
		if let Some(t) = tmp1
		{
			t
		}
		else
		{
			format!("None")
		}
	};

	let system_version: String = check_result(System::os_version());
	let system_name: String = check_result(System::name());
	let host_name: String = check_result(System::host_name());
	let host_ip_address: IpAddr = match local_ip()
	{
		Ok(t) => t,
		_ => IpAddr::from([
			25u8, 24u8, 23u8, 22u8, 21u8, 20u8, 19u8, 18u8,
			17u8, 16u8, 15u8, 14u8, 13u8, 12u8, 11u8, 10u8,
		]),
	};

	let (env_user, mut user_permission) = user_info();
	let mut gen_allow_cmd: AllowCmd = AllowCmd::new();
	let mut warn_allow_cmd: AllowCmd = AllowCmd::new();
	let terminal_type = |tmp: bool, tmp2: bool|
	{
		if tmp
		{
			println!("\n\x1b[1;31m┌──【『Ｏnly Member 👑 {}』】-[\x1b[0m {} \x1b[1;31m]\x1b[0m", host_name, cd::path_to_string(&cd::current_dir_check()));
			print!("\x1b[1;31m└─￥\x1b[0m ");
			return ();
		}
		let print_path: String =
		{
			let tmp1 = cd::return_current_path();
			if let Ok(t) = &tmp1.strip_prefix(HOME_PATH_02)
			{
				let mut t = t.to_path_buf();
				if t != PathBuf::from("")
				{
					t = PathBuf::from("/").join(t);
				}
				"~".to_string() + &cd::path_to_string(&t)
			}
			else
			{
				let tmp1 = cd::path_to_string(&tmp1);
				if &tmp1 == ""
				{
					"/".to_string()
				}
				else if ! tmp1.starts_with("///")
				{
					format!("/{}", tmp1)
				}
				else
				{
					tmp1
				}
			}
		};

		if tmp2
		{
			println!("\n\x1b[34m┌──(\x1b[0m\x1b[31m {}💀{} \x1b[0m\x1b[34m)-[\x1b[0m {} \x1b[34m]\x1b[0m", env_user, host_name, print_path);
			print!("\x1b[34m└─\x1b[0m\x1b[31m#\x1b[0m ");
		}
		else
		{
			println!("\n\x1b[32m┌──(\x1b[0m\x1b[34m {}@{} \x1b[0m\x1b[32m)-[\x1b[0m {} \x1b[32m]\x1b[0m", env_user, host_name, print_path);
			print!("\x1b[32m└─\x1b[0m\x1b[34m$\x1b[0m ");
		}
	};

	mark_file!("SYSTEM_INFO", SYSTEM_INFO, "[信息] 初始化完成，进入深度循环。", '0');
	info_count += 1;

	let mut loop_num: i32 = 0;
	let mut loop_permit: Vec<i32> = Vec::with_capacity(5);
	let mut file_crate_list: Vec<PathBuf> = Vec::with_capacity(8);
	let mut dirs_crate_list: Vec<PathBuf> = Vec::with_capacity(5);
	let mut file_crate_num: usize = 0;
	let mut dirs_crate_num: usize = 0;
	'node_01:
	loop
	{
		#[allow(unused_assignments)]
		let mut counter_01: usize = 0;
		
		ac_command = ['\0'; 25];
		terminal_type(only_member_bool, user_permission);
		let _ = io::stdout().flush();

		'node_02:
		loop
		{
			counter_01 = 0;
			let tmp1: Getch = Getch::new();
			let mut char_key_flag_01: usize = 0;
			let mut char_key_flag_03: usize = 0;
			loop {
				print!("");
				let _ = io::stdout().flush();
				if counter_01 > 20
				{
					println!("\x1b[0m");
					println!("[\x1b[33m!\x1b[0m] \x1b[33m命令最大长度：20字节。 WarningCode:04\x1b[0m");
					continue 'node_01;
				}
				match tmp1.getch()
				{
					Ok(Key::EOF) =>
					{
						panic_now(3);
					},
					Ok(Key::Backspace) =>
					{
						if ! (char_key_flag_01 > 0)
						{
							continue;
						}

						let mut tmp4: i32 = 0;
						if char_key_flag_01 != counter_01
						{
							let mut judgment_code_01: bool = true;
							if char_key_flag_03 == 0
							{
								judgment_code_01 = false;
							}

							if judgment_code_01
							&& char_key_flag_01 > 1
							&& ac_command[char_key_flag_01 - 2] != ' '
							&& ac_command[char_key_flag_01] != ' '
							&& ac_command[char_key_flag_01 - 1] == ' '
							&& char_key_flag_01 == char_key_flag_03 + 1
							{
								char_key_flag_03 = counter_01;
							}

							for tmp in char_key_flag_01 .. counter_01
							{
								ac_command[tmp - 1] = ac_command[tmp];
							}
							ac_command[counter_01 - 1] = '\0';
							print!("{}", 8_u8 as char);
							counter_01 -= 1;
							char_key_flag_01 -= 1;
							match judgment_code_01
							{
								true =>
								{
									for tmp in char_key_flag_01 .. counter_01
									{
										if tmp < (char_key_flag_03 + 1)
										{
											if ac_command[tmp] == ' '
											{
												print!(" ");
												char_key_flag_03 = tmp;
											}
											else
											{
												print!("\x1b[1;34m{}\x1b[0m", ac_command[tmp]);
											}
										}
										else
										{
											print!("{}", ac_command[tmp]);
										}
									}
								},
								false =>
								{
									for tmp in char_key_flag_01 .. counter_01
									{
										print!("\x1b[1;34m{}\x1b[0m", ac_command[tmp]);
									}
								},
							}
							print!(" ");
							for _tmp in char_key_flag_01 .. (counter_01 + 1)
							{
								print!("{}", 8_u8 as char);
							}
							if judgment_code_01 && char_key_flag_03 == 19
							{
								char_key_flag_03 = 0;
							}
							continue;
						}
						else
						{
							if ac_command[counter_01 - 1] == ' '
							{
								for tmp in 0 .. (counter_01 - 1)
								{
									if ac_command[tmp] == ' '
									{
										tmp4 = 163;
										break;
									}
								}
								if tmp4 != 163
								{
									char_key_flag_03 = 0;
								}
							}
							reset(1);
							counter_01 -= 1;
							char_key_flag_01 -= 1;
							ac_command[counter_01] = '\0';
							continue;
						}
					}
					Ok(Key::Delete) =>
					{
						if ! (char_key_flag_01 < counter_01)
						{
							continue;
						}

						if char_key_flag_01 == counter_01
						{
							continue;
						}

						let mut judgment_code_01: bool = true;
						if char_key_flag_03 == 0
						{
							judgment_code_01 = false;
						}

						if judgment_code_01
						&& char_key_flag_01 < (counter_01 - 1)
						&& ac_command[char_key_flag_01 + 1] != ' '
						&& ac_command[char_key_flag_01 - 1] != ' '
						&& ac_command[char_key_flag_01] == ' '
						{
							char_key_flag_03 = 20;
						}

						for tmp in char_key_flag_01 .. counter_01
						{
							ac_command[tmp] = ac_command[tmp + 1];
						}
						ac_command[counter_01] = '\0';
						counter_01 -= 1;
						match judgment_code_01
						{
							true =>
							{
								for tmp in char_key_flag_01 .. counter_01
								{
									if tmp < (char_key_flag_03 + 1)
									{
										if ac_command[tmp] == ' '
										{
											print!(" ");
											char_key_flag_03 = tmp;
										}
										else
										{
											print!("\x1b[1;34m{}\x1b[0m", ac_command[tmp]);
										}
									}
									else
									{
										print!("{}", ac_command[tmp]);
									}
								}
							},
							false =>
							{
								for tmp in char_key_flag_01 .. counter_01
								{
									print!("\x1b[1;34m{}\x1b[0m", ac_command[tmp]);
								}
							},
						}
						print!(" ");
						for _tmp in char_key_flag_01 .. (counter_01 + 1)
						{
							print!("{}", 8_u8 as char);
						}
						if judgment_code_01 && char_key_flag_03 == 19
						{
							char_key_flag_03 = 0;
						}
						continue;
					}
					Ok(Key::Esc) => continue,
					Ok(Key::Up) => continue,
					Ok(Key::Down) => continue,
					Ok(Key::Right) =>
					{
						if ! (char_key_flag_01 < counter_01)
						{
							continue;
						}

						print!("\x1b[C");
						char_key_flag_01 += 1;
					}
					Ok(Key::Left) =>
					{
						if ! (char_key_flag_01 > 0)
						{
							continue;
						}

						print!("\x1b[D");
						char_key_flag_01 -= 1;
					}
					Ok(Key::End) => continue,
					Ok(Key::Home) => continue,
					Ok(Key::BackTab) => continue,
					Ok(Key::Insert) => continue,
					Ok(Key::PageUp) => continue,
					Ok(Key::PageDown) => continue,
					Ok(Key::F(tmp2)) =>
					{
						if tmp2 == 2_u8
						{
							if license.len() != 10
							{
								println!("");
								println!("\x1b[1;30m######################################\x1b[0m");
								println!("\x1b[1;30m#\x1b[0m \x1b[34m许可证编号\x1b[0m：\x1b[31m无许可_{}\x1b[0m", license[0]);
								println!("\x1b[1;30m#\x1b[0m \x1b[34m操作系统\x1b[0m：{} {}", system_name, system_version);
								println!("\x1b[1;30m#\x1b[0m \x1b[34m设备编号\x1b[0m：无");
								println!("\x1b[1;30m#\x1b[0m \x1b[34m工作链路\x1b[0m：RTT \x1b[1;30m->\x1b[0m {:?}: 4444 \x1b[1;30m->\x1b[0m localhost: 2222", host_ip_address);
								println!("\x1b[1;30m#\x1b[0m \x1b[34m终端状态\x1b[0m：\x1b[1;30m⨂ 未激活\x1b[0m，正在运行");
								println!("\x1b[1;30m#\x1b[0m \x1b[34m激活日期\x1b[0m：无");
								println!("\x1b[1;30m#\x1b[0m \x1b[34m使用期限\x1b[0m：无");
								println!("\x1b[1;30m#\x1b[0m");
								println!("\x1b[1;30m#\x1b[0m   0 \x1b[31m错误\x1b[0m； 0 \x1b[33m警告\x1b[0m； 0 \x1b[34m信息\x1b[0m；");
								println!("\x1b[1;30m#\x1b[0m      未触发诧异或未定义行为。");
								println!("\x1b[1;30m######################################\x1b[0m");
							}
							else
							{
								println!("");
								println!("\x1b[1;30m######################################\x1b[0m");
								print!("\x1b[1;30m#\x1b[0m \x1b[34m许可证编号\x1b[0m：");
								for tmp in 0 .. 7
								{
									print!("{}", license[tmp]);
									if tmp < 6
									{
										print!("\x1b[1;30m-\x1b[0m");
									}
								}
								println!("");
								println!("\x1b[1;30m#\x1b[0m \x1b[34m操作系统\x1b[0m：{} {}", system_name, system_version);
								println!("\x1b[1;30m#\x1b[0m \x1b[34m设备编号\x1b[0m：{}_{}", license[2], license[6]);
								println!("\x1b[1;30m#\x1b[0m \x1b[34m工作链路\x1b[0m：RTT \x1b[1;30m->\x1b[0m {:?}: 4444 \x1b[1;30m->\x1b[0m localhost: 2222", host_ip_address);
								println!("\x1b[1;30m#\x1b[0m \x1b[34m终端状态\x1b[0m：\x1b[1;32m● 已激活: {}\x1b[0m，正在运行", license[9]);
								println!("\x1b[1;30m#\x1b[0m \x1b[34m激活日期\x1b[0m：{}", license[8]);
								println!("\x1b[1;30m#\x1b[0m \x1b[34m使用期限\x1b[0m：{}", license[7]);
								println!("\x1b[1;30m#\x1b[0m");
								println!("\x1b[1;30m#\x1b[0m   {} \x1b[31m错误\x1b[0m； {} \x1b[33m警告\x1b[0m； {} \x1b[34m信息\x1b[0m；", error_count, warn_count, info_count);
								println!("\x1b[1;30m#\x1b[0m      未触发诧异或未定义行为。");
								println!("\x1b[1;30m######################################\x1b[0m");
							}
							mark_file!("SYSTEM_INFO", SYSTEM_INFO, "[信息] 用户使用F2读取RyB系统信息。", '0');
						}
						reset(counter_01);
						continue 'node_01;
					}
					Ok(Key::Char(tmp2)) =>
					{
						if tmp2.unprint()
						{
							panic_now(5);
						}
						if counter_01 == 0
						{
							match tmp2
							{
								'\r' =>
								{
									println!("");
									continue 'node_01;
								}
								' ' =>
								{
									continue 'node_02;
								}
								_ => (),
							}
						}
						if tmp2 == ' '
						{
							if char_key_flag_03 == 0
							{
								char_key_flag_03 = char_key_flag_01;
							}
							print!("\x1b[0m");
						}
						if tmp2 == '\t'
						{
							reset(counter_01);
							continue 'node_01;
						}
						if tmp2 == 13_u8 as char
						{
							println!("");
							break 'node_02;
						}
						if detect_forbidden_characters(&tmp2)
						{
							reset(counter_01);
							ac_command = ['\0'; 25];
							continue 'node_02;
						}

						if char_key_flag_01 != counter_01
						{
							let mut judgment_code_01: bool = true;
							let mut judgment_code_02: bool = true;
							if char_key_flag_03 == 0
							{
								judgment_code_01 = false;
							}

							for tmp in (char_key_flag_01 + 1 .. counter_01 + 1).rev()
							{
								ac_command[tmp] = ac_command[tmp - 1];
							}
							ac_command[char_key_flag_01] = tmp2;

							match judgment_code_01
							{
								true =>
								{
									if char_key_flag_01 < (char_key_flag_03 + 1)
									{
										if ac_command[char_key_flag_01] == ' '
										{
											if char_key_flag_01 == 0
											{
												reset(0);
												ac_command = ['\0'; 25];
												continue 'node_02;
											}
											print!(" ");
											char_key_flag_03 = char_key_flag_01;
										}
										else
										{
											print!("\x1b[1;34m{}\x1b[0m", ac_command[char_key_flag_01]);
											char_key_flag_03 += 1;
										}
									}
									else
									{
										print!("{}", tmp2);
										judgment_code_02 = false;
									}
								}
								false =>
								{
									if ac_command[char_key_flag_01] == ' ' && char_key_flag_01 == 0
									{
										reset(0);
										ac_command = ['\0'; 25];
										continue 'node_02;
									}
									print!("\x1b[1;34m{}\x1b[0m", ac_command[char_key_flag_01]);
								}
							}

							counter_01 += 1;
							char_key_flag_01 += 1;

							match judgment_code_01
							{
								true =>
								{
									for tmp in char_key_flag_01 .. (char_key_flag_03 + 1)
									{
										print!("\x1b[1;34m{}\x1b[0m", ac_command[tmp]);
									}
									if judgment_code_02
									{
										for tmp in (char_key_flag_03 + 1) .. counter_01
										{
											print!("{}", ac_command[tmp]);
										}
									}
									else
									{
										for tmp in char_key_flag_01 .. counter_01
										{
											print!("{}", ac_command[tmp]);
										}
									}
									for _tmp in char_key_flag_01 .. counter_01
									{
										print!("{}", 8_u8 as char);
									}
								}
								false =>
								{
									for tmp in char_key_flag_01 .. counter_01
									{
										print!("\x1b[1;34m{}\x1b[0m", ac_command[tmp]);
									}
									for _tmp in char_key_flag_01 .. counter_01
									{
										print!("{}", 8_u8 as char);
									}
								}
							}
						}
						else
						{
							ac_command[counter_01] = tmp2;
							if char_key_flag_03 == 0
							{
								print!("\x1b[1;34m{}\x1b[0m",tmp2);
							}
							else
							{
								print!("{}",tmp2);
							}
							counter_01 += 1;
							char_key_flag_01 += 1;
						}

						if tmp2 == 'Z'
						{
							if loop_num > 0
							{
								loop_num -= 1;
								match loop_permit.pop()
								{
									Some(0) => (only_member_bool, user_permission) = (true, true),
									Some(1) => (only_member_bool, user_permission) = (false, true),
									Some(_) => (only_member_bool, user_permission) = (false, false),
									None => panic_now(9),
								}
								continue 'node_01;
							}
							else
							{
								if file_crate_num != 0
								{
									file_crate_list
									.iter()
									.for_each(
										|tmp|
										{
											if let Err(e) = __rm(&tmp, true)
											{
												mark_file!("SYSTEM_INFO", SYSTEM_INFO,
													&format!("[错误]：\n {}", e), '3');
											}
										});
								}
								if dirs_crate_num != 0
								{
									dirs_crate_list
									.iter()
									.for_each(
										|tmp|
										{
											if let Err(e) = __rm(&tmp, true)
											{
												mark_file!("SYSTEM_INFO", SYSTEM_INFO,
													&format!("[错误]：\n {}", e), '3');
											}
										});
								}
								println!("\n退出登录。");
							}
							mark_file!("SYSTEM_INFO", SYSTEM_INFO, "【信息】 Rusty Borders 伪终端正常终止。", '1');
							exit(0x000);
						}
					}
					Ok(Key::Alt(_tmp2)) => continue,
					Ok(Key::Ctrl(_tmp2)) => continue,
					Ok(Key::CtrlBackspace) => continue,
					Ok(Key::Other(_tmp2)) => continue,
					Err(e) => println!("{}", e),
				}
			}
		}

		mark_file!("USER_COMMAND", USER_COMMAND, &ac_command[0 .. counter_01]
					.iter()
					.collect::<String>(), '9');
		
		struct Space<'s> { space_01: &'s dyn Fn(&Space, [char; 25], usize, usize) -> ([char; 25], usize, usize) }
		let nullspace_iter = Space
		{
			space_01: &|nullspace_iter, mut ac_command, tmp, mut counter_01|
			{
				if ac_command[tmp] == ' '
				&& ac_command[tmp + 1] == ' '
				{
					for tmp1 in tmp .. counter_01
					{
						ac_command[tmp1] = ac_command[tmp1 + 1];
					}
					ac_command[counter_01] = '\0';
					counter_01 -= 1;

					return (nullspace_iter.space_01)(nullspace_iter, ac_command, tmp, counter_01);
				}

				(ac_command, tmp + 1, counter_01)
			},
		};

		let mut tmp: usize = 0;
		loop
		{
			(ac_command, tmp, counter_01) = (nullspace_iter.space_01)(&nullspace_iter, ac_command.clone(), tmp, counter_01.clone());
			if tmp >= counter_01
			{
				break;
			}
		}

		let os_shell_01: Vec<char> = misc::str_to_vec_char(KEY_CODE_01);
		let os_shell_02: Vec<char> = misc::str_to_vec_char(KEY_CODE_02);
		let os_shell_03: Vec<char> = misc::str_to_vec_char(KEY_CODE_03);
		let os_shell_04: Vec<char> = misc::str_to_vec_char(KEY_CODE_04);

		let os_shell: Vec<&[char]> =
		vec![&os_shell_01.as_slice(),
			 &os_shell_02.as_slice(),
			 &os_shell_03.as_slice(),
			 &os_shell_04.as_slice()];
		
		let res_01: Vec<bool> = os_shell
		.iter()
		.map(
			|tmp|
			{
				target(&ac_command, tmp)
			})
		.collect::<Vec<bool>>();

		for tmp in 0 .. res_01.len()
		{
			if res_01[tmp]
			{
				if only_member_bool
				{
					powershell::powershell_act();
					continue 'node_01;
				}
				else
				{
					println!("\x1b[1m[\x1b[1;31m-\x1b[0m\x1b[1m] \x1b[1;31m访问被拒绝。 ErrorCode:02{}\x1b[0m\n", tmp + 1);
					let _ = io::stdout().flush();
					continue 'node_01;
				}
			}
		}

		let updata_info = choose_cmd(&gen_allow_cmd, &warn_allow_cmd);
		if updata_info.0
		{
			mark_file!("SYSTEM_INFO", SYSTEM_INFO,
				"[信息] 命令列表更新。", '0');
			info_count += 1;
			gen_allow_cmd = updata_info.1;
			warn_allow_cmd = updata_info.2;
		}
		if gen_allow_cmd.error
		{
			mark_file!("SYSTEM_INFO", SYSTEM_INFO,
				"[错误]：GEN_CMD 组error值非预期的活跃。", '3');
			error_count += 1;
			continue 'node_01;
		}
		if warn_allow_cmd.error
		{
			mark_file!("SYSTEM_INFO", SYSTEM_INFO,
				"[错误]：WARN_CMD 组error值非预期的活跃。", '3');
			error_count += 1;
			continue 'node_01;
		}

		let ac_command: String = ac_command[.. counter_01]
				.iter()
				.collect::<String>();

		// cd
		let cd = |tmp1: &str| -> bool
		{
			let tmp = cd::current_dir_check();
			match __cd(&tmp1)
			{
				Ok(()) =>
				{
					mark_file!("SYSTEM_RESULT",
						SYSTEM_RESULT,
						&__pwd(false),
						'9');
					false
				}
				Err(e) =>
				{
					let _ = env::set_current_dir(tmp)
						.expect(&error_message_string(11, "03".to_string()));
					mark_file!("SYSTEM_INFO", SYSTEM_INFO,
						&format!("[警告]：\n {}", e), '2');
					println!("cd: {}", e.error());
					true
				}
			}
		};

		// ls
		let ls = |tmp1: &str, tmp2: Vec<char>| -> bool
		{
			match __ls(&tmp1, tmp2)
			{
				Ok(t) =>
				{
					mark_file!("SYSTEM_RESULT",
						SYSTEM_RESULT,
						&format!("<font color=\"66FFFF\">list_res</font>: {}\n<font color=\"66FFFF\">mode</font>: {}",
							t.0,
							t.1),
						'9');
					false
				}
				Err(e) =>
				{
					mark_file!("SYSTEM_INFO", SYSTEM_INFO,
						&format!("[警告]：\n {}", e), '2');
					println!("ls: {}", e.error());
					true
				}
			}
		};

		// cat
		let cat = |tmp1: &str| -> bool
		{
			match __cat(&tmp1)
			{
				Ok(t) =>
				{
					mark_file!("SYSTEM_RESULT",
						SYSTEM_RESULT,
						&format!("<font color=\"66FFFF\">cat_file_path</font>: {}\n<font color=\"66FFFF\">file_content</font>: {}",
							t.0,
							t.1),
						'9');
					false
				}
				Err(e) =>
				{
					mark_file!("SYSTEM_INFO", SYSTEM_INFO,
						&format!("[警告]：\n {}", e), '2');
					println!("cat: {}", e.error());
					true
				}
			}
		};

		// cls
		let cls = || -> bool
		{
			mark_file!("SYSTEM_RESULT", SYSTEM_RESULT, &__cls(), '9');
			false
		};

		// pwd
		let pwd = |tmp1: bool| -> bool
		{
			let tmp = __pwd(tmp1);
			mark_file!("SYSTEM_RESULT", SYSTEM_RESULT, &tmp, '9');
			println!("{tmp}");
			false
		};

		// whoami
		let whoami = |tmp1: bool| -> bool
		{
			let tmp = __whoami(tmp1);
			mark_file!("SYSTEM_RESULT", SYSTEM_RESULT, &tmp, '9');
			println!("{tmp}");
			false
		};

		// hostname
		let hostname = || -> bool
		{
			mark_file!("SYSTEM_RESULT", SYSTEM_RESULT, &host_name, '9');
			println!("{host_name}");
			false
		};

		// id
		let id = |tmp1: i32, tmp2: bool, tmp3: bool| -> bool
		{
			let tmp = __id(tmp1, tmp2, tmp3);
			mark_file!("SYSTEM_RESULT", SYSTEM_RESULT, &tmp, '9');
			println!("{tmp}");
			false
		};

		/*
			以下命令可与磁盘交互，须限制命令使用次数。
			 默认可创建文件夹数目：5 个。
			 默认可创建文件数目：8 个。
		*/

		// touch
		let touch = |tmp1: &str,
			tmp2: &mut Vec<PathBuf>,
			tmp3: usize| -> (bool, usize)
		{
			if tmp3 > 4
			{
				let tmp = error_message!(16);
				println!("touch: {tmp}");
				mark_file!("SYSTEM_INFO", SYSTEM_INFO,
						&format!("[警告]：\n {}", tmp.error()), '2');
				return (true, 0);
			}
			match __touch(&tmp1)
			{
				Ok(t) =>
				{
					mark_file!("SYSTEM_RESULT",
						SYSTEM_RESULT,
						&format!("<font color=\"66FFFF\">New_fiel_path</font>: {}", cd::path_to_string(&t)),
						'9');
					tmp2.push(t);
					(false, 1)
				}
				Err(e) =>
				{
					mark_file!("SYSTEM_INFO", SYSTEM_INFO,
						&format!("[错误]：\n {}", e), '3');
					println!("touch: {}", e.error());
					(true, 0)
				}
			}
		};

		// mkdir
		let mkdir = |tmp1: &str,
			tmp2: Vec<char>,
			tmp3: &mut Vec<PathBuf>,
			tmp4: usize| -> (bool, usize)
		{
			if tmp4 > 4
			{
				let tmp = error_message!(16);
				println!("mkdir: {tmp}");
				mark_file!("SYSTEM_INFO", SYSTEM_INFO,
						&format!("[警告]：\n {}", tmp.error()), '2');
				return (true, 0);
			}
			match __mkdir(&tmp1, tmp2)
			{
				Ok(t) =>
				{
					mark_file!("SYSTEM_RESULT",
						SYSTEM_RESULT,
						&format!("<font color=\"66FFFF\">New_dirs_path</font>: {}", cd::path_to_string(&t)),
						'9');
					tmp3.push(t);
					(false, 1)
				}
				Err(e) =>
				{
					mark_file!("SYSTEM_INFO", SYSTEM_INFO,
						&format!("[错误]：\n {}", e), '3');
					println!("mkdir: {}", e.error());
					(true, 0)
				}
			}
		};

		let rm_mode = |tmp1: Vec<char>| -> i32
		{
			let mut mode: i32 = 0;
			tmp1
			.iter()
			.for_each(
				|tmp|
				{
					match tmp
					{
						'f'	=> mode += 1,
						'r'	=> mode += 2,
						_	=>
							{
								let tmp = error_message!(12);
								println!("rm: {}", tmp.error());
								mark_file!("SYSTEM_INFO", SYSTEM_INFO,
										&format!("[警告]：\n {tmp}"), '2');
								mode += 101;
							},
					}
				});
			mode
		};

		// rm
		let mut rm = |tmp1: &str, tmp2: Vec<char>| -> bool
		{
			let mut mode: i32 = rm_mode(tmp2);
			if mode > 100
			{
				return true;
			}

			let tmp_path_01 = match cd::clean_path_2(&tmp1, true)
			{
				Ok(t) => t.0,
				Err(e) =>
					{
						if mode > 1
						{
							mode -= 2;
						}
						mark_file!("SYSTEM_INFO", SYSTEM_INFO,
							&format!("[错误]：\n {}", e), '3');
						if ! (mode > 0)
						{
							println!("rm: {}", e.error());
						}
						return true;
					}
			};
			if match_path(&dirs_crate_list, &tmp_path_01)
			&& match_path(&file_crate_list, &tmp_path_01)
			{
				let tmp = error_message!(18);
				println!("rm: {}", tmp.error());
				mark_file!("SYSTEM_INFO", SYSTEM_INFO,
						&format!("[警告]：\n {tmp}"), '2');
				return true;
			}

			match __rm(&tmp_path_01, mode > 1)
			{
				Ok(t) =>
				{
					dirs_crate_num = del_index_path(&mut dirs_crate_list, dirs_crate_num, &t);
					file_crate_num = del_index_path(&mut file_crate_list, file_crate_num, &t);

					mark_file!("SYSTEM_RESULT",
						SYSTEM_RESULT,
						&format!("<font color=\"66FFFF\">Remove_path</font>: {}", cd::path_to_string(&t)),
						'9');
					false
				}
				Err(e) =>
				{
					if mode > 1
					{
						mode -= 2;
					}
					mark_file!("SYSTEM_INFO", SYSTEM_INFO,
						&format!("[错误]：\n {}", e), '3');
					if ! (mode > 0)
					{
						println!("rm: {}", e.error());
					}
					true
				}
			}
		};

		// wipe
		let wipe = |tmp1: &str, tmp2: Vec<char>| -> bool
		{
			let mut mode: i32 = rm_mode(tmp2);
			if mode > 100
			{
				return true;
			}
			match __wipe(&tmp1, mode > 1)
			{
				Ok(t) =>
				{
					mark_file!("SYSTEM_RESULT",
						SYSTEM_RESULT,
						&format!("<font color=\"66FFFF\">Remove_path</font>: {}", cd::path_to_string(&t)),
						'9');
					false
				}
				Err(e) =>
				{
					if mode > 1
					{
						mode -= 2;
					}
					mark_file!("SYSTEM_INFO", SYSTEM_INFO,
						&format!("[错误]：\n {}", e), '3');
					if ! (mode > 0)
					{
						println!("wipe: {}", e.error());
					}
					true
				}
			}
		};

		let help = |tmp1: &str| -> bool
		{
			let tmp = __help(tmp1);
			mark_file!("SYSTEM_RESULT", SYSTEM_RESULT, &tmp1, '9');
			println!("{tmp}");
			false
		};

		// 这是一个隐藏命令，建议仅在测试时使用它。
		// 若想禁用，请移除许可证。
		// su
		let mut su = |tmp1: &str| -> bool
		{
			match __su(&tmp1, user_key, only_member_bool, user_permission)
			{
				Ok(t) =>
				{
					(only_member_bool, user_permission) = (t.0, t.1);
					loop_permit.push(t.2);
					if only_member_bool && user_permission
					{
						mark_file!("SYSTEM_INFO", SYSTEM_INFO,
							"[紧急]：\n 最高权限账户ONLY_MEMBER被启动了，等级：true:true。", '4');
					}
					else
					{
						mark_file!("SYSTEM_INFO", SYSTEM_INFO,
							&format!("[错误]：\n Sudo被启动了，等级：{}:{}。", t.0, t.1), '3');
					}
					loop_num += 1;
					true
				}
				Err(_) =>
				{
					mark_file!("SYSTEM_INFO", SYSTEM_INFO, "[错误]：\n Sudo尝试激活，但Ryb阻止了该命令启动。", '3');
					error_count += 1;
					false
				},
			}
		};

		let parameter: Vec<String> = parse_pair(Vec::with_capacity(8), &ac_command, ' ');
		let mut key: bool = true;
		let mut tmp3: String = String::new();
		let mut tmp4: Vec<char> = Vec::with_capacity(5);

		for tmp2 in &parameter[1..]
		{
			let mut tmp = tmp2.chars();
			match tmp.next()
			{
				Some('-') =>
					{
						loop
						{
							match tmp.next()
							{
								Some(t) => tmp4.push(t),
								None => break,
							}
						}
					},
				_ => match key
					{
						true => if tmp2.len() != 0
						{
							tmp3 = tmp2.to_string();
							key = false;
						},
						false =>
						{
							println!("{}：{}", parameter[0], error_message!(12));
							continue 'node_01;
						},
					}
				}
		}

		// far_way_cmd
		let far_way_cmd = || -> bool
		{
			println!("{}：命令未找到。", parameter[0]);
			true
		};

		// 下一版本可以使用find和链表
		if match parameter[0].as_str()
		{
			CMD_01 => if gen_allow_cmd.cmd1[0]
						|| only_member_bool { cd(&tmp3)					} else {far_way_cmd()},
			CMD_02 => if gen_allow_cmd.cmd2[0]
						|| only_member_bool { ls(&tmp3, tmp4)			} else {far_way_cmd()},
			CMD_03 => if gen_allow_cmd.cmd3[0]
						|| only_member_bool { cat(&tmp3)				} else {far_way_cmd()},
			CMD_04 => if gen_allow_cmd.cmd4[0]
						|| only_member_bool { cls()						} else {far_way_cmd()},
			CMD_05 => if gen_allow_cmd.cmd5[0]
						|| only_member_bool { pwd(only_member_bool)		} else {far_way_cmd()},
			CMD_06 => if gen_allow_cmd.cmd1[1]
						|| only_member_bool { whoami(only_member_bool)	} else {far_way_cmd()},
			CMD_07 => if gen_allow_cmd.cmd2[1]
						|| only_member_bool { hostname()				} else {far_way_cmd()},
			CMD_08 => if gen_allow_cmd.cmd3[1]
						|| only_member_bool { id(user_key,
												only_member_bool,
												user_permission)		} else {far_way_cmd()},
			CMD_09 => if warn_allow_cmd.cmd1[0]
						|| only_member_bool
						{
							let tmp = touch(&tmp3, &mut file_crate_list, file_crate_num);
							file_crate_num += tmp.1;
							tmp.0
						} else {far_way_cmd()},
			CMD_10 => if warn_allow_cmd.cmd2[0]
						|| only_member_bool
						{
							let tmp = mkdir(&tmp3, tmp4, &mut dirs_crate_list, dirs_crate_num);
							dirs_crate_num += tmp.1;
							tmp.0
						} else {far_way_cmd()},
			CMD_11 => if warn_allow_cmd.cmd3[0]
						|| only_member_bool { rm(&tmp3, tmp4)			} else {far_way_cmd()},
			CMD_12 => if (warn_allow_cmd.cmd4[0]
						&& user_permission)
						|| only_member_bool { wipe(&tmp3, tmp4) }
							else
							{
								println!("wipe: 权限不足。");
								true
							},
			CMD_13 => if user_key == 0 { su(&tmp3) }
							else
							{
								println!("su: 命令未找到。");
								true
							},
			CMD_14 => help(&tmp3),
			_ => far_way_cmd(),
		}
		{
			warn_count += 1;
		}
	}

	#[allow(unreachable_code)]
	{
		mark_file!("SYSTEM_INFO", SYSTEM_INFO, "【崩溃】 程序退出node_01循环。", '5');
		let local: DateTime<Local> = Local::now();
		let cur_time = local.format("%Y-%m-%d %H:%M:%S").to_string();
		let (ip, port) = get_target_ip();
		unsafe { e_mail_sender("程序退出node_01循环。", &format!("RyB于[{}]退出loop 'node_01，链接目标：{}:{}", cur_time, ip, port), 5) };
		panic!("099356");
	}
}