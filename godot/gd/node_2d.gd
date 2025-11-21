extends Node2D

var api_key: String = "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" # Thay bằng key của bạn
var url: String = "https://openrouter.ai/api/v1/chat/completions"
var model: String = "mistralai/mistral-7b-instruct" # Hoặc mô hình khác
var temp: float = 0.7
var max_token: int = 1024
var mess = []
var request: HTTPRequest


func _ready() -> void:
	request = HTTPRequest.new()
	add_child(request)
	request.connect("request_completed", _on_done)
	gui_thong_tin("giai thich godot la gi 20 tu")

func gui_thong_tin(thong_tin: String) -> void:
	mess.append({
		"role": "user",
		"content": thong_tin
	})

	var body = JSON.stringify({
		"model": model,
		"messages": mess,
		"temperature": temp,
		"max_tokens": max_token
	})

	var headers = [
		"Content-Type: application/json",
		"Authorization: Bearer " + api_key,
		"HTTP-Referer: https://localhost", # Bắt buộc, có thể là bất kỳ URL nào bạn sở hữu
		"X-Title: Godot AI Test" # Tên tùy chọn cho ứng dụng của bạn
	]

	var gui = request.request(url, headers, HTTPClient.METHOD_POST, body)
	if gui != OK:
		print("Lỗi gửi request!")

func _on_done(_result: int, _response_code: int, _headers: PackedStringArray, body: PackedByteArray) -> void:
	var json = JSON.new()
	if json.parse(body.get_string_from_utf8()) != OK:
		print("Lỗi parse JSON!")
		return

	var tra_ve = json.get_data()
	if tra_ve.has("error"):
		print("Lỗi từ API: ", tra_ve["error"]["message"])
		return

	var message = tra_ve["choices"][0]["message"]["content"]
	print("Phản hồi từ AI: ", message)
