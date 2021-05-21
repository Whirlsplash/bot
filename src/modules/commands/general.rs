// Copyleft (ɔ) 2021-2021 The Whirlsplash Collective
// SPDX-License-Identifier: GPL-3.0-only

use serenity::{
  framework::standard::{macros::command, Args, CommandResult},
  model::channel::Message,
  prelude::*,
  utils::{content_safe, ContentSafeOptions},
};

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
