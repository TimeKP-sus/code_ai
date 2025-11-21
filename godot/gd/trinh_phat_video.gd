extends Control
class_name TrinhPhatVideo

@onready var video_stream_player: VideoStreamPlayer = $VideoStreamPlayer
@export var video_url: String
@onready var time_label: Label = $time

var time: int = 0
var da_dung: bool = false
var do_dai:int

func _ready() -> void:
	var video: VideoStreamTheora = VideoStreamTheora.new()
	video.file = video_url
	video_stream_player.stream = video
	video_stream_player.play()
	do_dai = int(video_stream_player.get_stream_length())
	print("Độ dài video: ", video_stream_player.get_stream_length())
	pass

func _on_timer_timeout() -> void:
	if video_stream_player.is_playing() and !video_stream_player.paused:
		time += 1
		cap_nhat_thoi_gian(time)
	pass

func _on_tua_tien_button_down() -> void:

	time = max(time - 5, 0)
	video_stream_player.stream_position = time
	cap_nhat_thoi_gian(time)
	pass

func _on_tua_lui_button_down() -> void:
	time = min(time + 5, do_dai)
	video_stream_player.stream_position = time
	cap_nhat_thoi_gian(time)
	pass

func cap_nhat_thoi_gian(tg: int) -> void:
	var phut: int = int(tg / 60.0)
	var giay: int = tg % 60
	time_label.text = "%02d:%02d" % [phut, giay]

func _on_tam_dung_button_down() -> void:
	da_dung = !da_dung
	video_stream_player.paused = da_dung
	pass
