use std::process::Command;
use std::sync::{Arc, Mutex};
use std::thread;

// Hàm chạy mã Python không chặn UI
pub fn run_python_async(code: String, result_holder: Arc<Mutex<Option<String>>>) {
    thread::spawn(move || {
        let result: Result<std::process::Output, std::io::Error> = Command::new("pythonw")
            .arg("-c")
            .arg(code)
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .spawn()
            .and_then(|child| child.wait_with_output());

        let output = match result {
            Ok(output) => {
                let stdout = String::from_utf8_lossy(&output.stdout);
                let stderr = String::from_utf8_lossy(&output.stderr);
                format!("Kết quả:\n{}\nLỗi:\n{}", stdout, stderr)
            }
            Err(e) => format!("Lỗi???: {}", e),
        };

        let mut holder = result_holder.lock().unwrap();
        *holder = Some(output);
    });
}
