# rust-demo
rust语言初体验demo

Rustaceans：Rust社区深入活动的人群、Rust 编程码农

## Hello world
* 注释：和 js 一样，单行注释和多行注释
* 打印：println! (ln就是line的缩写)，js 中的 console，和java、python 中的打印类似
* 函数：fn 表示方法，相当于 js 中的 function

## 包管理器 cargo
和 npm 类似，安装 rust 时已经内置了
* 新建项目：cargo new xxx，和 npm init 类似，会自动生成项目目录和文件，Cargo.toml 就是配置文件，和 package.json 类似
* 编译项目：cargo build，类似 npm run build，会自动生成 target 文件，还会生成 Cargo.lock（和 package-lock.json 类似，项目自动维护），和 java 有点像
* 运行项目：./target/debug/hello_cargo，也可以 cargo run
* 代码检查：cargo check
* 生产编译：cargo build --release

## 猜数字游戏
* 接收用户输入：std::io，利用 rust 内置的标准库 io，跟 node 里的 fs、http、path 这些类似
* 申明变量：let，默认不可变，后面要改变变量的值需要加上 mut
* 花括号{}可以在字符串中插入变量，跟 js 的模板字符串 ${} 类似
* 下载第三方依赖先设置下 cargo 国内镜像源