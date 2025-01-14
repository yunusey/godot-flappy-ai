extends CharacterBody2D

const GRAVITY: int = 500
const MAX_VEL: int = 600
const FLAP_SPEED: int = -200

var flying: bool = true
var falling: bool = false
var score: int = 0
var current_pipe: Node2D = null
var stadium: Stadium = null
var agent_id: int = -1

func _ready():
	score = 0
	set_rotation(0)

func _physics_process(delta):
	if flying or falling:
		if get_should_jump():
			flap()

		velocity.y += GRAVITY * delta
		#terminal velocity
		if velocity.y > MAX_VEL:
			velocity.y = MAX_VEL
		if flying:
			set_rotation(deg_to_rad(velocity.y * 0.05))
			$AnimatedSprite2D.play()
		elif falling:
			set_rotation(PI/2)
			$AnimatedSprite2D.stop()

		position += velocity * delta
	else:
		$AnimatedSprite2D.stop()
	
		
func flap():
	velocity.y = FLAP_SPEED

func get_should_jump() -> bool:
	if current_pipe == null:
		return false

	var distance = current_pipe.position - position
	var input = PackedFloat64Array([
		distance.x / 288.0,
		distance.y / 512.0,
	])
	var result = stadium.feed_forward(agent_id, input)[0]
	return result > 0.5
