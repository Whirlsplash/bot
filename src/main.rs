// Copyleft (ɔ) 2021-2021 The Whirlsplash Collective
// SPDX-License-Identifier: GPL-3.0-only

mod core;

use std::{
  collections::{HashMap, HashSet},
  sync::Arc,
};

use serenity::{
  framework::{
    standard::{
      macros::{command, group},
      Args,
      CommandResult,
    },
    StandardFramework,
  },
  http::Http,
  model::channel::Message,
  prelude::*,
  utils::{content_safe, ContentSafeOptions},
};

use crate::core::{
  handler::Handler,
  keys::{CommandCounter, ShardManagerContainer},
};

#[group]
#[commands(say)]
struct General;

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

#[command]
async fn say(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
  let settings = if let Some(guild_id) = msg.guild_id {
    ContentSafeOptions::default()
      .clean_channel(false)
      .display_as_member_from(guild_id)
  } else {
    ContentSafeOptions::default()
      .clean_channel(false)
      .clean_role(false)
  };

  let content = content_safe(&ctx.cache, &args.rest(), &settings).await;
  msg.delete(&ctx.http).await?;

  msg.channel_id.say(&ctx.http, &content).await?;

  Ok(())
}
