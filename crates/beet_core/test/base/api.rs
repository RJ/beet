use beet_core::base::BeetPlugin;
use beet_core::base::SpawnEntity;
use beet_core::base::SpawnEntityHandler;
use beet_ecs::builtin_nodes::BuiltinNode;
use beet_net::relay::Relay;
use bevy_app::App;
use bevy_math::Vec3;
use sweet::*;

#[sweet_test(non_send)]
pub async fn works() -> Result<()> {
	let mut app = App::new();
	let mut relay = Relay::default();

	let mut send = SpawnEntityHandler::requester(&mut relay);
	send.req_mut()
		.send(&SpawnEntity::with_position(Vec3::new(0., 0., 0.)))?;

	app.add_plugins(BeetPlugin::<BuiltinNode>::new(relay.clone()));

	app.finish();
	app.update();

	let id = send.res_mut().recv()?;
	expect(id).to_be(0)?;


	Ok(())
}
