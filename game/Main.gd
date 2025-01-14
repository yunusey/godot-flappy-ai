extends Node2D

const PIPE_RANGE = [100, 300]
const PLAYER_POSITION = Vector2(50, 250)

@export var num_players = 100
@export_range(0.0, 1.0, 0.01) var survival_percentage = 0.5
@export_range(0.0, 1.0, 0.01) var mutation_probability = 0.1
@export_range(0.0, 1.0, 0.01) var exploration_rate = 0.2
@export_range(0, 1000) var stop_at = 7 # if 0, it will not stop until all are dead
var pipe_scene = preload("res://Pipe/Pipe.tscn")
var player_scene = preload("res://Player/Player.tscn")
var generation = 0
var stadium = null

func _ready():
	stadium = Stadium.initialize_stadium(num_players, PackedInt32Array([2, 4, 1]))
	generate_pipe()
	initialize_players()

func initialize_players():
	$ScoreLabel.text = "0"
	$GenButton.text = "Gen #" + str(generation)
	$Ground.playing = true
	$Background.playing = true

	for i in range(num_players):
		var player = player_scene.instantiate()
		player.position = PLAYER_POSITION
		player.agent_id = i
		player.stadium = stadium
		player.current_pipe = $PipeContainer.get_child(0)
		$PlayerContainer.add_child(player)

func generate_pipe():
	var y_pos = randf_range(PIPE_RANGE[0], PIPE_RANGE[1])

	var pipe = pipe_scene.instantiate()
	pipe.position.y = y_pos
	pipe.position.x = 314

	pipe.pipe_out.connect(generate_pipe)
	pipe.hit.connect(player_died)
	pipe.scored.connect(player_scored)

	for player in $PlayerContainer.get_children():
		player.current_pipe = pipe

	$PipeContainer.add_child(pipe)

func _on_ground_body_entered(body: Node2D):
	player_died(body)

func player_died(player: Node2D):
	stadium.set_score(player.agent_id, player.score)
	# print("Player #" + str(player.agent_id) + " died")

	$PlayerContainer.remove_child(player)
	player.queue_free()

	if $PlayerContainer.get_child_count() == 0:
		handle_generation_over()
		return

func player_scored(player: Node2D):
	player.score += 1
	$ScoreLabel.text = str(player.score)

	if stop_at > 0 and player.score >= stop_at:
		player_died(player)

func handle_generation_over():
	generation += 1
	print("Generation #" + str(generation) + " over")
	print("Average score: " + str(stadium.get_average_score()))
	$Ground.playing = false
	$Background.playing = false

	for pipe in $PipeContainer.get_children():
		$PipeContainer.call_deferred("remove_child", pipe)
		pipe.queue_free()
	
	# Start new generation
	stadium.handle_new_generation(survival_percentage, mutation_probability, exploration_rate)

	generate_pipe.call_deferred()
	initialize_players.call_deferred()


func _on_gen_button_pressed() -> void:
	print('hope')
	var children = $PlayerContainer.get_children()
	for player in children:
		player_died(player)
