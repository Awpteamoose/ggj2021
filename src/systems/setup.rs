use crate::prelude::*;

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
		.spawn((Parent(drawer_entity), GlobalTransform::default(), Transform::from_translation(vec3(-sprite_size.x * 0.5, sprite_size.y * 0.5, 0.))))
		.current_entity().unwrap();

	let item_size = (1, 1);
	let item_sprite_size = vec2(f32::from(item_size.0) * 0.05, f32::from(item_size.1) * 0.05);
	let item_entity = commands
		.spawn(SpriteBundle {
			material: materials.ring.clone(),
			transform: Transform::from_translation(vec3(item_sprite_size.x * 0.5, -item_sprite_size.y * 0.5, 1.)),
			sprite: Sprite::new(item_sprite_size),
			visible: Visible { is_visible: false, is_transparent: true },
			..default()
		})
		.with(Parent(drawer_root))
		.with(cmp::Item { material: materials.ring.clone(), size: item_size })
		.current_entity().unwrap();

	commands.insert_one(drawer_entity, cmp::Drawer { material: material.clone(), size, items: hmap![(0, 0) => item_entity] });
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
		ring: materials.add(asset_server.load("sprites/ring.png").into()),
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
		// TODO:
	];

	let root = commands.spawn((GlobalTransform::default(), Transform::from_translation(vec3(-0.5, -0.28125 + 0.0125, 0.)))).current_entity().unwrap();
	for (coords, size) in drawer_config {
		make_drawer(commands, root, &materials, coords, size);
	}

	commands.insert_resource(materials);
}
