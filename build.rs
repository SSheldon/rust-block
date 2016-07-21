#[cfg(not(any(target_os = "macos", target_os = "ios")))]
extern crate gcc;
#[cfg(not(any(target_os = "macos", target_os = "ios")))]
use std::error::Error;
#[cfg(not(any(target_os = "macos", target_os = "ios")))]
use std::io::Write;
#[cfg(not(any(target_os = "macos", target_os = "ios")))]
use std::process::{ExitStatus,Stdio};
#[cfg(not(any(target_os = "macos", target_os = "ios")))]
use std::result::Result;
#[cfg(not(any(target_os = "macos", target_os = "ios")))]
static MINI_MAIN: &'static str = r#"
extern void *_Block_copy(const void *aBlock);
int main() {
    void (*ptr)(const void*) = _Block_copy;
    return 0;
}"#;

#[cfg(any(target_os = "macos", target_os = "ios"))]
fn main() {
  println!("cargo:rustc-link-lib=framework=System")
}

#[cfg(not(any(target_os = "macos", target_os = "ios")))]
fn check_block_copy_symbol_in_lib(library: &str) -> Result<(),String> {
    let invocation_result = gcc::Config::new()
      .compiler("clang")
      .get_compiler().to_command()
      .arg(format!("-l{}", library))
      .args(&["-x", "c"])
      .args(&["-o", "/dev/null"])
      .arg("-")
      .stdin(Stdio::piped())
      .stdout(Stdio::piped())
      .spawn();
    if invocation_result.is_err() {
        return Result::Err("Could not spawn compiler".to_string())
    }
    let mut process = invocation_result.unwrap();
    if let Err(why) = process.stdin.take().unwrap().write_all(MINI_MAIN.as_bytes()) {
        return Result::Err(format!("Could not compile: {}", why.description()))
    }
    process.wait()
        .or(Result::Err("Failed to compile".to_string()))
        .and_then(| status : ExitStatus| if status.success() {
            Result::Ok(())
        } else {
            Result::Err(format!("{} unavailable", library))
        })
}


#[cfg(not(any(target_os = "macos", target_os = "ios")))]
fn main() {
    if check_block_copy_symbol_in_lib("objc").is_ok() {
        println!("cargo:rustc-link-lib=objc")
    } else if let Err(why) = check_block_copy_symbol_in_lib("BlocksRuntime") {
        panic!("Could not find blocks runtime library: {}", why)
    } else {
        println!("cargo:rustc-link-lib=BlocksRuntime")
    }

}
