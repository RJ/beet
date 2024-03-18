use beet_core::prelude::*;
use beet_ecs::prelude::*;
use bevy::prelude::*;
use sweet::*;

#[sweet_test]
fn algo() -> Result<()> {
	let impulse = wander_impulse(
		&Vec3::default(),
		&Velocity::default(),
		&mut WanderParams::default(),
		MaxSpeed::default(),
		MaxForce::default(),
	);
	expect(*impulse).not().to_be(Vec3::ZERO)?;

	Ok(())
}


#[sweet_test]
fn action() -> Result<()> {
	let mut app = App::new();

	app.add_plugins((
		SteeringPlugin::default(),
		ActionPlugin::<CoreNode, _>::default(),
	))
	.insert_time();

	let agent = app
		.world
		.spawn((
			TransformBundle::default(),
			ForceBundle::default(),
			SteerBundle::default(),
		))
		.id();

	Wander::default()
		.into_beet_node()
		.spawn(&mut app.world, agent);

	app.update();
	app.update_with_secs(1);

	expect(&app)
		.component::<Transform>(agent)?
		.map(|t| t.translation)
		.not()
		.to_be(Vec3::ZERO)?;

	Ok(())
}
