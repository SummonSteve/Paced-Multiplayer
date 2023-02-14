extends Node

class Event:
	var id: int
	var translation: Vector3

class DataFrame:
	var conv: int
	var time: int
	var msg: String
	var event: Event

func load_data(raw_data: PoolByteArray) -> DataFrame:
	var data = DataFrame.new()
	var stream = StreamPeerBuffer.new()
	stream.data_array = raw_data
	data.conv = stream.get_u32()
	data.time = stream.get_u32()
	var type = stream.get_u8()
	if type == 0:
		var _len = stream.get_u8()
		data.msg = stream.get_utf8_string(_len)
	elif type == 1:
		data.event = Event.new()
		data.event.id = stream.get_32()
		var v = Vector3.ZERO
		v.x = stream.get_float()
		v.y = stream.get_float()
		v.z = stream.get_float()
		data.event.translation = v

	return data

func parse_frame(frame: DataFrame) -> PoolByteArray:
	var stream = StreamPeerBuffer.new()
	stream.put_u32(frame.conv)
	stream.put_u32(frame.time)
	if frame.msg != "":
		stream.put_u8(0)
		stream.put_u8(frame.msg.length())
		stream.put_utf8_string(frame.msg)
	elif frame.event != null:
		stream.put_u8(1)
		stream.put_u32(frame.event.id)
		stream.put_float(frame.event.translation.x)
		stream.put_float(frame.event.translation.y)
		stream.put_float(frame.event.translation.z)
	return stream.data_array
