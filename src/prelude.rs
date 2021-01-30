pub use sugars::*;
pub use anyhow::Context;
pub use itertools::Itertools as _;
pub use once_cell::sync::{Lazy, OnceCell};
pub use rand::Rng;
pub use serde::{Deserialize, Serialize};
pub use std::{
	rc::Rc,
	cell::RefCell,
	collections::{BTreeMap, HashMap, HashSet},
	time::Duration,
	convert::TryFrom,
};
pub use typed_builder::TypedBuilder;
pub use parking_lot::{Mutex, RwLock};
pub use clone_block::clone;
pub use smart_default::SmartDefault;
pub use bevy::prelude::*;
pub use bevy_discovery::system;
pub use bevy_inspector_egui::Inspectable;
pub use crate::{components as cmp, systems as sys, resources as res};
pub use bevy::math::{Vec2Swizzles, Vec3Swizzles, Vec4Swizzles};
pub use shrinkwraprs::Shrinkwrap;
pub use bevy::math::*;

#[must_use]
pub fn default<T: Default>() -> T { T::default() }

// #[extend::ext(pub)]
// impl serde_json::Value {
//     fn from_pointer<T: serde::de::DeserializeOwned>(&self, pointer: &str) -> anyhow::Result<T> {
//         use anyhow::Context;

//         self
//             .pointer(pointer)
//             .context(format!("missing {}", pointer))
//             .and_then(|x| Ok(serde_json::from_value(x.clone())?))
//     }

//     fn from_pointer_mut<T: serde::de::DeserializeOwned>(&mut self, pointer: &str) -> anyhow::Result<T> {
//         use anyhow::Context;

//         self
//             .pointer_mut(pointer)
//             .map(serde_json::Value::take)
//             .context(format!("missing {}", pointer))
//             .and_then(|x| Ok(serde_json::from_value(x)?))
//     }
// }

// pub fn spawn_complain<T: Send + 'static>(x: impl Future<Output = anyhow::Result<T>> + Send + 'static) -> task::JoinHandle<anyhow::Result<T>> {
//     spawn(async move {
//         let res = x.await;
//         if let Err(e) = &res { log::error!("{:?}", e) };
//         res
//     })
// }

#[macro_export]
macro_rules! custom_try {
	($default: expr) => {
		#[allow(unused_macros)]
		macro_rules! try_opt {
			($test: expr) => { if let Some(x) = $test { x } else { return $default; }; }
		}

		#[allow(unused_macros)]
		macro_rules! try_res {
			($test: expr) => { if let Ok(x) = $test { x } else { return $default; }; }
		}

		#[allow(unused_macros)]
		macro_rules! try_assert {
			($test: expr) => { if !$test { return $default; }; }
		}

		#[allow(unused_macros)]
		macro_rules! try_assert_neg {
			($test: expr) => { if $test { return $default; }; }
		}
	}
}

pub fn collide(point: Vec2, center: Vec2, size: Vec2) -> bool {
	let (px, py) = (point.x, point.y);
	let (cx, cy) = (center.x, center.y);
	let (w, h) = (size.x * 0.5, size.y * 0.5);
	px >= cx - w && py >= cy - h && px <= cx + w && py <= cy + h
}
