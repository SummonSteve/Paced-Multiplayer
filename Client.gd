extends Node

onready var client = preload("res://Client.gdns").new()
onready var dataframe = preload("res://Scripts/Data.gd").new()
onready var client_cursor_ghost = $"../ClientView/CursorGhost"
var conv = 0
var latencies = []

func _on_Client1_pressed() -> void:
	client.start()
	var data = dataframe.DataFrame.new()
	data.conv = conv
	data.time = Time.get_ticks_msec()
	data.msg = "hello"
	client.send_data_frame(dataframe.parse_frame(data))
	
func _process(_delta):
	var buffer = client.recv_data_frame()
	if buffer != null:
		var data = dataframe.load_data(buffer)
		var rtt = Time.get_ticks_msec() - data.time
		if len(latencies) > 100:
			latencies.pop_front()
		latencies.push_back(rtt/2)
		$Label.text = "RTT: "+ str(rtt) + "ms"
		$latency.text = "latency: " + str(sum_array(latencies)/100) + "ms"
		if data.msg == "handshake":
			conv = data.conv
		else:
			if data.event != null:
				client_cursor_ghost.position.x = data.event.translation.x
				client_cursor_ghost.position.y = data.event.translation.y
		
func send_data(data):
	data.conv = conv
	data.time = Time.get_ticks_msec()
	client.send_data_frame(dataframe.parse_frame(data))


static func sum_array(array):
	var sum = 0.0
	for element in array:
		 sum += element
	return sum
