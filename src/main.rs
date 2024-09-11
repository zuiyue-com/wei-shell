use std::env;
use std::process::Command;
use serde::Serialize;
use serde_json::json;
use std::time::Duration;

#[macro_use]
extern crate wei_log;

#[derive(Serialize)]
struct CommandResult {
    success: bool,
    stdout: String,
    stderr: String,
    background: bool,
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 || args[1] != "run" {
        let error_json = json!({
            "error": format!("用法: {} run <命令>", args[0])
        });
        info!("{}", serde_json::to_string_pretty(&error_json).unwrap());
        return;
    }

    let command = &args[2..].join(" ");
    let is_background = command.contains('&') || command.contains("nohup");
    
    // 执行命令
    let child = Command::new("sh")
        .arg("-c")
        .arg(command)
        .spawn()
        .expect("命令执行失败");

    let result = if is_background {
        // 对于后台命令，等待很短的时间后返回
        std::thread::sleep(Duration::from_millis(100));
        CommandResult {
            success: true,
            stdout: "命令已在后台启动".to_string(),
            stderr: String::new(),
            background: true,
        }
    } else {
        // 对于前台命令，等待其完成
        let output = child.wait_with_output().expect("无法获取命令输出");
        CommandResult {
            success: output.status.success(),
            stdout: String::from_utf8_lossy(&output.stdout).to_string(),
            stderr: String::from_utf8_lossy(&output.stderr).to_string(),
            background: false,
        }
    };

    let json_result = serde_json::to_string_pretty(&result).unwrap();
    info!("{}", json_result);
}
