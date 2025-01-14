mod agent;
mod neural_network;
mod layer;
mod neuron;
mod stadium;

use godot::prelude::*;

struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}
