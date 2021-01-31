use crate::prelude::*;

pub fn make_item(commands: &mut Commands, root: Entity, materials: &res::Materials, is_visible: bool) -> Entity {
	let size = (1, 1);
	let sprite_size = vec2(f32::from(size.0) * 0.05, f32::from(size.1) * 0.05);
	let entity = commands
		.spawn(SpriteBundle {
			material: materials.ring.clone(),
			transform: Transform::from_translation(vec3(sprite_size.x * 0.5, -sprite_size.y * 0.5, 1.)),
			sprite: Sprite::new(sprite_size),
			visible: Visible { is_visible, is_transparent: true },
			..default()
		})
		.with(cmp::Item { size })
		.with(Parent(root))
		.current_entity().unwrap();

	entity
}

pub fn make_drawer(commands: &mut Commands, root: Entity, materials: &res::Materials, coords: (u8, u8), size: (u8, u8)) {
	let material = &materials.drawers[&size];

	let sprite_size = vec2(f32::from(size.0) * 0.05, f32::from(size.1) * 0.05);
	let drawer_entity = commands
		.spawn(SpriteBundle {
			material: material.clone(),
			transform: Transform::from_translation(vec3(f32::from(coords.0) / 20. + sprite_size.x * 0.5, f32::from(coords.1) / 20. + sprite_size.y * 0.5, -10.)),
			sprite: Sprite::new(sprite_size),
			..default()
		})
		.with(Parent(root))
		.current_entity().unwrap();

	let drawer_root = commands
		.spawn((Parent(drawer_entity), Children::with(&[]), GlobalTransform::default(), Transform::from_translation(vec3(-sprite_size.x * 0.5, sprite_size.y * 0.5, 0.))))
		.current_entity().unwrap();

	let drawer = cmp::Drawer {
		size,
		items: HashMap::new(),
		// items: hmap![
		//     (0, 0) => make_item(commands, drawer_root, materials, false),
		// ],
		root: drawer_root,
	};
	commands.insert_one(drawer_entity, drawer);
}

pub fn system(
	commands: &mut Commands,
	asset_server: Res<AssetServer>,
	mut materials: ResMut<Assets<ColorMaterial>>,
	mut windows: ResMut<Windows>,
) {
	let window = windows.get_primary_mut().unwrap();
	window.set_resizable(false);
	// window.set_mode(bevy::window::WindowMode::BorderlessFullscreen);
	window.set_resolution(1600., 900.);
	window.set_title("ggj2021".to_owned());

	// let scale = 1./dbg!(window.height());
	let scale = 1./1600.;

	commands
		.spawn(Camera2dBundle {
			transform: Transform::from_scale(vec3(scale, scale, 1.)),
			..default()
		})
		.with(cmp::MainCamera)
		.spawn(CameraUiBundle::default())

		.spawn(TextBundle {
			text: Text {
				font: asset_server.load("fonts/FiraSans-Bold.ttf"),
				value: "Score:".to_string(),
				style: TextStyle {
					color: Color::rgb(0.5, 0.5, 1.0),
					font_size: 40.0,
					..default()
				},
			},
			style: Style {
				position_type: PositionType::Absolute,
				position: Rect {
					top: Val::Px(5.0),
					left: Val::Px(5.0),
					..default()
				},
				..default()
			},
			..default()
		});

	commands
		.spawn(SpriteBundle {
			material: materials.add(asset_server.load("sprites/window.png").into()),
			transform: Transform::from_translation(vec3(0., 0., -40.)),
			sprite: Sprite::new(vec2(1., 9./16.)),
			..default()
		})
		.spawn(SpriteBundle {
			material: materials.add(asset_server.load("sprites/counter.png").into()),
			transform: Transform::from_translation(vec3(0., 0., -30.)),
			sprite: Sprite::new(vec2(1., 9./16.)),
			..default()
		})
		.spawn(SpriteBundle {
			material: materials.add(asset_server.load("sprites/shelves_background.png").into()),
			transform: Transform::from_translation(vec3(0., 0., -20.)),
			sprite: Sprite::new(vec2(1., 9./16.)),
			..default()
		});

	let materials = res::Materials {
		black: materials.add(Color::rgb(0.26666, 0.2274, 0.2784).into()),
		book: materials.add(asset_server.load("sprites/book.png").into()),
		hat: materials.add(asset_server.load("sprites/hat.png").into()),
		ring: materials.add(asset_server.load("sprites/rings/ring_body_gold.png").into()),
		umbrella: materials.add(asset_server.load("sprites/umbrella.png").into()),
		drawers: hmap![
			(1, 1) => materials.add(asset_server.load("sprites/drawer1x1.png").into()),
			(1, 2) => materials.add(asset_server.load("sprites/drawer1x2.png").into()),
			(1, 3) => materials.add(asset_server.load("sprites/drawer1x3.png").into()),
			(1, 4) => materials.add(asset_server.load("sprites/drawer1x4.png").into()),
			(2, 1) => materials.add(asset_server.load("sprites/drawer2x1.png").into()),
			(2, 2) => materials.add(asset_server.load("sprites/drawer2x2.png").into()),
			(2, 3) => materials.add(asset_server.load("sprites/drawer2x3.png").into()),
			(3, 1) => materials.add(asset_server.load("sprites/drawer3x1.png").into()),
			(3, 2) => materials.add(asset_server.load("sprites/drawer3x2.png").into()),
			(3, 3) => materials.add(asset_server.load("sprites/drawer3x3.png").into()),
			(4, 1) => materials.add(asset_server.load("sprites/drawer4x1.png").into()),
		],
	};

	let drawer_config = vec![
		((0, 0), (1, 4)),
		((1, 0), (3, 3)),
		((4, 0), (2, 2)),
		((6, 0), (2, 1)),
		((12, 0), (3, 1)),
		((15, 0), (1, 1)),
		((16, 0), (2, 1)),
		((18, 0), (2, 2)),
		((6, 1), (2, 1)),
		((12, 1), (1, 1)),
		((13, 1), (1, 1)),
		((14, 1), (3, 3)),
		((17, 1), (1, 2)),
		((4, 2), (4, 1)),
		((12, 2), (2, 1)),
		((18, 2), (2, 2)),
		((1, 3), (1, 2)),
		((2, 3), (3, 2)),
		((5, 3), (1, 1)),
		((6, 3), (2, 2)),
		((12, 3), (2, 1)),
		((17, 3), (1, 1)),
		((0, 4), (1, 1)),
		((5, 4), (1, 2)),
		((8, 4), (1, 1)),
		((9, 4), (2, 1)),
		((11, 4), (2, 3)),
		((13, 4), (1, 2)),
		((14, 4), (2, 1)),
		((16, 4), (2, 2)),
		((18, 4), (1, 1)),
		((19, 4), (1, 4)),
		((0, 5), (2, 1)),
		((2, 5), (2, 2)),
		((4, 5), (1, 1)),
		((6, 5), (3, 1)),
		((9, 5), (2, 2)),
		((14, 5), (1, 1)),
		((15, 5), (1, 1)),
		((18, 5), (1, 1)),
		((1, 6), (1, 1)),
		((4, 6), (3, 2)),
		((7, 6), (2, 3)),
		((13, 6), (2, 3)),
		((15, 6), (4, 1)),
		((1, 7), (3, 1)),
		((9, 7), (1, 3)),
		((10, 7), (2, 1)),
		((12, 7), (1, 1)),
		((15, 7), (1, 2)),
		((16, 7), (2, 1)),
		((18, 7), (1, 1)),
		((2, 8), (2, 1)),
		((4, 8), (1, 2)),
		((5, 8), (1, 1)),
		((6, 8), (1, 1)),
		((10, 8), (1, 1)),
		((11, 8), (2, 2)),
		((17, 8), (2, 2)),
		((6, 9), (2, 1)),
	];

	let root = commands.spawn((GlobalTransform::default(), Transform::from_translation(vec3(-0.5, -0.28125 + 0.0125, 0.)))).current_entity().unwrap();
	for (coords, size) in drawer_config {
		make_drawer(commands, root, &materials, coords, size);
	}

	let lost_items_root = commands.spawn((GlobalTransform::default(), Transform::from_translation(vec3(0., -0.2125, -2.)))).current_entity().unwrap();

	let lost_items = res::LostItems(lost_items_root, vec![
		make_item(commands, lost_items_root, &materials, true),
		make_item(commands, lost_items_root, &materials, true),
		make_item(commands, lost_items_root, &materials, true),
	]);

	commands.insert_resource(lost_items);
	commands.insert_resource(materials);
}
