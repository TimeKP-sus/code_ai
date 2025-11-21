extends Panel

@export var path:String
@export var logo:CompressedTexture2D
@export var ten:String
@export var mau:Color

func _ready() -> void:
	$Button/logo.texture = logo
	$Button/Label.text = ten
	# The snippet below assumes the child node "MyButton" has a StyleBoxFlat assigned.
	# Resources are shared across instances, so we need to duplicate it
	# to avoid modifying the appearance of all other buttons.
	var new_stylebox = $Button.get_theme_stylebox("hover").duplicate()
	new_stylebox.border_color = mau
	$Button.add_theme_stylebox_override("hover", new_stylebox)


func _on_button_button_down() -> void:
	get_tree().change_scene_to_file(path)
	pass # Replace with function body.
