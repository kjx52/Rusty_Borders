// Powershell.rs
// 包含powershell模块

// 本模块不受保护。

use std::io::{self, Write};
use std::process::Command;

pub fn powershell_act()
{
	println!("Windows StrongShell
版权所有（C） Minecraft Corporation。保留所有权利。

安装最新的 StrongShell，了解新功能和改进！https://awk.ms/PSWindows
");
	loop
	{
		print!("> ");
		let mut line: String = String::new();
		let _ = io::stdout().flush();
		let _ = io::stdin().read_line(&mut line);
		let line = line.trim();

		if line == "exit"
		{
			break;
		}

		let tmp = Command::new("PowerShell").arg("-Command").arg(line).output().unwrap();
		let tmp = String::from_utf8_lossy(&tmp.stdout);
		
		println!("{}", tmp);
	}
}