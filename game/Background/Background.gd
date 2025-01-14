extends ParallaxBackground

@export var speed = 100.0
var playing = true

func _process(delta):
	if not playing:
		return

	scroll_offset.x -= speed * delta
