extends Label


func _process(_delta: float) -> void:
	var frame_rate = Engine.get_frames_per_second()
	set_text("FPS:" + str(frame_rate) + "\n" + str(stepify(1000/float(frame_rate), 0.01)) + "ms")
