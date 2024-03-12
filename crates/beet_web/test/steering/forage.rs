use beet::prelude::*;
use beet_web::prelude::*;
use bevy_math::Vec3;
use bevy_utils::prelude::default;
use sweet::*;

#[sweet_test]
fn serde_bytes() -> Result<()> {
	let prefab1 = BehaviorPrefab::<EcsNode>::from_graph(ConstantScore(
		Score::Weight(0.5),
	))?;
	let bytes1 = bincode::serialize(&prefab1)?;
	let prefab2: BehaviorPrefab<EcsNode> = bincode::deserialize(&bytes1)?;
	let bytes2 = bincode::serialize(&prefab2)?;
	expect(bytes1).to_be(bytes2)?;
	Ok(())
}


#[sweet_test]
async fn works() -> Result<()> {
	append_html_for_tests();
	let awareness_radius = 0.5;

	AppOptions {
		bees: 1,
		// flowers: 10,
		// auto_flowers: Some(1000),
		..default()
	}
	.with_graph(
		(
			Repeat::default(),
			UtilitySelector::default(),
			FindSteerTarget::new("flower", awareness_radius),
		)
			// .child((Wander::default(), SetRunResult::new(RunResult::Success)))
			.child((Wander::default(), ConstantScore::new(Score::Weight(0.5))))
			.child(
				(
					SequenceSelector::default(),
					ScoreSteerTarget::new(awareness_radius),
				)
					.child((Seek::default(), SucceedOnArrive { radius: 0.1 }))
					.child((
						SetVelocity(Vec3::ZERO),
						SucceedInDuration::with_secs(1),
					))
					.child((
						SetRunResult::success(),
						DespawnSteerTarget::default(),
					)),
			),
	)
	.run();
	Ok(())
}
