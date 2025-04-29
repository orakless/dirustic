use serenity::all::{ChannelId, GuildId};

use crate::Context;

pub struct Playground {
    pub guild_id: GuildId,
    pub channel_id: Option<ChannelId>,
}

impl From<Context<'_>> for Playground {
    fn from(ctx: Context<'_>) -> Self {
        let guild = ctx.guild().unwrap();
        let channel_id = guild
            .voice_states
            .get(&ctx.author().id)
            .and_then(|voice_state| voice_state.channel_id);

        return Playground {
            guild_id: guild.id,
            channel_id,
        };
    }
}
