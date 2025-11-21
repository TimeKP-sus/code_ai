use std::sync::{Arc, Mutex};

use godot::{
    builtin::{Color, Dictionary, GString},
    classes::{CodeEdit, CodeHighlighter, ICodeEdit, TextEdit},
    global::godot_print,
    obj::{Base, Gd, NewAlloc, NewGd, WithBaseField},
    prelude::{GodotClass, godot_api},
};

use crate::chatbot::hoi_dap_async;
use crate::pythonrun::run_python_async;

// use crate::compile_py::run_python_code;

#[derive(GodotClass)]
#[class(base=CodeEdit)]
pub struct VietCode {
    #[base]
    base: Base<CodeEdit>,
    text_e: Gd<TextEdit>,
    ai_result: Arc<Mutex<Option<String>>>,
    py_result: Arc<Mutex<Option<String>>>,
    // pending_code: Arc<Mutex<Option<String>>>,
    // cau_hoi: String,
}

#[godot_api]
impl ICodeEdit for VietCode {
    fn init(base: Base<CodeEdit>) -> Self {
        Self {
            base,
            text_e: TextEdit::new_alloc(),
            ai_result: Arc::new(Mutex::new(None)),

            py_result: Arc::new(Mutex::new(None)),
        }
    }

    fn ready(&mut self) {
        self.text_e = self.base().get_node_as::<TextEdit>("../kq_ai");
        godot_print!("VietCode is ready!");

        let mut to_mau: godot::prelude::Gd<CodeHighlighter> = CodeHighlighter::new_gd();

        let mut keyword_dict = Dictionary::new();

        let color_control: Color = Color::from_rgb(
            96.0 / 255.0, // Xanh dương nhạt
            165.0 / 255.0,
            250.0 / 255.0,
        );

        let color_logic: Color = Color::from_rgb(
            255.0 / 255.0, // Cam đào
            129.0 / 255.0,
            102.0 / 255.0,
        );

        let color_builtin: Color = Color::from_rgb(
            139.0 / 255.0, // Vàng nhạt
            128.0 / 255.0,
            0.0 / 255.0,
        );

        let color_class: Color = Color::from_rgb(
            186.0 / 255.0, // Tím pastel
            104.0 / 255.0,
            200.0 / 255.0,
        );

        let control_keywords: [&'static str; 12] = [
            "if", "elif", "else", "for", "while", "break", "continue", "try", "except", "finally",
            "raise", "with",
        ];
        let logic_keywords: [&'static str; 5] = ["and", "or", "not", "is", "in"];
        let builtin_keywords: [&'static str; 12] = [
            "return", "import", "from", "as", "lambda", "print", "input", "len", "True", "False",
            "None", "def",
        ];
        let class_keywords: [&'static str; 2] = ["class", "self"];

        for &kw in &control_keywords {
            let _ = keyword_dict.insert(kw, color_control);
        }
        for &kw in &logic_keywords {
            let _ = keyword_dict.insert(kw, color_logic);
        }
        for &kw in &builtin_keywords {
            let _ = keyword_dict.insert(kw, color_builtin);
        }
        for &kw in &class_keywords {
            let _ = keyword_dict.insert(kw, color_class);
        }
        to_mau.set_symbol_color(Color::from_rgb(113.0 / 255.0, 117.0 / 255.0, 255.0 / 255.0));
        to_mau.set_keyword_colors(&keyword_dict);

        let mut color_regions = Dictionary::new();
        let _ = color_regions.insert(
            "#",
            Color::from_rgb(128.0 / 255.0, 128.0 / 255.0, 128.0 / 255.0),
        );
        // let _ = color_regions.insert("\"", Color::from_rgb(0.8, 0.6, 0.2)); // chuỗi ký tự (string)
        to_mau.set_color_regions(&color_regions);

        self.base_mut().set_syntax_highlighter(&to_mau);
    }
    fn process(&mut self, _delta: f64) {
        let mut result: std::sync::MutexGuard<'_, Option<String>> = self.ai_result.lock().unwrap();
        if let Some(answer) = result.take() {
            if self.text_e.is_instance_valid() {
                self.text_e.set_text(&answer);
            }
        }
        let mut py_result: std::sync::MutexGuard<'_, Option<String>> =
            self.py_result.lock().unwrap();
        if let Some(output) = py_result.take() {
            if self.text_e.is_instance_valid() {
                self.text_e.set_text(&output);
            }
        }

        // let mut code: std::sync::MutexGuard<'_, Option<String>> = self.pending_code.lock().unwrap();
        // if let Some(py_code) = code.take() {
        //     let output = run_python_code(&py_code);
        //     self.text_e.set_text(&output);
        // }
    }
}

#[godot_api]
impl VietCode {
    #[func]
    fn kiem_tra(&mut self) {
        self.text_e.set_text("Đang kiểm tra...");
        self.base_mut().call_deferred("thuc_hien_kiem_tra", &[]);
    }

    #[func]
    fn thuc_hien_kiem_tra(&mut self) {
        let code: GString = self.base().get_text();
        godot_print!("CodeEdit: {:?}", code);
        if self.base().get_text().is_empty() {
            self.text_e
                .set_text("Bạn chưa viết gì cả! Hãy viết code Python cần kiểm tra.");
            return;
        }
        let ques: String = format!(
            "Check if the following Python code is correct or not.If code is nothing then say that there are nothing so they have to coding. If there are any errors, what and where the error is. Answer only in Vietnamese no need to print tthe question again or print the english answer, and clearly state and abbreviated as possible and no auto correct or the answer and check carefully the code I send. Here is the code.\n{}\n",
            code
        );
        godot_print!("Q: {}", &ques);

        let result_holder: Arc<Mutex<Option<String>>> = self.ai_result.clone();
        hoi_dap_async(ques, result_holder);

        // let answer: String = hoi_dap(&ques);
        // godot_print!("AI: {}", &answer);
        // if self.text_e.is_instance_valid() {
        //     self.text_e.set_text(&answer);
        // } else {
        //     godot_print!("kq_ai?");
        // }
    }
    #[func]
    fn chay_python(&mut self) {
        let code: GString = self.base().get_text();
        godot_print!("CodeEdit: {:?}", code);
        if self.base().get_text().is_empty() {
            self.text_e
                .set_text("Bạn chưa viết gì cả! Hãy viết code Python cần chạy.");
            return;
        }
        self.base_mut().call_deferred("thuc_hien_chay_python", &[]);
    }
    #[func]
    fn thuc_hien_chay_python(&mut self) {
        let codes: String = format!(
            "import sys\nsys.stdout.reconfigure(encoding='utf-8')\n{}",
            self.base().get_text()
        );

        godot_print!("Chạy mã Python: {:?}", codes);

        let result_holder: Arc<Mutex<Option<String>>> = self.ai_result.clone();
        run_python_async(codes.to_string(), result_holder);
    }
    // #[func]
    // fn run_python_code(&mut self, code: String) {
    //     let result = Command::new("python3").arg("-c").arg(code).output();

    //     match result {
    //         Ok(output) => {
    //             let stdout = String::from_utf8_lossy(&output.stdout);
    //             let stderr = String::from_utf8_lossy(&output.stderr);
    //             let kq = format!("Output:\n{}\nErrors:\n{}", stdout, stderr);
    //             godot_print!("Output: {}", &kq);
    //             self.text_e.set_text(&kq);
    //         }
    //         Err(e) => godot_print!("Failed to run Python: {}", e),
    //     }
}
// #[func]
// fn chay_py(&mut self) {
//     godot_print!("Đang chạy mã Python...");
//     let code: GString = self.base().get_text();
//     if code.is_empty() {
//         self.text_e
//             .set_text("Bạn chưa viết gì cả! Hãy viết code Python cần chạy.");
//         return;
//     }

// // Khi cần chạy code (ví dụ trong hàm Godot)
// let code: String = self.base().get_text().to_string();
// self.pending_code.lock().unwrap().replace(code);

// let code: GString = self.base().get_text();
// if code.is_empty() {
//     self.text_e
//         .set_text("Bạn chưa viết gì cả! Hãy viết code Python cần chạy.");
//     return;
// }
// let output: String = run_python_code(&code.to_string());
// godot_print!("Output: {}", &output);
// self.text_e.set_text(&output);

// #[func]
// fn ghi_lai(&mut self) {
//     let chu_moi =
//     self.cau_hoi.push_str(&chu_moi);
// }
