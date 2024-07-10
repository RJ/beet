use crate::prelude::*;
use beet_ecs::prelude::*;
use bevy::prelude::*;
use std::borrow::Cow;

/// This component is for use with [`SentenceFlow`]. Add to either the agent or a child behavior.
#[derive(Debug, Clone, Component, PartialEq, Reflect)]
#[reflect(Component)]
pub struct Sentence(pub Cow<'static, str>);
impl Sentence {
	pub fn new(s: impl Into<Cow<'static, str>>) -> Self { Self(s.into()) }
}

/// Runs the child with the [`Sentence`] that is most similar to that of the agent.
/// for use with [`ScoreSelector`]
#[derive(Debug, Default, Clone, PartialEq, Action, Reflect)]
#[reflect(Component, ActionMeta)]
#[category(ActionCategory::ChildBehaviors)]
#[observers(sentence_flow)]
pub struct SentenceFlow;

impl SentenceFlow {
	pub fn new() -> Self { Self {} }
}

fn sentence_flow(
	trigger: Trigger<OnRun>,
	mut commands: Commands,
	mut berts: ResMut<Assets<Bert>>,
	sentences: Query<&Sentence>,
	// TODO double query, ie added running and added asset
	query: Query<(&SentenceFlow, &Sentence, &Handle<Bert>, &Children)>,
) {
	let (_scorer, target_sentence, handle, children) = query
		.get(trigger.entity())
		.expect(expect_action::ACTION_QUERY_MISSING);
	let Some(bert) = berts.get_mut(handle) else {
		log::warn!("{}", expect_asset::NOT_READY);
		return;
	};

	match bert.closest_sentence_entity(
		target_sentence.0.clone(),
		children.iter().map(|e| e.clone()),
		&sentences,
	) {
		Ok(entity) => {
			commands.entity(entity).trigger(OnRun);
		}
		Err(e) => log::error!("SentenceFlow: {}", e),
	}
}

#[cfg(test)]
mod test {
	use crate::prelude::*;
	use anyhow::Result;
	use beet_ecs::prelude::*;
	use bevy::prelude::*;
	use sweet::*;



	#[test]
	fn works() -> Result<()> {
		pretty_env_logger::try_init().ok();

		let mut app = App::new();
		app.add_plugins((
			MinimalPlugins,
			AssetPlugin::default(),
			BertPlugin::default(),
			LifecyclePlugin,
		))
		.finish();
		let on_run = observe_trigger_names::<OnRun>(app.world_mut());

		block_on_asset_load::<Bert>(&mut app, "default-bert.ron");

		let handle = app
			.world_mut()
			.resource_mut::<AssetServer>()
			.load::<Bert>("default-bert.ron");

		app.world_mut()
			.spawn((
				Name::new("root"),
				Sentence::new("destroy"),
				handle,
				SentenceFlow::default(),
			))
			.with_children(|parent| {
				parent.spawn((Name::new("heal"), Sentence::new("heal")));
				parent.spawn((Name::new("kill"), Sentence::new("kill")));
			})
			.flush_trigger(OnRun);


		expect(&on_run).to_have_been_called_times(2)?;
		expect(&on_run).to_have_returned_nth_with(0, &"root".to_string())?;
		expect(&on_run).to_have_returned_nth_with(1, &"kill".to_string())?;

		Ok(())
	}
}
