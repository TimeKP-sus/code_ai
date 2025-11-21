extends MainNode
#@onready var chay_py = PythonRunner.new()

#func _on_chay_button_down():
	#var code = $VietCode.text
	#var bosung = "import sys\n"
	#bosung += "sys.stdout.reconfigure(encoding='utf-8')\n"
	#code = bosung + code
	#print(code)
	#
	#var temp_path = "res://temp_code.py"
#
	## Ghi mã Python vào file tạm
	#var file = FileAccess.open(temp_path, FileAccess.WRITE)
	#if file:
		#file.store_string(code)
		#file.close()
#
	#var output = []
	#var exit_code = OS.execute("python", [ProjectSettings.globalize_path(temp_path)], output, true)
	#if exit_code == 0:
		#$kq_ai.text = "Kết quả:\n" + str(output[0])
	#else:
		#$kq_ai.text = "Lỗi:\n" + str(output[0])

#func bien_doi(code: String) -> String:
	## Escape dấu nháy đơn và backslash
	#code = code.replace("\\", "\\\\")  # escape backslash
	#code = code.replace("'", "\\'")    # escape dấu nháy đơn
	#return "'" + code + "'"  # bao toàn bộ bằng dấu nháy đơn
