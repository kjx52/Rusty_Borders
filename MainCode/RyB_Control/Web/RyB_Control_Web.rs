// 在线控制系统 Web后端

// #=========================
// RyB记录文件路径：
const USER_COMMAND: &'static str  = r###"C:\Users\Public\History\USER_COMMAND.ryb"###;
const SYSTEM_RESULT: &'static str = r###"C:\Users\Public\History\SYSTEM_RESULT.ryb"###;
const SYSTEM_INFO: &'static str   = r###"C:\Users\Public\History\RyB_INFO.ryb"###;
const OUTPUT_PATH: &'static str   = r###"C:\Users\Public\Control\OutPut01.alcm"###;
// #=========================

extern crate iron;
extern crate router;
extern crate urlencoded;
#[macro_use] extern crate mime;

use iron::prelude::*;
use iron::status;
use router::Router;
use std::env;
use std::fs::File;
use std::io::{self, Write, BufReader, BufRead,};
use std::process::exit;
use urlencoded::UrlEncodedBody;

const HTML01: &'static str = r####"<html>
	<head>
		<meta charset="utf-8">
		<title>RyB在线控制系统</title>
		<link rel="icon" href="ryb2">
		<link rel="stylesheet" type="text/css" href="style">
		<meta http-equiv="refresh" content="3">
	</head>
	<body>
		<div id="header_pc">
			<I>
			<font size="7" color="red" style="font-family: 'AaJiJiaHei', serif;">&nbsp;Rusty_Borders 危墙</font>
			<B>
			<font size="5">在线控制系统 0.2.11</font>
			</B>
			</I>
		</div>
		<p id="p_01"><font color="#AEB6BF">
			&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;一个以 Rust_Iron 为后端写成的小型在线控制器，需配合 RyB 终端使用。本系统包含：<br>
			&nbsp;&nbsp;&nbsp;·动态控制攻击者在 RyB 终端中可以使用的命令；<br>
			&nbsp;&nbsp;&nbsp;·监测攻击者命令输出；<br>
			&nbsp;&nbsp;&nbsp;·监测攻击者命令输入；<br>
			&nbsp;&nbsp;&nbsp;·监测RyB进程；<br>
			<br>
			&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;有任何意见或建议请联系作者：
			<B><U>< kjx52@outlook.com ></U></B><br>
			&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;很多页面都在开发中，并且我非常需要一位美工师傅来帮我重新设计一下这些个该死的显示框。
		</font></p>
		<br>
		<div id="div_01">
			<fieldset id="Fieldset_01">
				<legend>用户输入：</legend>
				<fieldset id="cmd_area_01">
					"####;
const HTML02: &'static str = r####"
					> <span class="blink">_</span>
				</fieldset>
			</fieldset>
		</div>
		<div id="div_03">
			<fieldset id="Fieldset_01">
				<legend>系统输出：</legend>
				<fieldset id="cmd_area_03">
					"####;
const HTML03: &'static str = r####"
					> <span class="blink">_</span>
				</fieldset>
			</fieldset>
		</div>
		<br>
		<form name="form1" action="/form" method="post">
			<fieldset id="Fieldset_01">
				<legend>RyB 已包含命令【多选】：</legend>
				<fieldset id="Fieldset_02">
					<p>
						&nbsp;&nbsp;&nbsp;<input type="checkbox", value="g0", name="gen" onclick="clicksubmit(this)" id="1">cd
						&nbsp;&nbsp;&nbsp;<input type="checkbox", value="g1", name="gen" onclick="clicksubmit(this)" id="2">ls
						&nbsp;&nbsp;&nbsp;<input type="checkbox", value="g2", name="gen" onclick="clicksubmit(this)" id="3">cat
						&nbsp;&nbsp;&nbsp;<input type="checkbox", value="g3", name="gen" onclick="clicksubmit(this)" id="4">cls
						&nbsp;&nbsp;&nbsp;<input type="checkbox", value="g4", name="gen" onclick="clicksubmit(this)" id="5">pwd
						&nbsp;&nbsp;&nbsp;<input type="checkbox", value="g5", name="gen" onclick="clicksubmit(this)" id="6">whoami
						&nbsp;&nbsp;&nbsp;<input type="checkbox", value="g6", name="gen" onclick="clicksubmit(this)" id="7">hostname
						<br>
						&nbsp;&nbsp;&nbsp;<input type="checkbox", value="g7", name="gen" onclick="clicksubmit(this)" id="8">id
					</p>
				</fieldset>
				<br>
				<fieldset id="Fieldset_02">
					<p>
						&nbsp;以下命令需与磁盘交互，请斟酌使用。
						<br>
						&nbsp;&nbsp;&nbsp;<input type="checkbox", value="w0", name="warn" onclick="clicksubmit(this)" id="9">touch
						&nbsp;&nbsp;&nbsp;<input type="checkbox", value="w1", name="warn" onclick="clicksubmit(this)" id="10">mkdir
						&nbsp;&nbsp;&nbsp;<input type="checkbox", value="w2", name="warn" onclick="clicksubmit(this)" id="11">rm
						&nbsp;&nbsp;&nbsp;<input type="checkbox", value="w3", name="warn" onclick="clicksubmit(this)" id="12">wipe
					</p>
				</fieldset>
			</fieldset>
			<p>
				"####;
const HTML04: &'static str = r####"
			</p>
			<!--
			<br><br><br>
			<button type="submit" class="btn_pc">修改</button>
			-->
			<script>
				if(typeof(Storage)!=="undefined")
				{
					/*
					console.table({
						"checkbox01":localStorage.getItem("1"),
						"checkbox02":localStorage.getItem("2"),
						"checkbox03":localStorage.getItem("3"),
						"checkbox04":localStorage.getItem("4"),
						"checkbox05":localStorage.getItem("5"),
					});
					*/
					for(var i=1; i<13; i++)
					{
						if(localStorage.getItem(i) == 'true')
						{
							document.getElementById(i).checked = true;
						}
						else if(localStorage.getItem(i) == null)
						{
							if(i<6)
							{
								localStorage.setItem(i, true);
								document.getElementById(i).checked = true;
							}
							else
							{
								localStorage.setItem(i, false);
								document.getElementById(i).checked = false;
							}
						}
						else
						{
							document.getElementById(i).checked = false;
						}
					}
					/*
					console.table({
						"checkbox01.checked":document.getElementById("1").checked,
						"checkbox02.checked":document.getElementById("2").checked,
						"checkbox03.checked":document.getElementById("3").checked,
						"checkbox04.checked":document.getElementById("4").checked,
						"checkbox05.checked":document.getElementById("5").checked,
					});
					*/
				}
				else
				{
					document.getElementById("checkbox").innerHTML="对不起，您的浏览器不支持 web 存储。";
				}
				
				function clicksubmit(checkbox)
				{
					if(checkbox.checked == true)
					{
						localStorage.setItem(checkbox.id, true);
					}
					else
					{
						localStorage.setItem(checkbox.id, false);
					}
					setTimeout("document.form1.submit()", 100);
				}
			</script>
		</form>
		<br>
		<div id="div_02">
			<fieldset id="Fieldset_01">
				<legend>RyB信息：</legend>
				<fieldset id="cmd_area_02">
					"####;
const HTML05: &'static str = r####"
					> <span class="blink">_</span>
				</fieldset>
			</fieldset>
		</div>
		<br>
		<br>
		<br><br><br><br>
		<div id="footer_pc">
			Copyright © 2022-2026
			<font style="font-family: 'a_AlgeriusCmFtz1', serif;">Двигатель Ржавчина</font>
			&nbsp;
			<strong>10.129.136.9</strong> 
			All Rights Reserved. 备案号：陕ICP备21049801号-1
			<br>
			Power By <B><I>Jessarin000</I></B>
		</div>
	</body>
</html>"####;

fn main()
{
	let mut router = Router::new();
	let ip: Vec<String> = env::args().collect();

	if ip.len() == 1
	{
		writeln!(std::io::stderr(), "用法： RyB_Control_Web <服务器 IP 地址> ...").unwrap();
		exit(1);
	}

	if ip.len() > 2
	{
		writeln!(std::io::stderr(), "[\x1b[31m-\x1b[0m] 错误：检测到多个 IP 地址。").unwrap();
		exit(2);
	}

	let mut current_ip: String = ip[1].to_string();
	current_ip.push_str(":3000");
	
	router.get("/", get_pic, "root");
	router.get("/ryb", get_png, "ryb");
	router.get("/ryb2", get_ico, "ryb2");
	router.get("/style", get_style, "style");

	router.get("/form", get_form, "form");
	router.post("/form", post_form, "form2");
	
	Iron::new(router).http(current_ip).expect("关键操作失败02");
}

fn get_sys_data(tmp: &'static str, tmp1: usize) -> String
{
	let mut lines: Vec<String> = BufReader::new(
			File::open(tmp)
			.expect("关键操作失败03"))
		.lines()
		.filter_map(|line| line.ok())
		.collect::<Vec<String>>();
	let count = lines.len();
	if count > tmp1
	{
		lines = lines[count - tmp1 ..].to_vec();
	}
	
	lines.into_iter().map(|tmp|
		{
			if tmp.len() == 0
			{
				"".to_string()
			}
			else
			{
				tmp + "\n<br>"
			}
		}).collect::<String>()
}

fn get_pic(_request: &mut Request) -> IronResult<Response>
{
	let mut response = Response::new();
	let pic = std::fs::File::open(r###"C:\Users\Public\SSHShell\Web_Control\pic.html"###).unwrap();

	response.set_mut(status::Ok);
	response.set_mut(mime!(Text/Html; Charset=Utf8));
	response.set_mut(pic);

	Ok(response)
}

fn get_form(_request: &mut Request) -> IronResult<Response>
{
	let mut response = Response::new();
	response.set_mut(status::Ok);
	response.set_mut(mime!(Text/Html; Charset=Utf8));
	response.set_mut(HTML01.to_string()
				+ &get_sys_data(USER_COMMAND, 9)
				+ HTML02
				+ &get_sys_data(SYSTEM_RESULT, 9)
				+ HTML03
				+ HTML04
				+ &get_sys_data(SYSTEM_INFO, 14)
				+ HTML05);

	Ok(response)
}

fn get_png(_request: &mut Request) -> IronResult<Response>
{
	let mut response = Response::new();
	let _img = std::fs::File::open(r###"C:\Users\Public\SSHShell\Web_Control\Rost.txt"###).unwrap();

	response.set_mut(status::Ok);
	response.set_mut(mime!(Image/Png; Charset=Utf8));
	response.set_mut(_img);

	Ok(response)
}

fn get_ico(_request: &mut Request) -> IronResult<Response>
{
	let mut response = Response::new();
	let _img = std::fs::File::open(r###"C:\Users\Public\SSHShell\Web_Control\rust.ico"###).unwrap();

	response.set_mut(status::Ok);
	response.set_mut(mime!(Image/Png; Charset=Utf8));
	response.set_mut(_img);

	Ok(response)
}

fn get_style(_request: &mut Request) -> IronResult<Response>
{
	let mut response = Response::new();
	let file = std::fs::File::open(r###"C:\Users\Public\SSHShell\Web_Control\style.css"###).unwrap();

	response.set_mut(status::Ok);
	response.set_mut(mime!(Text/Css; Charset=Utf8));
	response.set_mut(file);

	Ok(response)
}

fn post_form(request: &mut Request) -> IronResult<Response>
{
	let response_set_mut = |tmp: String| -> Response
	{
		let mut res = Response::new();
		res.set_mut(status::BadRequest);
		res.set_mut(tmp);
		res
	};

	let mut response = Response::new();
	
	let form_data = match request.get_ref::<UrlEncodedBody>()
	{
		Err(_e) => {
			println!("[\x1b[32m+\x1b[0m] 限制所有指令。");
			let _ = io::stdout().flush();
			response.set_mut(status::Ok);
			response.set_mut(mime!(Text/Html; Charset=Utf8));
			response.set_mut(HTML01.to_string()
				+ &get_sys_data(USER_COMMAND, 9)
				+ HTML02
				+ &get_sys_data(SYSTEM_RESULT, 9)
				+ HTML03
				+ &format!("完成。限制所有指令。")
				+ HTML04
				+ &get_sys_data(SYSTEM_INFO, 14)
				+ HTML05);
			return Ok(response);
		}
		Ok(map) => map
	};

	println!("\n[\x1b[32m+\x1b[0m]收到的原始数据：{:?}", form_data);
	let _ = io::stdout().flush();

	let none_vec = vec![];
	let gen_value = match form_data.get("gen")
	{
		None => &none_vec,
		Some(t) => t,
	};

	let warn_value = match form_data.get("warn")
	{
		None => &none_vec,
		Some(t) => t,
	};

	if gen_value.len() == 0
	&& warn_value.len() == 0
	{
		return Ok(
			response_set_mut(
				format!("Form data has no 'gen' or 'warn' parament!\n")));
	}

	println!("  接收到的ALCM常规命令：{:?}", gen_value);
	println!("  接收到的ALCM警告命令：{:?}", warn_value);
	let _ = io::stdout().flush();

	let mut output = File::create(OUTPUT_PATH).unwrap();
	let _ = write!(output, "{}\n{}",
		gen_value.iter().map(|tmp|{tmp.to_string()}).collect::<String>(),
		warn_value.iter().map(|tmp|{tmp.to_string()}).collect::<String>());

	response.set_mut(status::Ok);
	response.set_mut(mime!(Text/Html; Charset=Utf8));
	response.set_mut(HTML01.to_string()
				+ &get_sys_data(USER_COMMAND, 9)
				+ HTML02
				+ &get_sys_data(SYSTEM_RESULT, 9)
				+ HTML03
				+ &format!("完成。接收到的启动信号为： <b>\n<br>{:?}\n<br>{:?}</b>",
					gen_value,
					warn_value)
				+ HTML04
				+ &get_sys_data(SYSTEM_INFO, 14)
				+ HTML05);

	Ok(response)
}
