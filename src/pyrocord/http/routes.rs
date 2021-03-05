use reqwest::Method;

pub enum Route {
    // GET /applications/{application.id}/commands
    GetGlobalApplicationCommands(u64),

    // POST /applications/{application.id}/commands
    CreateGlobalApplicationCommand(u64),

    // DELETE /applications/{application.id}/commands/{command.id}
    DeleteGlobalApplicationCommand(u64, u64),

    // PATCH /applications/{application.id}/commands/{command.id}
    EditGlobalApplicationCommand(u64, u64),

    // GET /applications/{application.id}/entitlements
    GetEntitlements(u64),

    // DELETE /applications/{application.id}/entitlements/{entitlement.id}
    DeleteTestEntitlement(u64, u64),

    // GET /applications/{application.id}/entitlements/{entitlement.id}
    GetEntitlement(u64, u64),

    // POST /applications/{application.id}/entitlements/{entitlement.id}/consume
    ConsumeSKU(u64, u64),

    // GET /applications/{application.id}/guilds/{guild.id}/commands
    GetGuildApplicationCommands(u64, u64),

    // POST /applications/{application.id}/guilds/{guild.id}/commands
    CreateGuildApplicationCommand(u64, u64),

    // DELETE /applications/{application.id}/guilds/{guild.id}/commands/{command.id}
    DeleteGuildApplicationCommand(u64, u64, u64),

    // PATCH /applications/{application.id}/guilds/{guild.id}/commands/{command.id}
    EditGuildApplicationCommand(u64, u64, u64),

    // GET /applications/{application.id}/skus
    GetSKUs(u64),

    // DELETE /channels/{channel.id}
    DeleteOrCloseChannel(u64),

    // GET /channels/{channel.id}
    GetChannel(u64),

    // PATCH /channels/{channel.id}
    ModifyChannel(u64),

    // POST /channels/{channel.id}/followers
    FollowNewsChannel(u64),

    // GET /channels/{channel.id}/invites
    GetChannelInvites(u64),

    // POST /channels/{channel.id}/invites
    CreateChannelInvite(u64),

    // GET /channels/{channel.id}/messages
    GetChannelMessages(u64),

    // POST /channels/{channel.id}/messages
    CreateMessage(u64),

    // POST /channels/{channel.id}/messages/bulk-delete
    BulkDeleteMessages(u64),

    // DELETE /channels/{channel.id}/messages/{message.id}
    DeleteMessage(u64, u64),

    // GET /channels/{channel.id}/messages/{message.id}
    GetChannelMessage(u64, u64),

    // PATCH /channels/{channel.id}/messages/{message.id}
    EditMessage(u64, u64),

    // POST /channels/{channel.id}/messages/{message.id}/crosspost
    CrosspostMessage(u64, u64),

    // DELETE /channels/{channel.id}/messages/{message.id}/reactions
    DeleteAllReactions(u64, u64),

    // DELETE /channels/{channel.id}/messages/{message.id}/reactions/{emoji}
    DeleteAllReactionsforEmoji(u64, u64, String),

    // GET /channels/{channel.id}/messages/{message.id}/reactions/{emoji}
    GetReactions(u64, u64, String),

    // DELETE /channels/{channel.id}/messages/{message.id}/reactions/{emoji}/@me
    DeleteOwnReaction(u64, u64, String),

    // PUT /channels/{channel.id}/messages/{message.id}/reactions/{emoji}/@me
    CreateReaction(u64, u64, String),

    // DELETE /channels/{channel.id}/messages/{message.id}/reactions/{emoji}/{user.id}
    DeleteUserReaction(u64, u64, String, u64),

    // DELETE /channels/{channel.id}/permissions/{overwrite.id}
    DeleteChannelPermission(u64, u64),

    // PUT /channels/{channel.id}/permissions/{overwrite.id}
    EditChannelPermissions(u64, u64),

    // GET /channels/{channel.id}/pins
    GetPinnedMessages(u64),

    // DELETE /channels/{channel.id}/pins/{message.id}
    DeletePinnedChannelMessage(u64, u64),

    // PUT /channels/{channel.id}/pins/{message.id}
    AddPinnedChannelMessage(u64, u64),

    // DELETE /channels/{channel.id}/recipients/{user.id}
    GroupDMRemoveRecipient(u64, u64),

    // PUT /channels/{channel.id}/recipients/{user.id}
    GroupDMAddRecipient(u64, u64),

    // POST /channels/{channel.id}/typing
    TriggerTypingIndicator(u64),

    // GET /channels/{channel.id}/webhooks
    GetChannelWebhooks(u64),

    // POST /channels/{channel.id}/webhooks
    CreateWebhook(u64),

    // GET /gateway
    GetGateway,

    // GET /gateway/bot
    GetGatewayBot,

    // POST /guilds
    CreateGuild,

    // GET /guilds/templates/{template.code}
    GetTemplate(String),

    // POST /guilds/templates/{template.code}
    CreateGuildfromTemplate(String),

    // DELETE /guilds/{guild.id}
    DeleteGuild(u64),

    // GET /guilds/{guild.id}
    GetGuild(u64),

    // PATCH /guilds/{guild.id}
    ModifyGuild(u64),

    // GET /guilds/{guild.id}/audit-logs
    GetGuildAuditLog(u64),

    // GET /guilds/{guild.id}/bans
    GetGuildBans(u64),

    // DELETE /guilds/{guild.id}/bans/{user.id}
    RemoveGuildBan(u64, u64),

    // GET /guilds/{guild.id}/bans/{user.id}
    GetGuildBan(u64, u64),

    // PUT /guilds/{guild.id}/bans/{user.id}
    CreateGuildBan(u64, u64),

    // GET /guilds/{guild.id}/channels
    GetGuildChannels(u64),

    // PATCH /guilds/{guild.id}/channels
    ModifyGuildChannelPositions(u64),

    // POST /guilds/{guild.id}/channels
    CreateGuildChannel(u64),

    // GET /guilds/{guild.id}/emojis
    ListGuildEmojis(u64),

    // POST /guilds/{guild.id}/emojis
    CreateGuildEmoji(u64),

    // DELETE /guilds/{guild.id}/emojis/{emoji.id}
    DeleteGuildEmoji(u64, u64),

    // GET /guilds/{guild.id}/emojis/{emoji.id}
    GetGuildEmoji(u64, u64),

    // PATCH /guilds/{guild.id}/emojis/{emoji.id}
    ModifyGuildEmoji(u64, u64),

    // GET /guilds/{guild.id}/integrations
    GetGuildIntegrations(u64),

    // POST /guilds/{guild.id}/integrations
    CreateGuildIntegration(u64),

    // DELETE /guilds/{guild.id}/integrations/{integration.id}
    DeleteGuildIntegration(u64, u64),

    // PATCH /guilds/{guild.id}/integrations/{integration.id}
    ModifyGuildIntegration(u64, u64),

    // POST /guilds/{guild.id}/integrations/{integration.id}/sync
    SyncGuildIntegration(u64, u64),

    // GET /guilds/{guild.id}/invites
    GetGuildInvites(u64),

    // GET /guilds/{guild.id}/members
    ListGuildMembers(u64),

    // PATCH /guilds/{guild.id}/members/@me/nick
    ModifyCurrentUserNick(u64),

    // DELETE /guilds/{guild.id}/members/{user.id}
    RemoveGuildMember(u64, u64),

    // GET /guilds/{guild.id}/members/{user.id}
    GetGuildMember(u64, u64),

    // PATCH /guilds/{guild.id}/members/{user.id}
    ModifyGuildMember(u64, u64),

    // PUT /guilds/{guild.id}/members/{user.id}
    AddGuildMember(u64, u64),

    // DELETE /guilds/{guild.id}/members/{user.id}/roles/{role.id}
    RemoveGuildMemberRole(u64, u64, u64),

    // PUT /guilds/{guild.id}/members/{user.id}/roles/{role.id}
    AddGuildMemberRole(u64, u64, u64),

    // GET /guilds/{guild.id}/preview
    GetGuildPreview(u64),

    // GET /guilds/{guild.id}/prune
    GetGuildPruneCount(u64),

    // POST /guilds/{guild.id}/prune
    BeginGuildPrune(u64),

    // GET /guilds/{guild.id}/regions
    GetGuildVoiceRegions(u64),

    // GET /guilds/{guild.id}/roles
    GetGuildRoles(u64),

    // PATCH /guilds/{guild.id}/roles
    ModifyGuildRolePositions(u64),

    // POST /guilds/{guild.id}/roles
    CreateGuildRole(u64),

    // DELETE /guilds/{guild.id}/roles/{role.id}
    DeleteGuildRole(u64, u64),

    // PATCH /guilds/{guild.id}/roles/{role.id}
    ModifyGuildRole(u64, u64),

    // GET /guilds/{guild.id}/templates
    GetGuildTemplates(u64),

    // POST /guilds/{guild.id}/templates
    CreateGuildTemplate(u64),

    // DELETE /guilds/{guild.id}/templates/{template.code}
    DeleteGuildTemplate(u64, String),

    // PATCH /guilds/{guild.id}/templates/{template.code}
    ModifyGuildTemplate(u64, String),

    // PUT /guilds/{guild.id}/templates/{template.code}
    SyncGuildTemplate(u64, String),

    // GET /guilds/{guild.id}/vanity-url
    GetGuildVanityURL(u64),

    // GET /guilds/{guild.id}/webhooks
    GetGuildWebhooks(u64),

    // GET /guilds/{guild.id}/widget
    GetGuildWidgetSettings(u64),

    // PATCH /guilds/{guild.id}/widget
    ModifyGuildWidget(u64),

    // GET /guilds/{guild.id}/widget.json
    GetGuildWidget(u64),

    // GET /guilds/{guild.id}/widget.png
    GetGuildWidgetImage(u64),

    // POST /interactions/{interaction.id}/{interaction.token}/callback
    CreateInteractionResponse(u64, String),

    // DELETE /invites/{invite.code}
    DeleteInvite(String),

    // GET /invites/{invite.code}
    GetInvite(String),

    // GET /oauth2/applications/@me
    GetCurrentApplicationInformation,

    // DELETE /store/skus/{sku.id}/discounts/{user.id}
    DeletePurchaseDiscount(u64, u64),

    // PUT /store/skus/{sku.id}/discounts/{user.id}
    CreatePurchaseDiscount(u64, u64),

    // GET /users/@me
    GetCurrentUser,

    // PATCH /users/@me
    ModifyCurrentUser,

    // GET /users/@me/channels
    GetUserDMs,

    // POST /users/@me/channels
    CreateDM,

    // POST /users/@me/channels
    CreateGroupDM,

    // GET /users/@me/connections
    GetUserConnections,

    // GET /users/@me/guilds
    GetCurrentUserGuilds,

    // DELETE /users/@me/guilds/{guild.id}
    LeaveGuild(u64),

    // GET /users/{user.id}
    GetUser(u64),

    // GET /voice/regions
    ListVoiceRegions,

    // POST /webhooks/application.id/{interaction.token}
    CreateFollowupMessage(String),

    // DELETE /webhooks/application.id/{interaction.token}/messages/@original
    DeleteOriginalInteractionResponse(String),

    // PATCH /webhooks/application.id/{interaction.token}/messages/@original
    EditOriginalInteractionResponse(String),

    // DELETE /webhooks/application.id/{interaction.token}/messages/{message.id}
    DeleteFollowupMessage(String, u64),

    // PATCH /webhooks/application.id/{interaction.token}/messages/{message.id}
    EditFollowupMessage(String, u64),

    // DELETE /webhooks/{webhook.id}
    DeleteWebhook(u64),

    // GET /webhooks/{webhook.id}
    GetWebhook(u64),

    // PATCH /webhooks/{webhook.id}
    ModifyWebhook(u64),

    // DELETE /webhooks/{webhook.id}/{webhook.token}
    DeleteWebhookwithToken(u64, String),

    // GET /webhooks/{webhook.id}/{webhook.token}
    GetWebhookwithToken(u64, String),

    // PATCH /webhooks/{webhook.id}/{webhook.token}
    ModifyWebhookwithToken(u64, String),

    // POST /webhooks/{webhook.id}/{webhook.token}
    ExecuteWebhook(u64, String),

    // POST /webhooks/{webhook.id}/{webhook.token}/github
    ExecuteGitHubCompatibleWebhook(u64, String),

    // PATCH /webhooks/{webhook.id}/{webhook.token}/messages/{message.id}
    EditWebhookMessage(u64, String, u64),

    // POST /webhooks/{webhook.id}/{webhook.token}/slack
    ExecuteSlackCompatibleWebhook(u64, String),
}

impl Route {
    pub fn resolve(self) -> (Method, String) {
        match self {
            Self::GetGlobalApplicationCommands(application_id) => (
                Method::GET,
                format!(
                    "/applications/{application_id}/commands",
                    application_id = application_id
                ),
            ),
            Self::CreateGlobalApplicationCommand(application_id) => (
                Method::POST,
                format!(
                    "/applications/{application_id}/commands",
                    application_id = application_id
                ),
            ),
            Self::DeleteGlobalApplicationCommand(application_id, command_id) => (
                Method::DELETE,
                format!(
                    "/applications/{application_id}/commands/{command_id}",
                    application_id = application_id,
                    command_id = command_id
                ),
            ),
            Self::EditGlobalApplicationCommand(application_id, command_id) => (
                Method::PATCH,
                format!(
                    "/applications/{application_id}/commands/{command_id}",
                    application_id = application_id,
                    command_id = command_id
                ),
            ),
            Self::GetEntitlements(application_id) => (
                Method::GET,
                format!(
                    "/applications/{application_id}/entitlements",
                    application_id = application_id
                ),
            ),
            Self::DeleteTestEntitlement(application_id, entitlement_id) => (
                Method::DELETE,
                format!(
                    "/applications/{application_id}/entitlements/{entitlement_id}",
                    application_id = application_id,
                    entitlement_id = entitlement_id
                ),
            ),
            Self::GetEntitlement(application_id, entitlement_id) => (
                Method::GET,
                format!(
                    "/applications/{application_id}/entitlements/{entitlement_id}",
                    application_id = application_id,
                    entitlement_id = entitlement_id
                ),
            ),
            Self::ConsumeSKU(application_id, entitlement_id) => (
                Method::POST,
                format!(
                    "/applications/{application_id}/entitlements/{entitlement_id}/consume",
                    application_id = application_id,
                    entitlement_id = entitlement_id
                ),
            ),
            Self::GetGuildApplicationCommands(application_id, guild_id) => (
                Method::GET,
                format!(
                    "/applications/{application_id}/guilds/{guild_id}/commands",
                    application_id = application_id,
                    guild_id = guild_id
                ),
            ),
            Self::CreateGuildApplicationCommand(application_id, guild_id) => (
                Method::POST,
                format!(
                    "/applications/{application_id}/guilds/{guild_id}/commands",
                    application_id = application_id,
                    guild_id = guild_id
                ),
            ),
            Self::DeleteGuildApplicationCommand(application_id, guild_id, command_id) => (
                Method::DELETE,
                format!(
                    "/applications/{application_id}/guilds/{guild_id}/commands/{command_id}",
                    application_id = application_id,
                    guild_id = guild_id,
                    command_id = command_id
                ),
            ),
            Self::EditGuildApplicationCommand(application_id, guild_id, command_id) => (
                Method::PATCH,
                format!(
                    "/applications/{application_id}/guilds/{guild_id}/commands/{command_id}",
                    application_id = application_id,
                    guild_id = guild_id,
                    command_id = command_id
                ),
            ),
            Self::GetSKUs(application_id) => (
                Method::GET,
                format!(
                    "/applications/{application_id}/skus",
                    application_id = application_id
                ),
            ),
            Self::DeleteOrCloseChannel(channel_id) => (
                Method::DELETE,
                format!("/channels/{channel_id}", channel_id = channel_id),
            ),
            Self::GetChannel(channel_id) => (
                Method::GET,
                format!("/channels/{channel_id}", channel_id = channel_id),
            ),
            Self::ModifyChannel(channel_id) => (
                Method::PATCH,
                format!("/channels/{channel_id}", channel_id = channel_id),
            ),
            Self::FollowNewsChannel(channel_id) => (
                Method::POST,
                format!("/channels/{channel_id}/followers", channel_id = channel_id),
            ),
            Self::GetChannelInvites(channel_id) => (
                Method::GET,
                format!("/channels/{channel_id}/invites", channel_id = channel_id),
            ),
            Self::CreateChannelInvite(channel_id) => (
                Method::POST,
                format!("/channels/{channel_id}/invites", channel_id = channel_id),
            ),
            Self::GetChannelMessages(channel_id) => (
                Method::GET,
                format!("/channels/{channel_id}/messages", channel_id = channel_id),
            ),
            Self::CreateMessage(channel_id) => (
                Method::POST,
                format!("/channels/{channel_id}/messages", channel_id = channel_id),
            ),
            Self::BulkDeleteMessages(channel_id) => (
                Method::POST,
                format!(
                    "/channels/{channel_id}/messages/bulk-delete",
                    channel_id = channel_id
                ),
            ),
            Self::DeleteMessage(channel_id, message_id) => (
                Method::DELETE,
                format!(
                    "/channels/{channel_id}/messages/{message_id}",
                    channel_id = channel_id,
                    message_id = message_id
                ),
            ),
            Self::GetChannelMessage(channel_id, message_id) => (
                Method::GET,
                format!(
                    "/channels/{channel_id}/messages/{message_id}",
                    channel_id = channel_id,
                    message_id = message_id
                ),
            ),
            Self::EditMessage(channel_id, message_id) => (
                Method::PATCH,
                format!(
                    "/channels/{channel_id}/messages/{message_id}",
                    channel_id = channel_id,
                    message_id = message_id
                ),
            ),
            Self::CrosspostMessage(channel_id, message_id) => (
                Method::POST,
                format!(
                    "/channels/{channel_id}/messages/{message_id}/crosspost",
                    channel_id = channel_id,
                    message_id = message_id
                ),
            ),
            Self::DeleteAllReactions(channel_id, message_id) => (
                Method::DELETE,
                format!(
                    "/channels/{channel_id}/messages/{message_id}/reactions",
                    channel_id = channel_id,
                    message_id = message_id
                ),
            ),
            Self::DeleteAllReactionsforEmoji(channel_id, message_id, emoji) => (
                Method::DELETE,
                format!(
                    "/channels/{channel_id}/messages/{message_id}/reactions/{emoji}",
                    channel_id = channel_id,
                    message_id = message_id,
                    emoji = emoji
                ),
            ),
            Self::GetReactions(channel_id, message_id, emoji) => (
                Method::GET,
                format!(
                    "/channels/{channel_id}/messages/{message_id}/reactions/{emoji}",
                    channel_id = channel_id,
                    message_id = message_id,
                    emoji = emoji
                ),
            ),
            Self::DeleteOwnReaction(channel_id, message_id, emoji) => (
                Method::DELETE,
                format!(
                    "/channels/{channel_id}/messages/{message_id}/reactions/{emoji}/@me",
                    channel_id = channel_id,
                    message_id = message_id,
                    emoji = emoji
                ),
            ),
            Self::CreateReaction(channel_id, message_id, emoji) => (
                Method::PUT,
                format!(
                    "/channels/{channel_id}/messages/{message_id}/reactions/{emoji}/@me",
                    channel_id = channel_id,
                    message_id = message_id,
                    emoji = emoji
                ),
            ),
            Self::DeleteUserReaction(channel_id, message_id, emoji, user_id) => (
                Method::DELETE,
                format!(
                    "/channels/{channel_id}/messages/{message_id}/reactions/{emoji}/{user_id}",
                    channel_id = channel_id,
                    message_id = message_id,
                    emoji = emoji,
                    user_id = user_id
                ),
            ),
            Self::DeleteChannelPermission(channel_id, overwrite_id) => (
                Method::DELETE,
                format!(
                    "/channels/{channel_id}/permissions/{overwrite_id}",
                    channel_id = channel_id,
                    overwrite_id = overwrite_id
                ),
            ),
            Self::EditChannelPermissions(channel_id, overwrite_id) => (
                Method::PUT,
                format!(
                    "/channels/{channel_id}/permissions/{overwrite_id}",
                    channel_id = channel_id,
                    overwrite_id = overwrite_id
                ),
            ),
            Self::GetPinnedMessages(channel_id) => (
                Method::GET,
                format!("/channels/{channel_id}/pins", channel_id = channel_id),
            ),
            Self::DeletePinnedChannelMessage(channel_id, message_id) => (
                Method::DELETE,
                format!(
                    "/channels/{channel_id}/pins/{message_id}",
                    channel_id = channel_id,
                    message_id = message_id
                ),
            ),
            Self::AddPinnedChannelMessage(channel_id, message_id) => (
                Method::PUT,
                format!(
                    "/channels/{channel_id}/pins/{message_id}",
                    channel_id = channel_id,
                    message_id = message_id
                ),
            ),
            Self::GroupDMRemoveRecipient(channel_id, user_id) => (
                Method::DELETE,
                format!(
                    "/channels/{channel_id}/recipients/{user_id}",
                    channel_id = channel_id,
                    user_id = user_id
                ),
            ),
            Self::GroupDMAddRecipient(channel_id, user_id) => (
                Method::PUT,
                format!(
                    "/channels/{channel_id}/recipients/{user_id}",
                    channel_id = channel_id,
                    user_id = user_id
                ),
            ),
            Self::TriggerTypingIndicator(channel_id) => (
                Method::POST,
                format!("/channels/{channel_id}/typing", channel_id = channel_id),
            ),
            Self::GetChannelWebhooks(channel_id) => (
                Method::GET,
                format!("/channels/{channel_id}/webhooks", channel_id = channel_id),
            ),
            Self::CreateWebhook(channel_id) => (
                Method::POST,
                format!("/channels/{channel_id}/webhooks", channel_id = channel_id),
            ),
            Self::GetGateway => (Method::GET, "/gateway".to_string()),
            Self::GetGatewayBot => (Method::GET, "/gateway/bot".to_string()),
            Self::CreateGuild => (Method::POST, "/guilds".to_string()),
            Self::GetTemplate(template_code) => (
                Method::GET,
                format!(
                    "/guilds/templates/{template_code}",
                    template_code = template_code
                ),
            ),
            Self::CreateGuildfromTemplate(template_code) => (
                Method::POST,
                format!(
                    "/guilds/templates/{template_code}",
                    template_code = template_code
                ),
            ),
            Self::DeleteGuild(guild_id) => (
                Method::DELETE,
                format!("/guilds/{guild_id}", guild_id = guild_id),
            ),
            Self::GetGuild(guild_id) => (
                Method::GET,
                format!("/guilds/{guild_id}", guild_id = guild_id),
            ),
            Self::ModifyGuild(guild_id) => (
                Method::PATCH,
                format!("/guilds/{guild_id}", guild_id = guild_id),
            ),
            Self::GetGuildAuditLog(guild_id) => (
                Method::GET,
                format!("/guilds/{guild_id}/audit-logs", guild_id = guild_id),
            ),
            Self::GetGuildBans(guild_id) => (
                Method::GET,
                format!("/guilds/{guild_id}/bans", guild_id = guild_id),
            ),
            Self::RemoveGuildBan(guild_id, user_id) => (
                Method::DELETE,
                format!(
                    "/guilds/{guild_id}/bans/{user_id}",
                    guild_id = guild_id,
                    user_id = user_id
                ),
            ),
            Self::GetGuildBan(guild_id, user_id) => (
                Method::GET,
                format!(
                    "/guilds/{guild_id}/bans/{user_id}",
                    guild_id = guild_id,
                    user_id = user_id
                ),
            ),
            Self::CreateGuildBan(guild_id, user_id) => (
                Method::PUT,
                format!(
                    "/guilds/{guild_id}/bans/{user_id}",
                    guild_id = guild_id,
                    user_id = user_id
                ),
            ),
            Self::GetGuildChannels(guild_id) => (
                Method::GET,
                format!("/guilds/{guild_id}/channels", guild_id = guild_id),
            ),
            Self::ModifyGuildChannelPositions(guild_id) => (
                Method::PATCH,
                format!("/guilds/{guild_id}/channels", guild_id = guild_id),
            ),
            Self::CreateGuildChannel(guild_id) => (
                Method::POST,
                format!("/guilds/{guild_id}/channels", guild_id = guild_id),
            ),
            Self::ListGuildEmojis(guild_id) => (
                Method::GET,
                format!("/guilds/{guild_id}/emojis", guild_id = guild_id),
            ),
            Self::CreateGuildEmoji(guild_id) => (
                Method::POST,
                format!("/guilds/{guild_id}/emojis", guild_id = guild_id),
            ),
            Self::DeleteGuildEmoji(guild_id, emoji_id) => (
                Method::DELETE,
                format!(
                    "/guilds/{guild_id}/emojis/{emoji_id}",
                    guild_id = guild_id,
                    emoji_id = emoji_id
                ),
            ),
            Self::GetGuildEmoji(guild_id, emoji_id) => (
                Method::GET,
                format!(
                    "/guilds/{guild_id}/emojis/{emoji_id}",
                    guild_id = guild_id,
                    emoji_id = emoji_id
                ),
            ),
            Self::ModifyGuildEmoji(guild_id, emoji_id) => (
                Method::PATCH,
                format!(
                    "/guilds/{guild_id}/emojis/{emoji_id}",
                    guild_id = guild_id,
                    emoji_id = emoji_id
                ),
            ),
            Self::GetGuildIntegrations(guild_id) => (
                Method::GET,
                format!("/guilds/{guild_id}/integrations", guild_id = guild_id),
            ),
            Self::CreateGuildIntegration(guild_id) => (
                Method::POST,
                format!("/guilds/{guild_id}/integrations", guild_id = guild_id),
            ),
            Self::DeleteGuildIntegration(guild_id, integration_id) => (
                Method::DELETE,
                format!(
                    "/guilds/{guild_id}/integrations/{integration_id}",
                    guild_id = guild_id,
                    integration_id = integration_id
                ),
            ),
            Self::ModifyGuildIntegration(guild_id, integration_id) => (
                Method::PATCH,
                format!(
                    "/guilds/{guild_id}/integrations/{integration_id}",
                    guild_id = guild_id,
                    integration_id = integration_id
                ),
            ),
            Self::SyncGuildIntegration(guild_id, integration_id) => (
                Method::POST,
                format!(
                    "/guilds/{guild_id}/integrations/{integration_id}/sync",
                    guild_id = guild_id,
                    integration_id = integration_id
                ),
            ),
            Self::GetGuildInvites(guild_id) => (
                Method::GET,
                format!("/guilds/{guild_id}/invites", guild_id = guild_id),
            ),
            Self::ListGuildMembers(guild_id) => (
                Method::GET,
                format!("/guilds/{guild_id}/members", guild_id = guild_id),
            ),
            Self::ModifyCurrentUserNick(guild_id) => (
                Method::PATCH,
                format!("/guilds/{guild_id}/members/@me/nick", guild_id = guild_id),
            ),
            Self::RemoveGuildMember(guild_id, user_id) => (
                Method::DELETE,
                format!(
                    "/guilds/{guild_id}/members/{user_id}",
                    guild_id = guild_id,
                    user_id = user_id
                ),
            ),
            Self::GetGuildMember(guild_id, user_id) => (
                Method::GET,
                format!(
                    "/guilds/{guild_id}/members/{user_id}",
                    guild_id = guild_id,
                    user_id = user_id
                ),
            ),
            Self::ModifyGuildMember(guild_id, user_id) => (
                Method::PATCH,
                format!(
                    "/guilds/{guild_id}/members/{user_id}",
                    guild_id = guild_id,
                    user_id = user_id
                ),
            ),
            Self::AddGuildMember(guild_id, user_id) => (
                Method::PUT,
                format!(
                    "/guilds/{guild_id}/members/{user_id}",
                    guild_id = guild_id,
                    user_id = user_id
                ),
            ),
            Self::RemoveGuildMemberRole(guild_id, user_id, role_id) => (
                Method::DELETE,
                format!(
                    "/guilds/{guild_id}/members/{user_id}/roles/{role_id}",
                    guild_id = guild_id,
                    user_id = user_id,
                    role_id = role_id
                ),
            ),
            Self::AddGuildMemberRole(guild_id, user_id, role_id) => (
                Method::PUT,
                format!(
                    "/guilds/{guild_id}/members/{user_id}/roles/{role_id}",
                    guild_id = guild_id,
                    user_id = user_id,
                    role_id = role_id
                ),
            ),
            Self::GetGuildPreview(guild_id) => (
                Method::GET,
                format!("/guilds/{guild_id}/preview", guild_id = guild_id),
            ),
            Self::GetGuildPruneCount(guild_id) => (
                Method::GET,
                format!("/guilds/{guild_id}/prune", guild_id = guild_id),
            ),
            Self::BeginGuildPrune(guild_id) => (
                Method::POST,
                format!("/guilds/{guild_id}/prune", guild_id = guild_id),
            ),
            Self::GetGuildVoiceRegions(guild_id) => (
                Method::GET,
                format!("/guilds/{guild_id}/regions", guild_id = guild_id),
            ),
            Self::GetGuildRoles(guild_id) => (
                Method::GET,
                format!("/guilds/{guild_id}/roles", guild_id = guild_id),
            ),
            Self::ModifyGuildRolePositions(guild_id) => (
                Method::PATCH,
                format!("/guilds/{guild_id}/roles", guild_id = guild_id),
            ),
            Self::CreateGuildRole(guild_id) => (
                Method::POST,
                format!("/guilds/{guild_id}/roles", guild_id = guild_id),
            ),
            Self::DeleteGuildRole(guild_id, role_id) => (
                Method::DELETE,
                format!(
                    "/guilds/{guild_id}/roles/{role_id}",
                    guild_id = guild_id,
                    role_id = role_id
                ),
            ),
            Self::ModifyGuildRole(guild_id, role_id) => (
                Method::PATCH,
                format!(
                    "/guilds/{guild_id}/roles/{role_id}",
                    guild_id = guild_id,
                    role_id = role_id
                ),
            ),
            Self::GetGuildTemplates(guild_id) => (
                Method::GET,
                format!("/guilds/{guild_id}/templates", guild_id = guild_id),
            ),
            Self::CreateGuildTemplate(guild_id) => (
                Method::POST,
                format!("/guilds/{guild_id}/templates", guild_id = guild_id),
            ),
            Self::DeleteGuildTemplate(guild_id, template_code) => (
                Method::DELETE,
                format!(
                    "/guilds/{guild_id}/templates/{template_code}",
                    guild_id = guild_id,
                    template_code = template_code
                ),
            ),
            Self::ModifyGuildTemplate(guild_id, template_code) => (
                Method::PATCH,
                format!(
                    "/guilds/{guild_id}/templates/{template_code}",
                    guild_id = guild_id,
                    template_code = template_code
                ),
            ),
            Self::SyncGuildTemplate(guild_id, template_code) => (
                Method::PUT,
                format!(
                    "/guilds/{guild_id}/templates/{template_code}",
                    guild_id = guild_id,
                    template_code = template_code
                ),
            ),
            Self::GetGuildVanityURL(guild_id) => (
                Method::GET,
                format!("/guilds/{guild_id}/vanity-url", guild_id = guild_id),
            ),
            Self::GetGuildWebhooks(guild_id) => (
                Method::GET,
                format!("/guilds/{guild_id}/webhooks", guild_id = guild_id),
            ),
            Self::GetGuildWidgetSettings(guild_id) => (
                Method::GET,
                format!("/guilds/{guild_id}/widget", guild_id = guild_id),
            ),
            Self::ModifyGuildWidget(guild_id) => (
                Method::PATCH,
                format!("/guilds/{guild_id}/widget", guild_id = guild_id),
            ),
            Self::GetGuildWidget(guild_id) => (
                Method::GET,
                format!("/guilds/{guild_id}/widget.json", guild_id = guild_id),
            ),
            Self::GetGuildWidgetImage(guild_id) => (
                Method::GET,
                format!("/guilds/{guild_id}/widget.png", guild_id = guild_id),
            ),
            Self::CreateInteractionResponse(interaction_id, interaction_token) => (
                Method::POST,
                format!(
                    "/interactions/{interaction_id}/{interaction_token}/callback",
                    interaction_id = interaction_id,
                    interaction_token = interaction_token
                ),
            ),
            Self::DeleteInvite(invite_code) => (
                Method::DELETE,
                format!("/invites/{invite_code}", invite_code = invite_code),
            ),
            Self::GetInvite(invite_code) => (
                Method::GET,
                format!("/invites/{invite_code}", invite_code = invite_code),
            ),
            Self::GetCurrentApplicationInformation => {
                (Method::GET, "/oauth2/applications/@me".to_string())
            }
            Self::DeletePurchaseDiscount(sku_id, user_id) => (
                Method::DELETE,
                format!(
                    "/store/skus/{sku_id}/discounts/{user_id}",
                    sku_id = sku_id,
                    user_id = user_id
                ),
            ),
            Self::CreatePurchaseDiscount(sku_id, user_id) => (
                Method::PUT,
                format!(
                    "/store/skus/{sku_id}/discounts/{user_id}",
                    sku_id = sku_id,
                    user_id = user_id
                ),
            ),
            Self::GetCurrentUser => (Method::GET, "/users/@me".to_string()),
            Self::ModifyCurrentUser => (Method::PATCH, "/users/@me".to_string()),
            Self::GetUserDMs => (Method::GET, "/users/@me/channels".to_string()),
            Self::CreateDM => (Method::POST, "/users/@me/channels".to_string()),
            Self::CreateGroupDM => (Method::POST, "/users/@me/channels".to_string()),
            Self::GetUserConnections => (Method::GET, "/users/@me/connections".to_string()),
            Self::GetCurrentUserGuilds => (Method::GET, "/users/@me/guilds".to_string()),
            Self::LeaveGuild(guild_id) => (
                Method::DELETE,
                format!("/users/@me/guilds/{guild_id}", guild_id = guild_id),
            ),
            Self::GetUser(user_id) => (Method::GET, format!("/users/{user_id}", user_id = user_id)),
            Self::ListVoiceRegions => (Method::GET, "/voice/regions".to_string()),
            Self::CreateFollowupMessage(interaction_token) => (
                Method::POST,
                format!(
                    "/webhooks/application.id/{interaction_token}",
                    interaction_token = interaction_token
                ),
            ),
            Self::DeleteOriginalInteractionResponse(interaction_token) => (
                Method::DELETE,
                format!(
                    "/webhooks/application.id/{interaction_token}/messages/@original",
                    interaction_token = interaction_token
                ),
            ),
            Self::EditOriginalInteractionResponse(interaction_token) => (
                Method::PATCH,
                format!(
                    "/webhooks/application.id/{interaction_token}/messages/@original",
                    interaction_token = interaction_token
                ),
            ),
            Self::DeleteFollowupMessage(interaction_token, message_id) => (
                Method::DELETE,
                format!(
                    "/webhooks/application.id/{interaction_token}/messages/{message_id}",
                    interaction_token = interaction_token,
                    message_id = message_id
                ),
            ),
            Self::EditFollowupMessage(interaction_token, message_id) => (
                Method::PATCH,
                format!(
                    "/webhooks/application.id/{interaction_token}/messages/{message_id}",
                    interaction_token = interaction_token,
                    message_id = message_id
                ),
            ),
            Self::DeleteWebhook(webhook_id) => (
                Method::DELETE,
                format!("/webhooks/{webhook_id}", webhook_id = webhook_id),
            ),
            Self::GetWebhook(webhook_id) => (
                Method::GET,
                format!("/webhooks/{webhook_id}", webhook_id = webhook_id),
            ),
            Self::ModifyWebhook(webhook_id) => (
                Method::PATCH,
                format!("/webhooks/{webhook_id}", webhook_id = webhook_id),
            ),
            Self::DeleteWebhookwithToken(webhook_id, webhook_token) => (
                Method::DELETE,
                format!(
                    "/webhooks/{webhook_id}/{webhook_token}",
                    webhook_id = webhook_id,
                    webhook_token = webhook_token
                ),
            ),
            Self::GetWebhookwithToken(webhook_id, webhook_token) => (
                Method::GET,
                format!(
                    "/webhooks/{webhook_id}/{webhook_token}",
                    webhook_id = webhook_id,
                    webhook_token = webhook_token
                ),
            ),
            Self::ModifyWebhookwithToken(webhook_id, webhook_token) => (
                Method::PATCH,
                format!(
                    "/webhooks/{webhook_id}/{webhook_token}",
                    webhook_id = webhook_id,
                    webhook_token = webhook_token
                ),
            ),
            Self::ExecuteWebhook(webhook_id, webhook_token) => (
                Method::POST,
                format!(
                    "/webhooks/{webhook_id}/{webhook_token}",
                    webhook_id = webhook_id,
                    webhook_token = webhook_token
                ),
            ),
            Self::ExecuteGitHubCompatibleWebhook(webhook_id, webhook_token) => (
                Method::POST,
                format!(
                    "/webhooks/{webhook_id}/{webhook_token}/github",
                    webhook_id = webhook_id,
                    webhook_token = webhook_token
                ),
            ),
            Self::EditWebhookMessage(webhook_id, webhook_token, message_id) => (
                Method::PATCH,
                format!(
                    "/webhooks/{webhook_id}/{webhook_token}/messages/{message_id}",
                    webhook_id = webhook_id,
                    webhook_token = webhook_token,
                    message_id = message_id
                ),
            ),
            Self::ExecuteSlackCompatibleWebhook(webhook_id, webhook_token) => (
                Method::POST,
                format!(
                    "/webhooks/{webhook_id}/{webhook_token}/slack",
                    webhook_id = webhook_id,
                    webhook_token = webhook_token
                ),
            ),
        }
    }
}
