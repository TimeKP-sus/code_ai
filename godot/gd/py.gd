extends CodeHighlighter
func _init():
	# Python keywords
	var keywords = [
		"and", "as", "assert", "break", "class", "continue", "def", "del", "elif",
		"else", "except", "False", "finally", "for", "from", "global", "if", "import",
		"in", "is", "lambda", "None", "nonlocal", "not", "or", "pass", "raise", "return",
		"True", "try", "while", "with", "yield"
	]
	# Highlight keywords
	for keyword in keywords:
		add_keyword_color(keyword, Color(0.8, 0.3, 0.3)) # red-ish

	# Highlight comments
	add_color_region("#", "", Color(0.4, 0.6, 0.4), false)

	# Highlight strings
	add_color_region("\"", "\"", Color(0.7, 0.5, 0.1), false)
	add_color_region("'", "'", Color(0.7, 0.5, 0.1), false)
	add_color_region("\"\"\"", "\"\"\"", Color(0.7, 0.5, 0.1), false)
	add_color_region("'''", "'''", Color(0.7, 0.5, 0.1), false)
