# learn-rust-book

- [rust book 中文版](https://kaisery.github.io/trpl-zh-cn/ch01-02-hello-world.html)
- [rust book english](https://doc.rust-lang.org/book/)

---

## 第一章：入门指南

- `rustup update`更新rust
- `rustup self uninstall`卸载rust

---

- 可以使用`cargo new`创建项目。
- 可以使用`cargo build`构建项目。
- 可以使用`cargo run`一步构建并运行项目。
- 可以使用`cargo check`在不生成二进制文件的情况下构建项目来检查错误。
- 有别于将构建结果放在与源码相同的目录，`Cargo`会将其放到`target/debug`目录。

## 第二章：写个猜数字游戏

## 第三章：常见编程概念

### 3.1 变量与可变性

- `let x = 5` : 变量默认不可变，但是可以被shadowing（同一个作用域)
- `let mut x = 10` : 可变
- `const XXX: u32 = 10` 常量，不可以被shadowing



## 第四章：认识所有权

### 4.1 什么是所有权

#### 所有权的规则
1. Rust 中的每一个值都有一个 所有者（owner）。
2. 值在任一时刻有且只有一个所有者。
3. 当所有者（变量）离开作用域，这个值将被丢弃。

#### 借用
1. 在任意给定时间，要么 只能有一个可变引用，要么 只能有多个不可变引用。
2. 引用必须总是有效的。

