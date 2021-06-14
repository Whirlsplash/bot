// Copyright (C) 2021-2021 The Whirlsplash Collective
// SPDX-License-Identifier: GPL-3.0-only

use std::{collections::HashMap, sync::Arc};

use serenity::{client::bridge::gateway::ShardManager, prelude::*};

pub struct ShardManagerContainer;
impl TypeMapKey for ShardManagerContainer {
  type Value = Arc<Mutex<ShardManager>>;
}

pub struct CommandCounter;
impl TypeMapKey for CommandCounter {
  type Value = HashMap<String, u64>;
}
