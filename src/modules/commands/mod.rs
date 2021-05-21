// Copyleft (ɔ) 2021-2021 The Whirlsplash Collective
// SPDX-License-Identifier: GPL-3.0-only

pub mod general;

use general::*;
use serenity::framework::standard::macros::group;

#[group]
#[commands(say)]
struct General;
