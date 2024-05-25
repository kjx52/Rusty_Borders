// 在线控制系统 总体调用

// #=========================
// RyB记录文件路径：
const USER_COMMAND: &'static str  = r###"C:\Users\Public\History\USER_COMMAND.ryb"###;
const SYSTEM_RESULT: &'static str = r###"C:\Users\Public\History\SYSTEM_RESULT.ryb"###;
const SYSTEM_INFO: &'static str   = r###"C:\Users\Public\History\RyB_INFO.ryb"###;
const OUTPUT_PATH: &'static str   = r###"C:\Users\Public\Control\OutPut01.alcm"###;
// 版本：
const VERSION: &'static str = "0.2.11";
// #=========================

use chrono::prelude::*;
use std::env;
use std::fs::OpenOptions;
use std::io::{self, Read, Write};
use std::process::{exit, Command,};

fn list()
{
	println!("");
	println!("Rust_Borders 危墙 2024 \x1b[1;30mv0.3.9\x1b[0m");
	println!("在线控制系统 \x1b[1;30mv0.2.11\x1b[0m");
	println!("  目前可用命令包括：");
	println!("  ·help  显示此命令列表。");
	println!("  ·on    启动SSH服务（得off过后才会有用）");
	println!("  ·off   关闭SSHD服务，但不关闭项目。");
	println!("  ·Z     正常退出蜜罐。");
	println!("  ·kill  立即终止RyB终端部分并关闭SSH服务与链接。");
	println!("");
	println!("*下一版本（如果有）会加入终端状态和终端调试命令。");
	println!("或将加入向终端传口信的功能（￥:一个善意的声音？23333）");
	println!("");
}

fn on(ssh_key: bool) -> bool
{
	if ssh_key
	{
		println!("[\x1b[32m+\x1b[0m] 检测到SSHD服务已开启。");
		return true;
	}

	print!("[\x1b[32m+\x1b[0m] 正在开启SSHD服务...");
	match Command::new("PowerShell")
		.arg("-Command")
		.arg("net start sshd 1>$null 2>&1")
		.output()
	{
		Ok(_) => println!("完成。"),
		Err(e) =>
		{
			writeln!(std::io::stderr(), "\n[\x1b[31m-\x1b[0m] 错误：SSHD启动失败：{:?}。", e).unwrap();
			over(5);
		}
	}

	true
}

fn off(ssh_key: bool) -> bool
{
	if ! ssh_key
	{
		println!("[\x1b[32m+\x1b[0m] 检测到SSHD服务已关闭。");
		return false;
	}

	print!("[\x1b[32m+\x1b[0m] 正在关闭SSHD服务...");
	match Command::new("PowerShell")
		.arg("-Command")
		.arg("net stop sshd 1>$null")
		.output()
	{
		Ok(_) => println!("完成。"),
		Err(e) =>
		{
			writeln!(std::io::stderr(), "\n[\x1b[31m-\x1b[0m] 错误：SSHD关闭失败：{:?}。", e).unwrap();
			over(5);
		}
	}

	false
}

fn kill(ssh_key: bool) -> bool
{
	if ! ssh_key
	{
		println!("[\x1b[32m+\x1b[0m] SSHD服务未开启，不存在远程链接。");
		return false;
	}
	print!("[\x1b[32m+\x1b[0m] 正在终止RyB终端...");
	let _ = Command::new("PowerShell")
		.arg("-Command")
		.arg("Get-Process -Name \"Rusty_Borders_危墙\" | Stop-Process -Force")
		.output();
	println!("完成。");
	off(ssh_key)
}

fn lg_time() -> String
{
	let local: DateTime<Local> = Local::now();
	local.format("%Y-%m-%d %H:%M:%S").to_string()
}

fn sh_time() -> String
{
	let local: DateTime<Local> = Local::now();
	local.format("%H:%M:%S").to_string()
}

fn over(num: i32) -> !
{
	println!("\n[{}] 已终止Rusty Borders在线控制系统。", lg_time());
	println!("感谢使用，键入ENTER退出程序。\n==================================");
	let mut buffer = [0_u8; 1];
	let _ = io::stdout().flush();
	let _ = io::stdin().read(&mut buffer);
	exit(num)
}




fn main()
{
	println!("[{}] 已启动Rusty Borders在线控制系统。启动版本：{}\n\n", lg_time(), VERSION);
	println!("");
	println!("\x1b[31m                             |                                  \x1b[34m  . _ .   \x1b[0m");
	println!("\x1b[31m                           '~|~.                                \x1b[34m    :  :  \x1b[0m");
	println!("\x1b[31m                        __   .   __              __   .   __    \x1b[34m   : '    \x1b[0m");
	println!("\x1b[31m                      .'  '-'| .'  .           .'  ':' .:'._ ': \x1b[34m.         \x1b[0m");
	println!("\x1b[31m                     ' .-._.'.'.'; |          ' .-.' .':    ; ;           \x1b[0m");
	println!("\x1b[31m                     |/  .'-: |  | |          |/  .: : ;    | :           \x1b[0m");
	println!("\x1b[31m                        : | ; | . :    \x1b[34m      .\x1b[31m   : ; | ; | :.'            \x1b[0m");
	println!("\x1b[31m                       .; | ; |. :.    \x1b[34m.      \x1b[31m   | ; | ; |'`;.            \x1b[0m");
	println!("\x1b[31m                     .: ; | ; |'.  :             | ; | ; |  | '           \x1b[0m");
	println!("\x1b[31m                       .; | ; |  : |  .:.   _    | ; | ; |'`|  ;          \x1b[0m");
	println!("\x1b[31m                     .: ; | | :  | | ' |':.'. '; | ; | : |._|  ;          \x1b[0m");
	println!("\x1b[34m                  . \x1b[31m    | :-.'   : '.. | ;  |  ; | : |'  |  |  ;          \x1b[0m");
	println!("\x1b[34m                :   \x1b[31m   -.'_.--._  : /  | ;  :.'  |';'__  |  |  :          \x1b[0m");
	println!("\x1b[34m             : :    \x1b[31m   .:;.:_   '. '   | . '    .:;..  '':..:'`           \x1b[0m");
	println!("\x1b[34m           :  :     \x1b[31m .''     ':/'     : '     .''    `': :'               \x1b[0m");
	println!("\x1b[34m          :   :     \x1b[31m         |       :   _    `:'       '                 \x1b[0m");
	println!("\x1b[34m          :   _.    \x1b[31m         |        `''`                                \x1b[0m");
	println!("\x1b[34m           :-'      \x1b[31m         |                                            \x1b[0m");
	println!("");
	println!("\x1b[1;30m  ----     -      ---  -----  -   -       ----    \x1b[0m\x1b[37m===   ====    ====  =\x1b[1m###     ###\x1b[0m");
	println!("\x1b[1;30m  -   -  -   -  -        -     - -        \x1b[0m\x1b[37m=  =   =   =  =   =  \x1b[1m#      #   #  #    \x1b[0m");
	println!("\x1b[1;30m  ----   -----   ---     -      \x1b[0m\x1b[37m=         ===    =   =  \x1b[1m#   #   ###   ####    ### \x1b[0m");
	println!("\x1b[1;30m  -  -   -   -      -    \x1b[0m\x1b[37m=      =         =   =  \x1b[1m#   #  #   #  #      #  #       #\x1b[0m");
	println!("\x1b[1;30m  -   -  -   -  \x1b[0m\x1b[37m===      =      =         \x1b[1m#####   ###   ####    ####  #   #  ###  \x1b[0m");
	println!("");
	println!("\x1b[35m                               [ Rust_Borders 危墙 ]\x1b[0m");
	println!("\x1b[35m                                   [版本：0.3.9]\x1b[0m");
	println!("");
	println!("                                  \x1b[1;30mJessarin000 作\x1b[0m");
	println!("\n\n");

	OpenOptions::new().append(true).create(true).open(USER_COMMAND).expect("CRITICAL: 关键操作失败01");
	OpenOptions::new().append(true).create(true).open(SYSTEM_RESULT).expect("CRITICAL: 关键操作失败01");
	OpenOptions::new().append(true).create(true).open(SYSTEM_INFO).expect("CRITICAL: 关键操作失败01");
	OpenOptions::new().append(true).create(true).open(OUTPUT_PATH).expect("CRITICAL: 关键操作失败01");

	let ip: Vec<String> = env::args().collect();

	if ip.len() == 1
	{
		writeln!(std::io::stderr(), "用法： RyB_Control <服务器 IP 地址> ...").unwrap();
		over(1);
	}

	if ip.len() > 2
	{
		writeln!(std::io::stderr(), "[\x1b[31m-\x1b[0m] 错误：检测到多个 IP 地址。").unwrap();
		over(2);
	}

	println!("[\x1b[1;30m{}\x1b[0m] 启动初始化。", sh_time());

	match Command::new("PowerShell")
		.arg("-Command")
		.arg("sc.exe query state= all |findstr SERVICE_NAME |findstr sshd")
		.output()
	{
		Ok(t) =>
		{
			if t.stdout.len() == 0
			{
				writeln!(std::io::stderr(), "[\x1b[31m-\x1b[0m] 错误：未检测到SSHD服务。请检查服务状态后再启动。").unwrap();
				over(4);
			}
			if let Ok(r) = Command::new("PowerShell")
				.arg("-Command")
				.arg(r###"sshd -v 2>&1 |findstr "OpenSSH_for_Windows" |findstr -v "findstr""###)
				.output()
			{
				println!("[\x1b[32m+\x1b[0m] 检测到的SSHD服务版本：{}。", String::from_utf8_lossy(&r.stdout).trim());
			}
			else
			{
				writeln!(std::io::stderr(), "[\x1b[33m!\x1b[0m] 警告：无法获取SSHD版本。").unwrap();
			}
		}
		Err(e) =>
		{
			writeln!(std::io::stderr(), "[\x1b[31m-\x1b[0m] 错误：关键服务校验失败：{:?}。", e).unwrap();
			over(3);
		}
	}
	println!("[\x1b[32m+\x1b[0m] SSHD服务状态校验正常。");

	match Command::new("PowerShell")
		.arg("-Command")
		.arg("net start |findstr SSH")
		.output()
	{
		Ok(t) =>
		{
			if t.stdout.len() == 0
			{
				let _ = on(false);
			}
			else
			{
				println!("[\x1b[32m+\x1b[0m] 检测到SSHD服务已开启。");
			}
		}
		Err(e) =>
		{
			writeln!(std::io::stderr(), "[\x1b[31m-\x1b[0m] 错误：关键服务校验失败：{:?}。", e).unwrap();
			over(3);
		}
	}

	let mut ssh_key: bool = true;
	let current_ip: String = ip[1].to_string();
	
	print!("[\x1b[1;30m{}\x1b[0m] 在线控制系统开启于 http://{}:3000 ...", sh_time(), current_ip);
	let _ = io::stdout().flush();
	let mut child = Command::new(r###"C:\Users\Public\SSHShell\Web_Control\web_01.exe"###)
				.arg(current_ip)
				.spawn()
				.expect("子线程开启失败01");
	println!("完成。");

	println!("[\x1b[1;30m{}\x1b[0m] 开始监听终端指令。", sh_time());
	println!(" 可以使用“\x1b[1;32mhelp\x1b[0m”命令来显示当前所有可用命令。");
	let _ = io::stdout().flush();
	loop
	{
		print!("\x1b[1;34m$\x1b[0m ");
		let mut tmp1 = String::new();
		let _ = io::stdout().flush();
		let _ = io::stdin().read_line(&mut tmp1);

		match tmp1.trim()
		{
			""		=> continue,
			"help"	=> list(),
			"on"	=> ssh_key = on(ssh_key),
			"off"	=> ssh_key = off(ssh_key),
			"kill"	=> ssh_key = kill(ssh_key),
			"Z"		=> break,
			_		=> println!("[\x1b[33m!\x1b[0m] 警告：命令未找到。"),
		}

		if let Some(state) = child.try_wait().unwrap()
		{
			if state.success()
			{
				println!("[\x1b[31m-\x1b[0m] 错误：Web子线程提前终结。");
			}
			else
			{
				println!("[\x1b[31m-\x1b[0m] 错误：Web子线程诧异。");
			}
			over(6);
		}
	}
	if ssh_key
	{
		let _ = off(true);
		let _ = io::stdout().flush();
	}
	child.kill().expect("无法终结子线程01（Iron_gcd）");

	over(0)
}