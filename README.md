# tokendemo

**简体中文** | [English](#english)

BORUIX 的**控制台令牌真值验收**程序——验证"这个终端归谁"这个信息，用户态程序**真的能读到**。

```
[tokendemo] my pid = 7
[tokendemo] fd0 is_terminal=1 console_owner=7
[tokendemo] fd1 is_terminal=1 console_owner=7
[tokendemo] fd2 is_terminal=1 console_owner=7
[tokendemo] PASS: stdin/stdout/stderr all owned by this pid, all terminals
```

---

## 它验证什么

系统里每个控制台有一个**所有者**——记录"这个终端当前归哪个进程"。这个信息由内核维护。

但内核里"有"这个信息，和用户态程序"能读到"它是两件事。这个程序验证的是**后者**：它只通过
公开的接口去查，不依赖任何内核内部结构，然后检查读回来的结果是否合理。

这是一条**真实数据链路**的证据：从用户态程序出发，经公开接口，一直走到内核里那笔记录。

## 三条不变式

程序检查三件事：

| 不变式 | 含义 |
| --- | --- |
| **有主** | 标准输入、输出、错误三个流都有明确的所有者，不是"无主"状态 |
| **是终端** | 这三个流确实都是终端 |
| **真值一致** | 上面两条信息来自**同一个节点**、经**同一次**查询取回 |

第三条是设计上的要点：如果"是不是终端"和"归谁"来自两次独立的查询，它们就有可能互相矛盾
——比如一个流被告知是终端、却查不到所有者。**同一次查询同时返回两者，矛盾就无从产生。**

## 一个被推翻的初始假设

这个程序最初打算断言"所有者就是我自己"。**这个假设是错的**，值得记下来。

原因是：令牌属于**控制台对象**，而文件描述符表是**继承**的。一个进程从父进程那里继承了标准
输入输出，它看到的所有者自然是**父进程**，而不是自己。

所以真正该断言的不是"所有者等于我"，而是——**所有者非零，且是一个真实的进程标识**。

这个区别很重要：如果按最初的假设去写，程序会在任何继承了终端的正常场景下**误报失败**。

## 它不验证什么

这个程序**只读**所有者，**不测试移交**——"谁该持有令牌、什么时候该移交"属于用户态策略，
不由内核提供移交接口。

所以它证明的是"所有者信息可被用户态如实读到"，**不**证明"令牌移交机制可用"。这是如实划出
的边界，不是遗漏。

## 失败时的行为

任何一项检查失败都会打印具体是哪个描述符、哪一项不符，并返回失败项数量作为退出码。

还有一条纪律：**如果连自己的进程标识都取不到，程序如实报错退出，绝不编造一个假的标识让断言
"通过"**。验收程序如果会自欺，它就没有价值。

## 输出

逐个描述符打印实际读到的值，最后给出结论：

- 通过 —— 打印 `PASS`，退出码 0
- 失败 —— 打印 `FAILED` 与失败项数量，退出码为失败项数

## 构建

```bash
cargo build --release
```

编译产物部署为 BORUIX 系统中的用户态程序。

## 文件结构

```
tokendemo/
├── Cargo.toml    # 包定义
├── build.rs      # 注入链接脚本
├── linker.ld     # 用户态段布局
└── src/
    └── main.rs   # 三条不变式的检查
```

## 相关项目

- [`libsys`](https://github.com/BRX-Boruix/libsys) —— 提供文件属性查询接口
- [`consoled`](https://github.com/BRX-Boruix/consoled) —— 控制台字节流服务
- [`selftest`](https://github.com/BRX-Boruix/selftest) —— 系统自检

## 许可

MIT License，版权归 Yang Borui 所有。详见 [LICENSE](LICENSE)。

---

# English

[简体中文](#tokendemo) | **English**

A **console token truth acceptance** program for BORUIX — it verifies that "who owns this terminal"
is information a user-space program **can actually read**.

```
[tokendemo] my pid = 7
[tokendemo] fd0 is_terminal=1 console_owner=7
[tokendemo] fd1 is_terminal=1 console_owner=7
[tokendemo] fd2 is_terminal=1 console_owner=7
[tokendemo] PASS: stdin/stdout/stderr all owned by this pid, all terminals
```

---

## What it verifies

Each console in the system has an **owner** — a record of which process currently holds that
terminal. The kernel maintains this information.

But the kernel *having* the information and a user-space program *being able to read* it are two
different things. This program verifies the latter: it queries only through public interfaces,
depends on no kernel internals, and checks that what comes back is sound.

It is evidence of a **real data chain**: from a user-space program, through the public interface, all
the way to that record inside the kernel.

## Three invariants

The program checks three things:

| Invariant | Meaning |
| --- | --- |
| **Owned** | Standard input, output, and error all have a definite owner, not an "unowned" state |
| **Terminals** | Those three streams really are terminals |
| **Consistent truth** | Both facts come from **the same node** in **the same query** |

The third is the design point: were "is it a terminal" and "who owns it" to come from two separate
queries, they could contradict each other — a stream reported as a terminal yet with no findable
owner. **One query returning both leaves no room for contradiction.**

## An initial assumption that was overturned

The program originally intended to assert "the owner is myself". **That assumption was wrong**, and is
worth recording.

The reason: the token belongs to the **console object**, while the file descriptor table is
**inherited**. A process that inherits standard input and output from its parent naturally sees the
**parent** as the owner, not itself.

So the right assertion is not "the owner equals me" but — **the owner is non-zero and is a real
process identifier**.

The distinction matters: written against the original assumption, the program would **report failure
falsely** in any normal case where a terminal is inherited.

## What it does not verify

This program only **reads** the owner; it does **not** test handover — deciding who should hold the
token and when to hand it on is user-space policy, and the kernel offers no handover interface.

It therefore shows that "owner information can be read honestly from user space", and does **not**
show that "token handover works". That boundary is drawn deliberately, not left out.

## On failure

Any failed check prints which descriptor and which item did not hold, and returns the number of
failures as the exit code.

One further discipline: **if the program cannot even obtain its own process identifier, it reports an
error and exits rather than inventing a fake identifier so the assertion "passes"**. An acceptance
program that can fool itself is worthless.

## Output

It prints the values it actually read for each descriptor, then a verdict:

- Passing — prints `PASS`, exit code 0
- Failing — prints `FAILED` and the number of failures, exit code equal to that count

## Building

```bash
cargo build --release
```

The artifact is deployed as a user-space program in a BORUIX system.

## Layout

```
tokendemo/
├── Cargo.toml    # package definition
├── build.rs      # injects the linker script
├── linker.ld     # user-space section layout
└── src/
    └── main.rs   # the three invariant checks
```

## Related projects

- [`libsys`](https://github.com/BRX-Boruix/libsys) — provides the file attribute query interface
- [`consoled`](https://github.com/BRX-Boruix/consoled) — the console byte stream service
- [`selftest`](https://github.com/BRX-Boruix/selftest) — the system self-test

## License

MIT License, copyright Yang Borui. See [LICENSE](LICENSE).
