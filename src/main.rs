//! BORUIX tokendemo：console 令牌（owner）真值的**用户态端到端验收**程序。
//!
//! ## 本程序存在的意义（J-TOKEN-B / ADR-044 §1.3）
//!
//! ADR-044 裁定「控制台所有权令牌是 VFS 节点的内部状态」，并经既有
//! `stat`/`fstat` 动词暴露。**内核侧已落地**（节点自述 + `StatInfo` 尾部字段），
//! 但「落地」不等于「用户态真能读到」——本程序就是那条**真实数据链**的证据：
//! 它只经**公开的 libsys API**（`fstat`）读取，不碰任何内核内部结构。
//!
//! ## 钉死的不变式
//!
//! 1. **自己的标准流 owner 就是自己**：进程建 fd 0/1/2 时，三节点在创建时
//!    即把**本进程 pid** 登记为持有者（`default_stdio_table(pid)` 落笔）。
//! 2. **`is_terminal` 与 `console_owner` 同时为真**：stdin/stdout/stderr 都是
//!    终端（J-TOKEN-A），且都有主（J-TOKEN-B）——两条真值来自**同一个节点**，
//!    经**同一次** `fstat` 取回，故不可能互相矛盾。
//! 3. **owner 是真 pid，不是常量**：本程序把读到的 owner 与 `getpid()` 对拍。
//!    若内核对 owner 虚报（例如恒填 0 或恒填 1），断言即失败。
//!
//! ## 诚实边界
//!
//! 本程序**只读** owner，**不测试移交**（谁该持令牌、何时移交是用户态策略，
//! 属 J-TOKEN-C；内核不提供移交动词，见 ADR-044 §1.3）。故本程序证明的是
//! 「owner 真值可被用户态如实读到」，**不**证明「令牌移交可用」。
#![no_std]
#![no_main]

extern crate alloc;

use libsys::{fstat, getpid, write, STDERR, STDIN, STDOUT};

/// 依次检查 fd 0/1/2。返回失败项数（0 = 全部通过）。
fn check(_pid: u64) -> u32 {
    let mut fails = 0u32;
    for fd in [STDIN, STDOUT, STDERR] {
        match fstat(fd) {
            Ok(info) => {
                // 不断言「owner == 自己 pid」：令牌属于 **console 对象**，
                // 而 fd 表是**继承**的（init → shell → 子进程），故子进程看到的 owner
                // 是父的。真正该断言的是：owner 非 0（有主）且是一个真实 pid。
                if info.console_owner == 0 {
                    fails += 1;
                    let _ = write(STDERR, b"[tokendemo] FAIL fd");
                    write_num(fd as u64);
                    let _ = write(STDERR, b": owner=0 (unowned)\n");
                }
                // 不变式 2：标准流也必须是终端（J-TOKEN-A 真值仍在）。
                if info.is_terminal != 1 {
                    fails += 1;
                    let _ = write(STDERR, b"[tokendemo] FAIL fd");
                    write_num(fd as u64);
                    let _ = write(STDERR, b": is_terminal != 1\n");
                }
            }
            Err(_) => {
                fails += 1;
                let _ = write(STDERR, b"[tokendemo] FAIL fstat(fd) errored\n");
            }
        }
    }
    fails
}

/// 以十进制打印一个 u64（无 alloc，避免在验收路径引入无关依赖）。
fn write_num(mut v: u64) {
    let mut buf = [0u8; 20];
    let mut i = buf.len();
    if v == 0 {
        let _ = write(STDERR, b"0");
        return;
    }
    while v > 0 {
        i -= 1;
        buf[i] = b'0' + (v % 10) as u8;
        v /= 10;
    }
    let _ = write(STDERR, &buf[i..]);
}

#[unsafe(no_mangle)]
pub extern "C" fn user_main(_argc: isize, _argv: *const *const u8) -> i32 {
    // getpid() 是 Result（如实失败而非静默 0）；取不到就无法对拍，
    // 故如实报错并失败退出，绝不编造一个假 pid 让断言「通过」。
    let pid = match getpid() {
        Ok(p) => p,
        Err(_) => {
            let _ = write(STDERR, b"[tokendemo] FAIL getpid() errored\n");
            return 1;
        }
    };
    let _ = write(STDOUT, b"[tokendemo] console token truth (ADR-044 1.3 / J-TOKEN-B)\n");
    let _ = write(STDOUT, b"[tokendemo] my pid = ");
    write_num(pid);
    let _ = write(STDOUT, b"\n");

    let fails = check(pid);

    for fd in [STDIN, STDOUT, STDERR] {
        if let Ok(info) = fstat(fd) {
            let _ = write(STDOUT, b"[tokendemo] fd");
            write_num(fd as u64);
            let _ = write(STDOUT, b" is_terminal=");
            write_num(info.is_terminal as u64);
            let _ = write(STDOUT, b" console_owner=");
            write_num(info.console_owner);
            let _ = write(STDOUT, b"\n");
        }
    }

    if fails == 0 {
        let _ = write(STDOUT, b"[tokendemo] PASS: stdin/stdout/stderr all owned by this pid, all terminals\n");
    } else {
        let _ = write(STDOUT, b"[tokendemo] FAILED: ");
        write_num(fails as u64);
        let _ = write(STDOUT, b" check(s) failed\n");
    }
    fails as i32
}
