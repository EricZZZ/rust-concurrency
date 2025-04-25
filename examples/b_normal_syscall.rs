use std::io;

fn main() {
    let message = "你好👋，Hello, world from normal syscall!\n";
    let message = String::from(message);
    syscall(message).unwrap();

    // 使用标准库实现
    println!("Hello, world from normal syscall!");
}

#[cfg(target_family = "unix")]
#[link(name = "c")]
extern "C" {
    fn write(fd: u32, buf: *const u8, count: usize) -> i32;
}

// 使用正常的系统调用实现
fn syscall(message: String) -> io::Result<()> {
    let ptr = message.as_ptr();
    let len = message.len();

    let result = unsafe { write(1, ptr, len) };
    if result == -1 {
        return Err(io::Error::last_os_error());
    }

    Ok(())
}
