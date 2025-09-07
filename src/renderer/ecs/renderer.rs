use bevy_ecs::prelude::Resource;
use std::sync::{Arc, Mutex};

use super::super::Renderer;

#[derive(Resource)]
pub struct RendererRes(pub Arc<Mutex<Renderer>>);
