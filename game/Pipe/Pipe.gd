extends Area2D

signal pipe_out
signal hit(body: Node2D)
signal scored(body: Node2D)

const SCROLL_SPEED = -100.0
var playing = true

func _process(delta):
	if not playing:
		return

	position.x += SCROLL_SPEED * delta
	if position.x < -100:
		pipe_out.emit()
		queue_free()


func _on_score_area_body_entered(body: Node2D):
	scored.emit(body)

func _on_body_entered(body: Node2D):
	hit.emit(body)
