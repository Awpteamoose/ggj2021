use crate::prelude::*;

pub fn make_item(commands: &mut Commands, root: Entity, item_template: &res::ItemTemplate, is_visible: bool) -> Entity {
	let size = item_template.size;
	let sprite_size = vec2(f32::from(size.0) * 0.05, f32::from(size.1) * 0.05);
	let entity = commands
		.spawn(SpriteBundle {
			material: item_template.material.clone(),
			transform: Transform::from_translation(vec3(sprite_size.x * 0.5, -sprite_size.y * 0.5, 1.)),
			sprite: Sprite::new(sprite_size),
			visible: Visible { is_visible, is_transparent: true },
			..default()
		})
		.with(cmp::Item { size, description: item_template.description })
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
		//	   (0, 0) => make_item(commands, drawer_root, materials, false),
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

		.spawn(NodeBundle {
			style: Style {
				size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
				position_type: PositionType::Absolute,
				justify_content: JustifyContent::Center,
				align_items: AlignItems::FlexEnd,
				position: Rect {
					top: Val::Px(170.),
					..default()
				},
				..default()
			},
			material: materials.add(Color::NONE.into()),
			..default()
		})
		.with_children(|parent| {
			parent.spawn(TextBundle {
				text: Text {
					font: asset_server.load("fonts/m5x7.ttf"),
					style: TextStyle {
						color: Color::rgb(1., 1., 1.),
						font_size: 40.0,
						alignment: TextAlignment {
							vertical: VerticalAlign::Top,
							horizontal: HorizontalAlign::Center,
						},
					},
					..default()
				},
				style: Style {
					align_self: AlignSelf::Center,
					..default()
				},
				..default()
			})
			.with(cmp::RequestText);
		});
		// .spawn(TextBundle {
		//	   text: Text {
		//		   font: asset_server.load("fonts/m5x7.ttf"),
		//		   style: TextStyle {
		//			   color: Color::rgb(1., 1., 1.),
		//			   font_size: 40.0,
		//			   alignment: TextAlignment {
		//				   vertical: VerticalAlign::Top,
		//				   horizontal: HorizontalAlign::Center,
		//			   },
		//		   },
		//		   ..default()
		//	   },
		//	   style: Style {
		//		   // display: Display::Flex,
		//		   // position_type: PositionType::Absolute,
		//		   // position: Rect {
		//		   //	  left: Val::Percent(50.),
		//		   //	  bottom: Val::Px(240.),
		//		   //	  ..default()
		//		   // },
		//		   align_self: AlignSelf::Center,
		//		   ..default()
		//	   },
		//	   ..default()
		// })
		// .with(cmp::RequestText);

	commands
		// .spawn(SpriteBundle {
		//	   material: materials.add(asset_server.load("sprites/man.png").into()),
		//	   transform: Transform::from_translation(vec3(0., 0., -50.)),
		//	   sprite: Sprite::new(vec2(1., 9./16.)),
		//	   ..default()
		// })
		// .with(cmp::Man)
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

	let materials_res = res::Materials {
		black: materials.add(Color::rgb(0.26666, 0.2274, 0.2784).into()),
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
		((0 , 0), (1, 4)),
		((1 , 0), (3, 3)),
		((4 , 0), (2, 2)),
		((6 , 0), (2, 1)),
		((12, 0), (3, 1)),
		((15, 0), (1, 1)),
		((16, 0), (2, 1)),
		((18, 0), (2, 2)),
		((6 , 1), (2, 1)),
		((12, 1), (1, 1)),
		((13, 1), (1, 1)),
		((14, 1), (3, 3)),
		((17, 1), (1, 2)),
		((4 , 2), (4, 1)),
		((12, 2), (2, 1)),
		((18, 2), (2, 2)),
		((1 , 3), (1, 2)),
		((2 , 3), (3, 2)),
		((5 , 3), (1, 1)),
		((6 , 3), (2, 2)),
		((12, 3), (2, 1)),
		((17, 3), (1, 1)),
		((0 , 4), (1, 1)),
		((5 , 4), (1, 2)),
		((8 , 4), (1, 1)),
		((9 , 4), (2, 1)),
		((11, 4), (2, 3)),
		((13, 4), (1, 2)),
		((14, 4), (2, 1)),
		((16, 4), (2, 2)),
		((18, 4), (1, 1)),
		((19, 4), (1, 4)),
		((0 , 5), (2, 1)),
		((2 , 5), (2, 2)),
		((4 , 5), (1, 1)),
		((6 , 5), (3, 1)),
		((9 , 5), (2, 2)),
		((14, 5), (1, 1)),
		((15, 5), (1, 1)),
		((18, 5), (1, 1)),
		((1 , 6), (1, 1)),
		((4 , 6), (3, 2)),
		((7 , 6), (2, 3)),
		((13, 6), (2, 3)),
		((15, 6), (4, 1)),
		((1 , 7), (3, 1)),
		((9 , 7), (1, 3)),
		((10, 7), (2, 1)),
		((12, 7), (1, 1)),
		((15, 7), (1, 2)),
		((16, 7), (2, 1)),
		((18, 7), (1, 1)),
		((2 , 8), (2, 1)),
		((4 , 8), (1, 2)),
		((5 , 8), (1, 1)),
		((6 , 8), (1, 1)),
		((10, 8), (1, 1)),
		((11, 8), (2, 2)),
		((17, 8), (2, 2)),
		((6 , 9), (2, 1)),
	];

	let root = commands.spawn((GlobalTransform::default(), Transform::from_translation(vec3(-0.5, -0.28125 + 0.0125, 0.)))).current_entity().unwrap();
	for (coords, size) in drawer_config {
		make_drawer(commands, root, &materials_res, coords, size);
	}

	let item_templates = res::ItemTemplates(vec![
		res::ItemTemplate {
			size: (2, 1),
			material: materials.add(asset_server.load("sprites/book_blue.png").into()),
			description: "a blue book"
		},
		res::ItemTemplate {
			size: (2, 1),
			material: materials.add(asset_server.load("sprites/book_brown.png").into()),
			description: "a brown book"
		},
		res::ItemTemplate {
			size: (2, 1),
			material: materials.add(asset_server.load("sprites/book_green.png").into()),
			description: "a green book"
		},
		res::ItemTemplate {
			size: (2, 1),
			material: materials.add(asset_server.load("sprites/book_pink.png").into()),
			description: "a pink book"
		},
		res::ItemTemplate {
			size: (2, 1),
			material: materials.add(asset_server.load("sprites/book_silver.png").into()),
			description: "a silver book"
		},
		res::ItemTemplate {
			size: (2, 1),
			material: materials.add(asset_server.load("sprites/book_yellow.png").into()),
			description: "a yellow book"
		},
		res::ItemTemplate {
			size: (1, 1),
			material: materials.add(asset_server.load("sprites/egg_blue_pink.png").into()),
			description: "a blue egg\nwith pink dots"
		},
		res::ItemTemplate {
			size: (1, 1),
			material: materials.add(asset_server.load("sprites/egg_blue_yellow.png").into()),
			description: "a blue egg\nwith yellow dots"
		},
		res::ItemTemplate {
			size: (1, 1),
			material: materials.add(asset_server.load("sprites/egg_pink_blue.png").into()),
			description: "a pink egg\nwith blue dots"
		},
		res::ItemTemplate {
			size: (1, 1),
			material: materials.add(asset_server.load("sprites/egg_pink_yellow.png").into()),
			description: "a pink egg\nwith yellow dots"
		},
		res::ItemTemplate {
			size: (1, 1),
			material: materials.add(asset_server.load("sprites/egg_yellow_pink.png").into()),
			description: "a yellow egg\nwith pink dots"
		},
		res::ItemTemplate {
			size: (1, 1),
			material: materials.add(asset_server.load("sprites/egg_yellow_blue.png").into()),
			description: "a yellow egg\nwith blue dots"
		},
		res::ItemTemplate {
			size: (1, 1),
			material: materials.add(asset_server.load("sprites/key_blue.png").into()),
			description: "a blue key"
		},
		res::ItemTemplate {
			size: (1, 1),
			material: materials.add(asset_server.load("sprites/key_copper.png").into()),
			description: "a copper key"
		},
		res::ItemTemplate {
			size: (1, 1),
			material: materials.add(asset_server.load("sprites/key_golden.png").into()),
			description: "a golden key"
		},
		res::ItemTemplate {
			size: (1, 1),
			material: materials.add(asset_server.load("sprites/key_green.png").into()),
			description: "a green key"
		},
		res::ItemTemplate {
			size: (1, 1),
			material: materials.add(asset_server.load("sprites/key_pink.png").into()),
			description: "a pink key"
		},
		res::ItemTemplate {
			size: (1, 1),
			material: materials.add(asset_server.load("sprites/key_silver.png").into()),
			description: "a silver key"
		},
		res::ItemTemplate {
			size: (1, 2),
			material: materials.add(asset_server.load("sprites/necklace_copper_blue.png").into()),
			description: "a copper necklace\nwith a blue stone"
		},
		res::ItemTemplate {
			size: (1, 2),
			material: materials.add(asset_server.load("sprites/necklace_copper_green.png").into()),
			description: "a copper necklace\nwith a green stone"
		},
		res::ItemTemplate {
			size: (1, 2),
			material: materials.add(asset_server.load("sprites/necklace_copper_pink.png").into()),
			description: "a copper necklace\nwith a pink stone"
		},
		res::ItemTemplate {
			size: (1, 2),
			material: materials.add(asset_server.load("sprites/necklace_golden_blue.png").into()),
			description: "a golden necklace\nwith a blue stone"
		},
		res::ItemTemplate {
			size: (1, 2),
			material: materials.add(asset_server.load("sprites/necklace_golden_green.png").into()),
			description: "a golden necklace\nwith a green stone"
		},
		res::ItemTemplate {
			size: (1, 2),
			material: materials.add(asset_server.load("sprites/necklace_golden_pink.png").into()),
			description: "a golden necklace\nwith a pink stone"
		},
		res::ItemTemplate {
			size: (1, 2),
			material: materials.add(asset_server.load("sprites/necklace_silver_blue.png").into()),
			description: "a silver necklace\nwith a blue stone"
		},
		res::ItemTemplate {
			size: (1, 2),
			material: materials.add(asset_server.load("sprites/necklace_silver_green.png").into()),
			description: "a silver necklace\nwith a green stone"
		},
		res::ItemTemplate {
			size: (1, 2),
			material: materials.add(asset_server.load("sprites/necklace_silver_pink.png").into()),
			description: "a silver necklace\nwith a pink stone"
		},
		res::ItemTemplate {
			size: (1, 1),
			material: materials.add(asset_server.load("sprites/potion_blue_round.png").into()),
			description: "a round blue potion"
		},
		res::ItemTemplate {
			size: (1, 1),
			material: materials.add(asset_server.load("sprites/potion_blue_square.png").into()),
			description: "a square blue potion"
		},
		res::ItemTemplate {
			size: (1, 1),
			material: materials.add(asset_server.load("sprites/potion_blue_triangle.png").into()),
			description: "a triangular blue potion"
		},
		res::ItemTemplate {
			size: (1, 1),
			material: materials.add(asset_server.load("sprites/potion_green_round.png").into()),
			description: "a round green potion"
		},
		res::ItemTemplate {
			size: (1, 1),
			material: materials.add(asset_server.load("sprites/potion_green_square.png").into()),
			description: "a square green potion"
		},
		res::ItemTemplate {
			size: (1, 1),
			material: materials.add(asset_server.load("sprites/potion_green_triangle.png").into()),
			description: "a triangular green potion"
		},
		res::ItemTemplate {
			size: (1, 1),
			material: materials.add(asset_server.load("sprites/potion_pink_round.png").into()),
			description: "a round pink potion"
		},
		res::ItemTemplate {
			size: (1, 1),
			material: materials.add(asset_server.load("sprites/potion_pink_square.png").into()),
			description: "a square pink potion"
		},
		res::ItemTemplate {
			size: (1, 1),
			material: materials.add(asset_server.load("sprites/potion_pink_triangle.png").into()),
			description: "a triangular pink potion"
		},
		res::ItemTemplate {
			size: (1, 1),
			material: materials.add(asset_server.load("sprites/ring_copper.png").into()),
			description: "a copper ring"
		},
		res::ItemTemplate {
			size: (1, 1),
			material: materials.add(asset_server.load("sprites/ring_copper_blue_round.png").into()),
			description: "a copper ring\nwith a round blue stone"
		},
		res::ItemTemplate {
			size: (1, 1),
			material: materials.add(asset_server.load("sprites/ring_copper_blue_square.png").into()),
			description: "a copper ring\nwith a square blue stone"
		},
		res::ItemTemplate {
			size: (1, 1),
			material: materials.add(asset_server.load("sprites/ring_copper_blue_triangle.png").into()),
			description: "a copper ring\nwith a triangular blue stone"
		},
		res::ItemTemplate {
			size: (1, 1),
			material: materials.add(asset_server.load("sprites/ring_copper_green_round.png").into()),
			description: "a copper ring\nwith a round green stone"
		},
		res::ItemTemplate {
			size: (1, 1),
			material: materials.add(asset_server.load("sprites/ring_copper_green_square.png").into()),
			description: "a copper ring\nwith a square green stone"
		},
		res::ItemTemplate {
			size: (1, 1),
			material: materials.add(asset_server.load("sprites/ring_copper_green_triangle.png").into()),
			description: "a copper ring\nwith a triangular green stone"
		},
		res::ItemTemplate {
			size: (1, 1),
			material: materials.add(asset_server.load("sprites/ring_copper_pink_round.png").into()),
			description: "a copper ring\nwith a round pink stone"
		},
		res::ItemTemplate {
			size: (1, 1),
			material: materials.add(asset_server.load("sprites/ring_copper_pink_square.png").into()),
			description: "a copper ring\nwith a square pink stone"
		},
		res::ItemTemplate {
			size: (1, 1),
			material: materials.add(asset_server.load("sprites/ring_copper_pink_triangle.png").into()),
			description: "a copper ring\nwith a triangular pink stone"
		},
		res::ItemTemplate {
			size: (1, 1),
			material: materials.add(asset_server.load("sprites/ring_golden.png").into()),
			description: "a golden ring"
		},
		res::ItemTemplate {
			size: (1, 1),
			material: materials.add(asset_server.load("sprites/ring_golden_blue_round.png").into()),
			description: "a golden ring\nwith a round blue stone"
		},
		res::ItemTemplate {
			size: (1, 1),
			material: materials.add(asset_server.load("sprites/ring_golden_blue_square.png").into()),
			description: "a golden ring\nwith a square blue stone"
		},
		res::ItemTemplate {
			size: (1, 1),
			material: materials.add(asset_server.load("sprites/ring_golden_blue_triangle.png").into()),
			description: "a golden ring\nwith a triangular blue stone"
		},
		res::ItemTemplate {
			size: (1, 1),
			material: materials.add(asset_server.load("sprites/ring_golden_green_round.png").into()),
			description: "a golden ring\nwith a round green stone"
		},
		res::ItemTemplate {
			size: (1, 1),
			material: materials.add(asset_server.load("sprites/ring_golden_green_square.png").into()),
			description: "a golden ring\nwith a square green stone"
		},
		res::ItemTemplate {
			size: (1, 1),
			material: materials.add(asset_server.load("sprites/ring_golden_green_triangle.png").into()),
			description: "a golden ring\nwith a triangular green stone"
		},
		res::ItemTemplate {
			size: (1, 1),
			material: materials.add(asset_server.load("sprites/ring_golden_pink_round.png").into()),
			description: "a golden ring\nwith a round pink stone"
		},
		res::ItemTemplate {
			size: (1, 1),
			material: materials.add(asset_server.load("sprites/ring_golden_pink_square.png").into()),
			description: "a golden ring\nwith a square pink stone"
		},
		res::ItemTemplate {
			size: (1, 1),
			material: materials.add(asset_server.load("sprites/ring_golden_pink_triangle.png").into()),
			description: "a golden ring\nwith a triangular pink stone"
		},
		res::ItemTemplate {
			size: (1, 1),
			material: materials.add(asset_server.load("sprites/ring_silver.png").into()),
			description: "a silver ring"
		},
		res::ItemTemplate {
			size: (1, 1),
			material: materials.add(asset_server.load("sprites/ring_silver_blue_round.png").into()),
			description: "a silver ring\nwith a round blue stone"
		},
		res::ItemTemplate {
			size: (1, 1),
			material: materials.add(asset_server.load("sprites/ring_silver_blue_square.png").into()),
			description: "a silver ring\nwith a square blue stone"
		},
		res::ItemTemplate {
			size: (1, 1),
			material: materials.add(asset_server.load("sprites/ring_silver_blue_triangle.png").into()),
			description: "a silver ring\nwith a triangular blue stone"
		},
		res::ItemTemplate {
			size: (1, 1),
			material: materials.add(asset_server.load("sprites/ring_silver_green_round.png").into()),
			description: "a silver ring\nwith a round green stone"
		},
		res::ItemTemplate {
			size: (1, 1),
			material: materials.add(asset_server.load("sprites/ring_silver_green_square.png").into()),
			description: "a silver ring\nwith a square green stone"
		},
		res::ItemTemplate {
			size: (1, 1),
			material: materials.add(asset_server.load("sprites/ring_silver_green_triangle.png").into()),
			description: "a silver ring\nwith a triangular green stone"
		},
		res::ItemTemplate {
			size: (1, 1),
			material: materials.add(asset_server.load("sprites/ring_silver_pink_round.png").into()),
			description: "a silver ring\nwith a round pink stone"
		},
		res::ItemTemplate {
			size: (1, 1),
			material: materials.add(asset_server.load("sprites/ring_silver_pink_square.png").into()),
			description: "a silver ring\nwith a square pink stone"
		},
		res::ItemTemplate {
			size: (1, 1),
			material: materials.add(asset_server.load("sprites/ring_silver_pink_triangle.png").into()),
			description: "a silver ring\nwith a triangular pink stone"
		},
		res::ItemTemplate {
			size: (2, 1),
			material: materials.add(asset_server.load("sprites/glasses_black_blue.png").into()),
			description: "blue-tinted glasses\nin a black frame"
		},
		res::ItemTemplate {
			size: (2, 1),
			material: materials.add(asset_server.load("sprites/glasses_black_green.png").into()),
			description: "green-tinted glasses\nin a black frame"
		},
		res::ItemTemplate {
			size: (2, 1),
			material: materials.add(asset_server.load("sprites/glasses_black_pink.png").into()),
			description: "pink-tinted glasses\nin a black frame"
		},
		res::ItemTemplate {
			size: (2, 1),
			material: materials.add(asset_server.load("sprites/glasses_black_yellow.png").into()),
			description: "yellow-tinted glasses\nin a black frame"
		},
		res::ItemTemplate {
			size: (2, 1),
			material: materials.add(asset_server.load("sprites/glasses_blue_black.png").into()),
			description: "dark-tinted glasses\nin a blue frame"
		},
		res::ItemTemplate {
			size: (2, 1),
			material: materials.add(asset_server.load("sprites/glasses_golden_black.png").into()),
			description: "dark-tinted glasses\nin a golden frame"
		},
		res::ItemTemplate {
			size: (2, 1),
			material: materials.add(asset_server.load("sprites/glasses_golden_blue.png").into()),
			description: "blue-tinted glasses\nin a golden frame"
		},
		res::ItemTemplate {
			size: (2, 1),
			material: materials.add(asset_server.load("sprites/glasses_golden_green.png").into()),
			description: "green-tinted glasses\nin a golden frame"
		},
		res::ItemTemplate {
			size: (2, 1),
			material: materials.add(asset_server.load("sprites/glasses_golden_pink.png").into()),
			description: "pink-tinted glasses\nin a golden frame"
		},
		res::ItemTemplate {
			size: (2, 1),
			material: materials.add(asset_server.load("sprites/glasses_green_black.png").into()),
			description: "dark-tinted glasses\nin a green frame"
		},
		res::ItemTemplate {
			size: (2, 1),
			material: materials.add(asset_server.load("sprites/glasses_green_blue.png").into()),
			description: "blue-tinted glasses\nin a green frame"
		},
		res::ItemTemplate {
			size: (2, 1),
			material: materials.add(asset_server.load("sprites/glasses_pink_black.png").into()),
			description: "dark-tinted glasses\nin a pink frame"
		},
		res::ItemTemplate {
			size: (2, 1),
			material: materials.add(asset_server.load("sprites/glasses_pink_blue.png").into()),
			description: "blue-tinted glasses\nin a pink frame"
		},
		res::ItemTemplate {
			size: (2, 1),
			material: materials.add(asset_server.load("sprites/glasses_silver_black.png").into()),
			description: "dark-tinted glasses\nin a silver frame"
		},
		res::ItemTemplate {
			size: (2, 1),
			material: materials.add(asset_server.load("sprites/glasses_silver_green.png").into()),
			description: "green-tinted glasses\nin a silver frame"
		},
		res::ItemTemplate {
			size: (2, 1),
			material: materials.add(asset_server.load("sprites/glasses_silver_blue.png").into()),
			description: "blue-tinted glasses\nin a silver frame"
		},
		res::ItemTemplate {
			size: (2, 1),
			material: materials.add(asset_server.load("sprites/glasses_silver_pink.png").into()),
			description: "pink-tinted glasses\nin a silver frame"
		},
	]);

	let lost_items_root = commands.spawn((GlobalTransform::default(), Transform::from_translation(vec3(0., -0.2125, -2.)))).current_entity().unwrap();

	let lost_items = res::LostItems(lost_items_root, vec![
		make_item(commands, lost_items_root, item_templates.get_random(), true),
		make_item(commands, lost_items_root, item_templates.get_random(), true),
		make_item(commands, lost_items_root, item_templates.get_random(), true),
		make_item(commands, lost_items_root, item_templates.get_random(), true),
		make_item(commands, lost_items_root, item_templates.get_random(), true),
		make_item(commands, lost_items_root, item_templates.get_random(), true),
		make_item(commands, lost_items_root, item_templates.get_random(), true),
		make_item(commands, lost_items_root, item_templates.get_random(), true),
		make_item(commands, lost_items_root, item_templates.get_random(), true),
	]);

	commands.insert_resource(res::ItemRequest("", true, 1));
	commands.insert_resource(item_templates);
	commands.insert_resource(lost_items);
	commands.insert_resource(materials_res);
}
