extends Panel
class_name ThanhPhan
@onready var so: Label = $Button/so
@onready var tieude: Label = $Button/tieude
@onready var dang: Label = $Button/dang

@export var so_ex:String
@export var tieude_ex:String
@export var path:String
@export var dang_ex:String
@export var cau_hoi:String
@export var dap_an:Array[String]



# Called when the node enters the scene tree for the first time.
func _ready() -> void:
	so.text = so_ex
	tieude.text = tieude_ex
	dang.text = dang_ex
	pass # Replace with function body.

func _on_button_button_down() -> void:
	if (dang_ex == "Bài tập"):
		get_tree().change_scene_to_file(path)
	else:
		var vd: PackedScene = load("res://scene/trinh_phat_video.tscn")
		var scene_video = vd.instantiate() as TrinhPhatVideo
		scene_video.video_url = path
		get_tree().get_root().add_child(scene_video)
	pass # Replace with function body.
