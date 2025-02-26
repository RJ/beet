mod signal;
// use crate::rsx::RsxAttribute;
// use crate::rsx::RsxNode;
// use crate::rsx::RsxRust;
use crate::prelude::*;
pub use signal::*;



/// a signals implementation of an rsx mapper
pub struct SignalsRsx;

impl SignalsRsx {
	pub fn map_node_block<M>(
		block: impl 'static + Clone + IntoRsx<M>,
	) -> RsxNode {
		RsxNode::Block {
			initial: Box::new(block.clone().into_rsx()),
			register_effect: Box::new(move |cx| {
				let cx = cx.clone();
				effect(move || {
					let block = block.clone();
					let cx = cx.clone();
					CurrentHydrator::with(move |hydrator| {
						let node = block.clone().into_rsx();
						hydrator.update_rsx_node(node, &cx).unwrap()
					});
				});
			}),
		}
	}
	pub fn map_attribute_block(
		&self,
		mut block: impl 'static + FnMut() -> RsxAttribute,
	) -> RsxAttribute {
		RsxAttribute::Block {
			initial: vec![block()],
			register_effect: Box::new(move |cx| {
				let cx = cx.clone();
				effect(move || {
					let attrs = block();
					println!(
						"would update attributes for {}\n{}",
						cx.element_idx(),
						RsxToHtml::default().map_attribute(&attrs).render()
					);
					todo!();
				});
			}),
		}
	}
	pub fn map_attribute_value<M>(
		key: &str,
		block: impl 'static + Clone + IntoRsxAttributeValue<M>,
	) -> RsxAttribute {
		let key = key.to_string();
		RsxAttribute::BlockValue {
			key: key.clone(),
			initial: block.clone().into_attribute_value(),
			register_effect: Box::new(move |cx| {
				let cx = cx.clone();
				effect(move || {
					let value = block.clone().into_attribute_value();
					println!(
						"would update attribute for {}\n{key}: {value}",
						cx.element_idx()
					);
					todo!();
				});
			}),
		}
	}
}


#[cfg(test)]
mod test {
	use super::signal;
	use crate::prelude::*;
	use sweet::prelude::*;

	#[test]
	fn works() {
		let (get, set) = signal(7);

		let rsx = || rsx! {<div>value is {get}</div>};
		CurrentHydrator::set(HtmlNodeHydrator::new(rsx.clone()));

		rsx().register_effects();
		expect(&CurrentHydrator::with(|h| h.render()))
			.to_contain("<div data-sweet-id=\"0\">value is 7</div>");
		set(8);
		expect(&CurrentHydrator::with(|h| h.render()))
			.to_contain("<div data-sweet-id=\"0\">value is 8</div>");
		set(9);
		expect(&CurrentHydrator::with(|h| h.render()))
			.to_contain("<div data-sweet-id=\"0\">value is 9</div>");
	}
}
