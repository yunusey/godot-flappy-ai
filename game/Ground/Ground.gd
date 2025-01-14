extends Area2D

const SCROLL_SPEED = -100.0
var playing = true

func _process(delta):
	if not playing:
		return

	position.x += SCROLL_SPEED * delta
	position.x = fmod(position.x, 24.0)
