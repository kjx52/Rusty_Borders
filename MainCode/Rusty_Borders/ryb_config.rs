// ryb_config.rs
// 本文件包含Rusty Borders所需的配置参数。

//############################################
// 以下配置应根据实际情况进行修改

// 蜜罐用户的主目录：
pub const HOME_PATH_01: &'static str = r###"C:\Users\kali\Shell\home\kali"###;
pub const HOME_PATH_02: &'static str = r###"home\kali"###;
pub const HOME_PATH_03: &'static str = r###"C:\Users\kali\Shell\\\\"###;
pub const HOME_PATH_04: &'static str = r###"c:\Users\kali\Shell\\\\"###;
// 用户信息记录：
pub const USER_COMMAND: &'static str  = r###"C:\Users\Public\History\USER_COMMAND.ryb"###;
pub const SYSTEM_RESULT: &'static str = r###"C:\Users\Public\History\SYSTEM_RESULT.ryb"###;
pub const SYSTEM_INFO: &'static str   = r###"C:\Users\Public\History\RyB_INFO.ryb"###;
// OS_Shell组：
pub const KEY_CODE_01: &'static str = "powershell";
pub const KEY_CODE_02: &'static str = "cmd";
pub const KEY_CODE_03: &'static str = "exe";
pub const KEY_CODE_04: &'static str = "runas";
// 控制类文件：
pub const LICENSE: &'static str			 = r###"C:\Users\Public\Control\License.txt"###;
pub const CMD_CONTROL_FILE: &'static str = r###"C:\Users\Public\Control\OutPut01.alcm"###;
pub const SSH_LOG_FILE: &'static str     = r###"C:\ProgramData\ssh\logs\sshd.log"###;
// 邮箱配置
pub const ECFG: &'static str = r###"C:\Users\Public\Control\Email_Config.cfg"###;
pub struct EmailInfo
{
	pub tago: String,
	pub sosr: String,
	pub smtp: String,
	pub pawd: String,
}
// 命令模块列表：
pub const CMD_01: &'static str = "cd";
pub const CMD_02: &'static str = "ls";
pub const CMD_03: &'static str = "cat";
pub const CMD_04: &'static str = "cls";
pub const CMD_05: &'static str = "pwd";
pub const CMD_06: &'static str = "whoami";
pub const CMD_07: &'static str = "hostname";
pub const CMD_08: &'static str = "id";
pub const CMD_09: &'static str = "touch";
pub const CMD_10: &'static str = "mkdir";
pub const CMD_11: &'static str = "rm";
pub const CMD_12: &'static str = "wipe";
pub const CMD_13: &'static str = "su";
pub const CMD_14: &'static str = "help";
//############################################