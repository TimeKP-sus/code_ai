extends Control

@onready var grid_container: GridContainer = $Control/ScrollContainer/GridContainer


#@export var so_ex:String
#@export var tieude_ex:String
#@export var path:String
#@export var dang_ex:String
#@export var cau_hoi:String
#@export var dap_an:Array[String]
@export var dic:Dictionary[String,Array] = {
	"1": ["Mở đầu", "Lý thuyết_Video", "res://asset/video/Python in 100 Seconds.ogv"],
	"1.1": ["test","test","test"],
	"2": ["Hello world", "Bài tập", "res://scene/main_code.tscn"],
	"3": ["Mở đầu", "Lý thuyết", "res://scene/main_code.tscn"],
	"3.1": ["test","test","test"],
	"4": ["Hello world", "Bài tập", "res://scene/main_code.tscn"],
	"5": ["Mở đầu", "Lý thuyết", "res://scene/main_code.tscn"],
	"6.1": ["test","test","test"],
	"7": ["Hello world", "Bài tập", "res://scene/main_code.tscn"],
}

var tp: PackedScene = preload("res://scene/thanh_phan_ds.tscn")

# Called when the node enters the scene tree for the first time.
func _ready() -> void:
	for i in dic.keys():
		var thanh_phan = tp.instantiate() as ThanhPhan
		thanh_phan.so_ex = i
		thanh_phan.tieude_ex = dic[i][0]
		thanh_phan.dang_ex = dic[i][1]
		thanh_phan.path = dic[i][2]
		print(thanh_phan.so_ex, " ", thanh_phan.tieude_ex)
		grid_container.add_child(thanh_phan)
	pass # Replace with function body.
