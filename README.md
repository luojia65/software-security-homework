# software-security-homework

## 使用方法

比较1a.c和1b.c两个文件，使用词法分析和LCS算法：

```
cargo run -- r2 files-1/1a.c files-1/1b.c
```

输出应该是`重复率：96.875%`。

使用CFG图比较2a.c和2b.c文件：

```
cargo run -- r3 files-1/2a.c files-1/2b.c
```

栈的溢出：

```
cargo run -- r4 files-1/4-1.c
```

格式化字符串：

```
cargo run -- r5 files-1/5.c
```

同源性检测，比对不同语言的文件：

```
cargo run -- a1 files-1/2a.c files-1/2c.rs
```

整数长度溢出：

```
cargo run -- b2 files-1/b2-1.c
```

整数运算溢出：

```
cargo run -- b3 files-1/b3-1.c
```

整数符号溢出：

```
cargo run -- b4 files-1/b4-1.c
```

空指针：

```
cargo run -- b5 files-1/b5-1.c
```

## 版权声明

源码中部分测试样例由开源项目中采集而来，这些项目包括qemu和chrome。这些测试样例遵守开源项目原有的开源协议。
