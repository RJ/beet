// use beet::prelude::*;
use beet::prelude::*;
// use example_plugin::ExamplePlugin;
use beet_examples::*;
use bevy::prelude::*;
use rand::prelude::IteratorRandom;
use std::time::Duration;

fn main() {
	let mut app = App::new();
	app.add_plugins((
		ExamplePlugin3d,
		DefaultBeetPlugins,
		BeetDebugPlugin::default(),
		DialogPanelPlugin,
		MlPlugin,
		ActionPlugin::<(
			// SetAgentOnRun<SteerTarget>,
			InsertOnAssetEvent<RunResult, Bert>,
			FindSentenceSteerTarget<With<Item>>,
			RemoveAgentOnRun<Sentence>,
			RemoveAgentOnRun<SteerTarget>,
		)>::default(),
	))
	.add_systems(Startup, (setup_camera, setup_fox, setup_chat, setup_items))
	.add_systems(Update, (set_player_sentence, rotate_items));

	#[cfg(target_arch = "wasm32")]
	app.add_plugins(PostmessageInputPlugin);

	app.run();
}

fn setup_camera(mut commands: Commands) {
	commands.spawn(Camera3dBundle {
		transform: Transform::from_xyz(0., 1.6, 5.)
			.looking_at(Vec3::new(0., 0., 0.), Vec3::Y),
		..default()
	});
}

#[derive(Component)]
pub struct Player;

fn setup_fox(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
	mut graphs: ResMut<Assets<AnimationGraph>>,
) {
	let mut graph = AnimationGraph::new();

	let idle_anim_clip = asset_server.load("Fox.glb#Animation0");
	let idle_anim_index =
		graph.add_clip(idle_anim_clip.clone(), 1.0, graph.root);
	let walk_anim_clip = asset_server.load("Fox.glb#Animation1");
	let walk_anim_index =
		graph.add_clip(walk_anim_clip.clone(), 1.0, graph.root);

	commands
		.spawn((
			Player,
			SceneBundle {
				scene: asset_server.load("Fox.glb#Scene0"),
				transform: Transform::from_xyz(0., 0., 0.)
					.with_scale(Vec3::splat(0.01)),
				..default()
			},
			graphs.add(graph),
			AnimationTransitions::new(),
			RotateToVelocity3d::default(),
			ForceBundle::default(),
			SteerBundle {
				max_force: MaxForce(0.05),
				max_speed: MaxSpeed(2.),
				..default()
			}
			.scaled_to(1.),
			// Uncomment this to have an initial target
			// Sentence::new("tasty"),
		))
		.with_children(|parent| {
			let agent = parent.parent_entity();

			let bert_handle = asset_server.load("default-bert.ron");

			parent
				.spawn((
					Name::new("Fetch Behavior"),
					Running,
					SequenceSelector,
					Repeat,
				))
				.with_children(|parent| {
					parent.spawn((
						Name::new("Await Bert Load"),
						InsertOnAssetEvent::loaded(
							RunResult::Success,
							&bert_handle,
						),
					));
					parent
						.spawn((
							Name::new("Idle Or Fetch"),
							TargetAgent(agent),
							ScoreSelector::default(),
							// ScoreSelector::consuming(),
							FindSentenceSteerTarget::<With<Item>>::new(
								bert_handle,
							),
						))
						.with_children(|parent| {
							parent.spawn((
								Name::new("Idle"),
								Score::neutral(),
								TargetAgent(agent),
								SetAgentOnRun(Velocity::default()),
								PlayAnimation::new(idle_anim_index)
									.repeat_forever(),
								RunTimer::default(),
								InsertInDuration::new(
									RunResult::Success,
									Duration::from_secs(1),
								),
							));
							parent
								.spawn((
									Name::new("Fetch"),
									Score::default(),
									TargetAgent(agent),
									ScoreSteerTarget::new(10.),
									PlayAnimation::new(walk_anim_index)
										.repeat_forever(),
									SequenceSelector,
									RemoveAgentOnRun::<Sentence>::default(),
								))
								.with_children(|parent| {
									parent.spawn((
										Name::new("Go To Item"),
										TargetAgent(agent),
										Seek,
										SucceedOnArrive::new(1.),
									));
									parent.spawn((
										Name::new("Pick Up Item"),
										TargetAgent(agent),
										// SetAgentOnRun(SteerTarget::Position(
										// 		Vec3::ZERO,
										// 	)),
										RemoveAgentOnRun::<SteerTarget>::default(),
										InsertOnRun(RunResult::Success),
									));
									// parent.spawn((
									// 	Name::new("Return Item To Center"),
									// 	TargetAgent(agent),
									// 	Seek,
									// 	SucceedOnArrive::new(6.),
									// ));
								});
						});
				});
		});
}


#[derive(Component)]
struct Item;

fn setup_items(mut commands: Commands, asset_server: Res<AssetServer>) {
	let scale = Vec3::splat(0.5);
	let offset = 2.;
	commands
		.spawn((
			Name::new("Potion"),
			Sentence::new("red healing potion"),
			Item,
			SpatialBundle {
				transform: Transform::from_xyz(offset, 0., offset),
				..default()
			},
		))
		.with_children(|parent| {
			parent.spawn(SceneBundle {
				scene: asset_server.load("kaykit/potion.glb#Scene0"),
				transform: Transform::from_xyz(0., 0., 0.).with_scale(scale),
				..default()
			});
		});
	commands
		.spawn((
			Name::new("Coin"),
			Sentence::new("gold coin"),
			Item,
			SpatialBundle {
				transform: Transform::from_xyz(offset, 0., -offset),
				..default()
			},
		))
		.with_children(|parent| {
			parent.spawn(SceneBundle {
				scene: asset_server.load("kaykit/coin.glb#Scene0"),
				transform: Transform::from_xyz(0., 0.2, 0.).with_scale(scale),
				..default()
			});
		});
	commands
		.spawn((
			Name::new("Sword"),
			Sentence::new("silver sword"),
			Item,
			SpatialBundle {
				transform: Transform::from_xyz(-offset, 0., offset),
				..default()
			},
		))
		.with_children(|parent| {
			parent.spawn(SceneBundle {
				scene: asset_server.load("kaykit/sword.glb#Scene0"),
				transform: Transform::from_xyz(0., 0.15, 0.).with_scale(scale),
				..default()
			});
		});
	commands
		.spawn((
			Name::new("Cheese"),
			Sentence::new("tasty cheese"),
			Item,
			SpatialBundle {
				transform: Transform::from_xyz(-offset, 0., -offset),
				..default()
			},
		))
		.with_children(|parent| {
			parent.spawn(SceneBundle {
				scene: asset_server.load("kaykit/cheese.glb#Scene0"),
				transform: Transform::from_xyz(0., 0., 0.).with_scale(scale),
				..default()
			});
		});
}

fn rotate_items(time: Res<Time>, mut query: Query<&mut Transform, With<Item>>) {
	for mut transform in query.iter_mut() {
		transform.rotate(Quat::from_rotation_y(time.delta_seconds() * 0.5));
	}
}

fn setup_chat(mut npc_events: EventWriter<OnNpcMessage>) {
	npc_events.send(OnNpcMessage(what_does_the_fox_say()));
}

fn set_player_sentence(
	mut commands: Commands,
	mut npc_events: EventWriter<OnNpcMessage>,
	mut events: EventReader<OnPlayerMessage>,
	query: Query<Entity, With<Player>>,
) {
	for ev in events.read() {
		log::info!("setting player sentence");
		commands
			.entity(query.iter().next().unwrap())
			.insert(Sentence::new(ev.0.clone()));

		npc_events.send(OnNpcMessage("ok".to_string()));
	}
}


fn what_does_the_fox_say() -> String {
	let sounds = [
		"Wa-pa-pa-pa-pa-pa-pow!",
		"Hatee-hatee-hatee-ho!",
		"Joff-tchoff-tchoffo-tchoffo-tchoff!",
		"Jacha-chacha-chacha-chow!",
		"Fraka-kaka-kaka-kaka-kow!",
		"A-hee-ahee ha-hee!",
		"A-oo-oo-oo-ooo!",
		"Ring-ding-ding-ding-dingeringeding!",
	];
	let sound = sounds.iter().choose(&mut rand::thread_rng()).unwrap();
	sound.to_string()
}
