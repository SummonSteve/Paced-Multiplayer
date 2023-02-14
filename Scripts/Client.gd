extends Node

onready var cursor = $Cursor
var cursor_position = Vector2.ZERO
onready var client_node = get_parent().get_node("Client")
onready var dataframe = preload("res://Scripts/Data.gd").new()

func _input(event: InputEvent) -> void:
	if event is InputEventMouseMotion:
		if event.position.x > 512 and event.position.y > 300:
			cursor_position = Vector2(event.position.x - 512, event.position.y - 300)
			cursor.position = cursor_position
			var data = dataframe.DataFrame.new()
			data = dataframe.DataFrame.new()
			data.event = dataframe.Event.new()
			data.event.id = 0
			data.event.translation = Vector3(event.position.x - 512, event.position.y - 300, 0)
			client_node.send_data(data)
