// Copyleft (ɔ) 2021-2021 The Whirlsplash Collective
// SPDX-License-Identifier: GPL-3.0-only

use serenity::{async_trait, model::gateway::Ready, prelude::*};

pub struct Handler;
#[async_trait]
impl EventHandler for Handler {
  async fn ready(&self, _: Context, ready: Ready) {
    if let Some(shard) = ready.shard {
      println!(
        "{} is connected on shard {}/{}",
        ready.user.name, shard[0], shard[1]
      );
    }
  }
}
