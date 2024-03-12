use crate::prelude::*;
use anyhow::anyhow;
use anyhow::Result;
use bevy_derive::Deref;
use bevy_derive::DerefMut;
use bevy_ecs::entity::Entity;
use bevy_ecs::reflect::AppTypeRegistry;
use bevy_ecs::world::World;
use petgraph::graph::DiGraph;


#[derive(Default, Debug, Clone, Deref, DerefMut)]
pub struct WillyBehaviorGraph(pub DiGraph<WillyBehaviorNode, ()>);

impl WillyBehaviorGraph {
	pub fn into_scene<T: ActionTypes>(&self) {}

	pub fn spawn(&self, world: &mut World, agent: Entity) -> EntityGraph {
		EntityGraph::spawn(world, self.clone(), agent)
	}

	pub fn spawn_no_target(&self, world: &mut World) -> EntityGraph {
		EntityGraph::spawn_no_target(world, self.clone())
	}

	pub fn with_indexed_names(mut self) -> Self {
		self.node_weights_mut().enumerate().for_each(|(i, node)| {
			node.name = format!("Node {i}");
		});
		self
	}


	/// # Errors
	/// If a type in the graph is missing from `T`
	fn get_checked_type_registry<T: ActionTypes>(
		&self,
	) -> Result<AppTypeRegistry> {
		let registry = BehaviorGraphPrefab::<T>::get_type_registry();
		let registry_read = registry.read();
		for node in self.node_weights() {
			for action in node.actions.iter() {
				registry_read
					.get_type_data::<ReflectAction>(action.type_id())
					.ok_or_else(|| {
						anyhow::anyhow!(
							"Type not registered: {:?}",
							action.type_id()
						)
					})?;
			}
		}
		drop(registry_read);

		Ok(registry)
	}

	pub fn into_prefab<T: ActionTypes>(self) -> Result<BehaviorGraphPrefab<T>> {
		let mut world = World::new();
		let entity_graph =
			EntityGraph::spawn_no_target(&mut world, self.clone());
		let _root = entity_graph
			.root()
			.ok_or_else(|| anyhow!("No root entity"))?;
		let registry = self.get_checked_type_registry::<T>()?;
		world.insert_resource(registry);
		Ok(BehaviorGraphPrefab::from_world(world))
	}
}


pub trait IntoWillyBehaviorGraph<M> {
	fn into_behavior_graph(self) -> WillyBehaviorGraph;
}

impl<M, T> IntoWillyBehaviorGraph<M> for T
where
	T: IntoWillyBehaviorTree<M>,
{
	fn into_behavior_graph(self) -> WillyBehaviorGraph {
		self.into_behavior_tree().into()
	}
}
