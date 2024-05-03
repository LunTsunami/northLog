# northLog 发布信息

你可以在这里找到版本的发布信息，从1.0.0开始。

# 1.0.0 Major NorthLog

## 基本介绍
此发行版是northLog的第一个版本，包括所有最新的累积更新以及功能更新！

**NorthLog**
1.0.0 Major Update
别名：NORTHFAMILY LOG RECODE MACHINE

## 已知问题
此二进制文件并不适合所有操作系统，我们鼓励你从源码构建。

# 1.1.0 Patch NorthLog

此版本相较于上一个版本有所改进，推荐你使用。

## 修复
* 解决了 NDT-1000。  

## 新特性

### panic_console
向`main.rs`添加了`panic_console()`函数，函数原型如下：
```Rust
fn panic_console(level:u64, content: &String)
```
你可以使用这个函数进行错误处理，传入参数`level`和`content`。

content：错误处理的内容  
level  ：错误等级（1~3）

### utility.rs
此文件内提供实用功能。

### 参数
引入新的参数`def`，默认为`nolg`模式，可以自行更改。此参数为了节省键入的文本。  
引入参数`[filetype]`，用于自定义文本格式。输入如`log`，`txt`。

### 优化
优化了一些细节。