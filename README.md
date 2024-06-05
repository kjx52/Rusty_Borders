# Rusty_Borders 2024 《 *危墙* 》项目简介
### 由 Rust 语言编写的新一代 Windows SSH 蜜罐 

本项目为原 SSH 蜜罐项目 Windows 版本的 Rust 衍生版，  
SSH_Honeypot For Windows 详见 < https://github.com/kjx52/ssh-honeypot-new >。  

> 本项目开发环境为 Windows 11 (22631)家庭版，测试环境为 Windows IIS 7.0、Windows 10、Windows 11。  
> 本项目开发使用 1.78.0 版本的Rust编译器。

“危墙_Setup.exe”安装程序安装目录为 Desktop/Rusty_Borders ，运行Setup程序后请运行RyB_Deployment_Program.exe并完成后续安装。  
~~*// 下个版本（如果有）我会使用config文件代替硬编码来提高其灵活性。*~~  
考虑到大多数同志电脑上都没有Rust环境，所以我这个版本就将Config做了出来。  
默认安装目录："C:\Users\Public\Control\Email_Config.cfg"，这个cfg文件用于配置邮件发送。  

__###############################__  
__# 注意，不要在“=”前后加空格! #__  
__###############################__

本项目为开源特种类项目，作者尽力确保其普遍的适用性，但用户应根据开源源代码针对特定情形加以修改。
同样，作者尽力确保其安全性，但因水平有限，若是百密之中有所纰漏，还望各位告知本人。

作者联系方式： < <u>kjx52@outlook.com</u> >。

***更多信息详见先知社区：《 [新一代 Windows SSH 蜜罐-Rusty_Borders 2024 危墙__v 0.3.9](https://xz.aliyun.com/t/14752) 》***

*注意：本项目目前尚不兼容 SSH-Honeypot For Windows 中的 WinBacsh 终端。下一版本作者将根据反馈决定是否合并两项目。  

***

#### 本项目的亮点有：
* 在策略上，本项目继承了SSH_HoneyPot_for_Windows的思路：
  + 以第三方伪终端替换原 SSH 终端 CMD ，用于模拟 Linux 下的 Chroot 隔离环境，从而限制攻击者的路径变迁以及代码执行。
  + 详见< https://xz.aliyun.com/t/12928 >。
* 在实现上，本项目
  + 跳开了其祖先所使用的内置OS_Shell执行用户命令，取消了守护进程，转而完全依靠Windows API实现，极大地提升了性能和项目安全性。
  + 使用了在线控制系统，可以实现远程监测和控制Allow_Command。  
    注：考虑到网站开放端口的隐患，关键命令均由许可证控制。
  + 总体调用系统RyB_Control程序实现了使用多种命令操控RyB终端以及SSH服务。
  + 内置了邮件发送功能，可以在异常时提醒使用者。
  + 模拟了Linux终端样式和13个常用的Shell命令。
* Rust语言本身的高性能和内存安全性。

本项目使用了自定义许可证，这种特色功能的意义仅在于标明用户身份以及控制部分命令的使用，对项目本身无任何约束。  
许可证规则写于License_key.txt文件。  
默认许可记录于License.txt文件中，但用户应根据环境自行配置许可证。  

***

#### 本项目遵循GPL v2.0协议。  
本项目允许使用者修改、移植并再发布源码，但“当你发布它的时候，请确保你的项目使用者享有你曾经拥有的所有权力”。  
让我们看到你的创意:D。  
  
本项目或将推出 Linux 版本。  

***

#### 后记
这次的项目花了两个月时间完成。

本项目的祖先，也就是 SSH-Honeypot For Windows 在上一次 PWK 中被攻破了，漏洞点正是在内存方面（详情咨询PWN爷），  
当错误的地址被植入后，对面顺理成章地获得了反向Shell，这也让我产生了使用内存安全地编程语言复写这个项目的想法。

希望这个项目能对你有用:)
<br />
<br />
<br />
  
Jessarin000  
2024-05-25 作
