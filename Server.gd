extends Node

onready var server = preload("res://server.gdns").new()
onready var dataframe = preload("res://Scripts/Data.gd").new()
onready var server_view_cursor = $"../ServerView/Cursor"

func _on_Server_pressed() -> void:
	server.start()

func _process(_delta: float) -> void:
	var buffer = server.recv_data_frame()
	if buffer != null:
		var data = dataframe.load_data(buffer)
		if data.event != null:
			server_view_cursor.position.x = data.event.translation.x
			server_view_cursor.position.y = data.event.translation.y
			server.send_data_frame(buffer)
