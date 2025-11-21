




// pub fn run_python_code(code: &str) -> String {
//     Python::attach(|py| {
//         // Import sys và io, trả về lỗi rõ ràng nếu không thành công
//         let sys: Bound<'_, PyModule> = match py.import("sys") {
//             Ok(s) => s,
//             Err(e) => return format!("Không import được sys: {}", e),
//         };
//         let io: Bound<'_, PyModule> = match py.import("io") {
//             Ok(i) => i,
//             Err(e) => return format!("Không import được io: {}", e),
//         };

//         // Tạo StringIO để redirect stdout và stderr
//         let string_io: Bound<'_, PyAny> = match io.call_method0("StringIO") {
//             Ok(sio) => sio,
//             Err(e) => return format!("Không tạo được StringIO: {}", e),
//         };
//         if let Err(e) = sys.setattr("stdout", &string_io) {
//             return format!("Không set stdout: {}", e);
//         }
//         if let Err(e) = sys.setattr("stderr", &string_io) {
//             return format!("Không set stderr: {}", e);
//         }

//         let globals: Bound<'_, PyDict> = PyDict::new(py);

//         // Thực thi mã Python
//         let c_code: std::ffi::CString = std::ffi::CString::new(code).unwrap();
//         let result: Result<(), PyErr> = py.run(c_code.as_c_str(), Some(&globals), None);

//         // Lấy kết quả từ StringIO
//         let output_str: String = match string_io.call_method0("getvalue") {
//             Ok(val) => val.extract::<String>().unwrap_or_else(|_| "".to_string()),
//             Err(_) => "".to_string(),
//         };

//         match result {
//             Ok(_) => output_str,
//             Err(e) => format!("Lỗi khi chạy mã Python:\n{}\nOutput:\n{}", e, output_str),
//         }
//     })
// }