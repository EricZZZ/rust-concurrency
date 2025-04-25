use std::arch::asm;

fn main() {
    let message = "Hello, world from raw syscall!\n";
    let message = String::from(message);
    syscall(message);
}

// 使用内联汇编实现原始系统调用
#[inline(never)]
fn syscall(message: String) {
    let ptr = message.as_ptr();
    let len = message.len();

    unsafe {
        asm!(
            "mov x16, 4",
            "mov x0, 1",
            "svc 0",
            in("x1") ptr,
            in("x2") len,
            out("x16") _,
            out("x0") _,
            lateout("x1") _,
            lateout("x2") _
        )
    }
}
