use beet_core::base::BeetPlugin;
use beet_core::base::SpawnEntityPayload;
use beet_core::prelude::*;
use beet_ecs::builtin_nodes::BuiltinNode;
use beet_net::relay::Relay;
use bevy_app::App;
use bevy_math::Vec3;
use sweet::*;

#[sweet_test]
pub fn spawn_request() -> Result<()> {
	let mut app = App::new();
	let mut relay = Relay::default();
	app.add_plugins(BeetPlugin::<BuiltinNode>::new(relay.clone()));

	let mut send = SpawnEntityHandler::requester(&mut relay);
	let message_id = send.start_request(
		&SpawnEntityPayload::default().with_position(Vec3::new(0., 0., 0.)),
	)?;

	app.update();

	let id = send.block_on_response(message_id)?;
	expect(*id).to_be(0)?;

	Ok(())
}
