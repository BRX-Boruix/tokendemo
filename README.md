# tokendemo

BORUIX 的验收程序，验证控制台令牌的真值能被用户态读到。

[English](README.en.md)

## 测什么

每个终端节点记录着自己的持有者，并可经文件状态查询接口读出。本程序只经公开接口读取，不碰内核
内部结构。

- 标准输入、标准输出、标准错误三个节点都必须是终端
- 三个节点都必须有持有者
- 「是终端」与「有持有者」两条值来自同一个节点、同一次查询，不可能互相矛盾

## 用法

无需参数。检查 fd 0/1/2 并打印各自的终端标志与持有者：

```
[tokendemo] my pid = 12
[tokendemo] fd0 is_terminal=1 console_owner=1
[tokendemo] fd1 is_terminal=1 console_owner=1
[tokendemo] fd2 is_terminal=1 console_owner=1
[tokendemo] PASS: stdin/stdout/stderr all owned by this pid, all terminals
```

注意：持有者未必是本进程的 pid。文件描述符表是继承的，子进程看到的持有者是父进程登记的那个。

## 退出码

- `0`——全部通过
- 非零——有失败；进程号查询失败固定为 `1`，检查未通过时数值为失败项数（失败的输出行以 `FAIL` 开头）

## 构建

```bash
cargo build --release
```

## 文件结构

```
tokendemo/
├── Cargo.toml    # 包定义
├── build.rs      # 注入链接脚本
├── linker.ld     # 用户态段布局
└── src/
    └── main.rs   # 逐项检查与结果打印
```

## 相关项目

- [`libsys`](https://github.com/BRX-Boruix/libsys) —— 文件状态查询接口
- [`consoled`](https://github.com/BRX-Boruix/consoled) —— 控制台守护进程

## 许可

MIT License，版权归 Yang Borui 所有。详见 [LICENSE](LICENSE)。
