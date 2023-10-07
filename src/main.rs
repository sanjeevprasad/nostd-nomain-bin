#![no_std]
#![no_main]

use core::fmt::Write;

#[inline]
fn rust_main() -> core::result::Result<(), core::fmt::Error> {
  let s = "Hello, world!";
  let mut stdout = Writer::stdout();
  writeln!(stdout, "s: {}", s)?;
  let orig_num = 0;
  let next_num = next_num(orig_num);
  writeln!(stdout, "next_num: {}", next_num.next_num())?;
  Ok(())
}

#[no_mangle]
extern "C" fn main(_argc: isize, _argv: *const *const u8) -> isize {
  rust_main().unwrap();
  0
}

trait NextNum {
  fn next_num(&self) -> i32;
}

impl NextNum for i32 {
  fn next_num(&self) -> i32 {
    *self + 1
  }
}

fn next_num(i: i32) -> impl NextNum {
  i
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
  let mut stdout = Writer::stdout();
  writeln!(stdout, "s: {:?}", info).unwrap();
  loop {}
}

pub struct Writer(i32);
impl Writer {
  #[inline]
  pub fn stdout() -> Self {
    Self(1)
  }
  #[inline]
  pub fn stderr() -> Self {
    Self(2)
  }
}

impl core::fmt::Write for Writer {
  #[inline]
  fn write_str(&mut self, s: &str) -> core::fmt::Result {
    let msg = s.as_bytes();
    let mut written = 0;
    while written < msg.len() {
      let bytes = &msg[written..];
      let buf = bytes.as_ptr().cast::<core::ffi::c_void>();
      match unsafe { libc::write(self.0, buf, bytes.len()) } {
        n if n < 1 => break,
        res => written += res as usize,
      }
    }
    Ok(())
  }
}
