use crate::prelude::*;

#[system]
pub fn system(
	items: Query<&cmp::Item>,
	mut request_texts: Query<(&mut Visible, &mut Text), With<cmp::RequestText>>,
	mut lost_items: ResMut<res::LostItems>,
	mut item_request: ResMut<res::ItemRequest>,
	item_templates: Res<res::ItemTemplates>,
	commands: &mut Commands,
) {
	let descriptions = items.iter().map(|x| x.description).collect::<Vec<_>>();
	let (mut visible, mut text) = request_texts.iter_mut().next().unwrap();

	if descriptions.iter().find(|&x| x == &item_request.0).is_none() {
		item_request.0 = descriptions[thread_rng().gen_range(0..descriptions.len())];
		text.value = format!("I think I lost \n{}.", item_request.0);
	}

	if descriptions.len() < item_request.2 && lost_items.1.is_empty() {
		for _ in 0..(2 * item_request.2) + thread_rng().gen_range(1..3) {
			let new_item = sys::setup::make_item(commands, lost_items.0, item_templates.get_random(), true);
			lost_items.1.push(new_item);
		}
	}

	visible.is_visible = lost_items.1.is_empty();

	if item_request.1 {
		item_request.2 += thread_rng().gen_range(1..3);
		item_request.1 = false;
		item_request.0 = descriptions[thread_rng().gen_range(0..descriptions.len())];
		text.value = format!("I think I lost \n{}.", item_request.0);
	}
}
