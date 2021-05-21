// Copyleft (ɔ) 2021-2021 The Whirlsplash Collective
// SPDX-License-Identifier: GPL-3.0-only

mod core;
mod modules;

use std::{
  collections::{HashMap, HashSet},
  sync::Arc,
};

use modules::commands::*;
use serenity::{framework::StandardFramework, http::Http, prelude::*};

use crate::core::{
  handler::Handler,
  keys::{CommandCounter, ShardManagerContainer},
};

#[tokio::main]
async fn main() {
  dotenv::dotenv().ok();

  let token = std::env::var("DISCORD_TOKEN").expect("expected a token in the environment");

  let http = Http::new_with_token(&token);

  let (owners, bot_id) = match http.get_current_application_info().await {
    Ok(info) => {
      let mut owners = HashSet::new();
      if let Some(team) = info.team {
        owners.insert(team.owner_user_id);
      } else {
        owners.insert(info.owner.id);
      }
      match http.get_current_user().await {
        Ok(bot_id) => (owners, bot_id.id),
        Err(why) => panic!("could not access bot id: {:?}", why),
      }
    }
    Err(why) => panic!("could not access application info: {:?}", why),
  };

  let framework = StandardFramework::new()
    .configure(|c| {
      c.with_whitespace(true)
        .on_mention(Some(bot_id))
        .prefix("w.")
        .delimiters(vec![", ", ","])
        .owners(owners)
    })
    .group(&GENERAL_GROUP);

  let mut client = Client::builder(&token)
    .event_handler(Handler)
    .framework(framework)
    .await
    .expect("error creating client");

  {
    let mut data = client.data.write().await;
    data.insert::<CommandCounter>(HashMap::default());
    data.insert::<ShardManagerContainer>(Arc::clone(&client.shard_manager));
  }

  if let Err(why) = client.start().await {
    println!("error starting client: {:?}", why);
  }
}
