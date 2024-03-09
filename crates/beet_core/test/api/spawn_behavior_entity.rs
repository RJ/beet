use beet_core::prelude::*;
use beet_ecs::graph::BehaviorTree;
use beet_net::relay::Relay;
use bevy_app::App;
use bevy_math::Vec3;
use bevy_transform::components::Transform;
use sweet::*;

#[sweet_test]
pub fn works() -> Result<()> {
	let mut app = App::new();
	let mut relay = Relay::default();
	app.add_plugins(BeetPlugin::<CoreNode>::new(relay.clone()));
	app.insert_time();

	let mut send = SpawnEntityHandler::requester(&mut relay);

	let graph =
		BehaviorTree::<CoreNode>::new(Translate::new(Vec3::new(1., 0., 0.)));

	let message_id = send.start_request(
		&SpawnEntityPayload::default()
			.with_graph(graph)
			.with_position(Vec3::ZERO),
	)?;

	app.update_with_secs(2);
	let id = send.block_on_response(message_id)?;

	let entity = app
		.world
		.resource::<BeetEntityMap>()
		.map()
		.get(&id)
		.unwrap();

	let translation = app
		.world
		.entity(*entity)
		.get::<Transform>()
		.unwrap()
		.translation;
	expect(translation).to_be(Vec3::new(2., 0., 0.))?;

	Ok(())
}
