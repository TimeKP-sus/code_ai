extends Panel

@export var icon:CompressedTexture2D
@export var path:String
@export var scene_chinh:String

# Called when the node enters the scene tree for the first time.
func _ready() -> void:
	$Button.icon = icon
	pass # Replace with function body.

func _on_button_button_down() -> void:
	if path != "":
		get_tree().change_scene_to_file(path)
	if scene_chinh != "":
		var scene_node = get_tree().get_root().get_node_or_null(scene_chinh)
		if scene_node:
			scene_node.queue_free()
	pass # Replace with function body.
