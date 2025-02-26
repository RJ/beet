use crate::prelude::*;
use beet_flow::prelude::*;
use bevy::prelude::*;


#[derive(Default)]
pub struct BertPlugin;

impl Plugin for BertPlugin {
	fn build(&self, app: &mut App) {

		app.add_plugins(ActionPlugin::<(
			SentenceFlow,
			InsertSentenceOnUserInput,
			RunOnInsertSentence 
		)>::default())
			.init_asset::<Bert>()
			.init_asset_loader::<BertLoader>()
			.register_type::<Sentence>()
			/*-*/;

		#[cfg(feature = "spatial")]
		app.add_plugins(
			ActionPlugin::<InsertSentenceSteerTarget<Sentence>>::default(),
		);

		let world = app.world_mut();
		world.register_component::<Sentence>();
	}
}
