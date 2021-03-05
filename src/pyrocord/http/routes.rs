use reqwest::Method;
use pyrocord_route_derive::Routes;

#[derive(Routes)]
pub enum Route {
    #[route(GET, "/applications/{application_id}/commands")]
    GetGlobalApplicationCommands(u64),

    #[route(POST, "/applications/{application_id}/commands")]
    CreateGlobalApplicationCommand(u64),

    #[route(DELETE, "/applications/{application_id}/commands/{command_id}")]
    DeleteGlobalApplicationCommand(u64, u64),

    #[route(PATCH, "/applications/{application_id}/commands/{command_id}")]
    EditGlobalApplicationCommand(u64, u64),

    #[route(GET, "/applications/{application_id}/entitlements")]
    GetEntitlements(u64),

    #[route(DELETE, "/applications/{application_id}/entitlements/{entitlement_id}")]
    DeleteTestEntitlement(u64, u64),

    #[route(GET, "/applications/{application_id}/entitlements/{entitlement_id}")]
    GetEntitlement(u64, u64),

    #[route(POST, "/applications/{application_id}/entitlements/{entitlement_id}/consume")]
    ConsumeSKU(u64, u64),

    #[route(GET, "/applications/{application_id}/guilds/{guild_id}/commands")]
    GetGuildApplicationCommands(u64, u64),

    #[route(POST, "/applications/{application_id}/guilds/{guild_id}/commands")]
    CreateGuildApplicationCommand(u64, u64),

    #[route(DELETE, "/applications/{application_id}/guilds/{guild_id}/commands/{command_id}")]
    DeleteGuildApplicationCommand(u64, u64, u64),

    #[route(PATCH, "/applications/{application_id}/guilds/{guild_id}/commands/{command_id}")]
    EditGuildApplicationCommand(u64, u64, u64),

    #[route(GET, "/applications/{application_id}/skus")]
    GetSKUs(u64),

    #[route(DELETE, "/channels/{channel_id}")]
    DeleteOrCloseChannel(u64),

    #[route(GET, "/channels/{channel_id}")]
    GetChannel(u64),

    #[route(PATCH, "/channels/{channel_id}")]
    ModifyChannel(u64),

    #[route(POST, "/channels/{channel_id}/followers")]
    FollowNewsChannel(u64),

    #[route(GET, "/channels/{channel_id}/invites")]
    GetChannelInvites(u64),

    #[route(POST, "/channels/{channel_id}/invites")]
    CreateChannelInvite(u64),

    #[route(GET, "/channels/{channel_id}/messages")]
    GetChannelMessages(u64),

    #[route(POST, "/channels/{channel_id}/messages")]
    CreateMessage(u64),

    #[route(POST, "/channels/{channel_id}/messages/bulk-delete")]
    BulkDeleteMessages(u64),

    #[route(DELETE, "/channels/{channel_id}/messages/{message_id}")]
    DeleteMessage(u64, u64),

    #[route(GET, "/channels/{channel_id}/messages/{message_id}")]
    GetChannelMessage(u64, u64),

    #[route(PATCH, "/channels/{channel_id}/messages/{message_id}")]
    EditMessage(u64, u64),

    #[route(POST, "/channels/{channel_id}/messages/{message_id}/crosspost")]
    CrosspostMessage(u64, u64),

    #[route(DELETE, "/channels/{channel_id}/messages/{message_id}/reactions")]
    DeleteAllReactions(u64, u64),

    #[route(DELETE, "/channels/{channel_id}/messages/{message_id}/reactions/{emoji}")]
    DeleteAllReactionsforEmoji(u64, u64, String),

    #[route(GET, "/channels/{channel_id}/messages/{message_id}/reactions/{emoji}")]
    GetReactions(u64, u64, String),

    #[route(DELETE, "/channels/{channel_id}/messages/{message_id}/reactions/{emoji}/@me")]
    DeleteOwnReaction(u64, u64, String),

    #[route(PUT, "/channels/{channel_id}/messages/{message_id}/reactions/{emoji}/@me")]
    CreateReaction(u64, u64, String),

    #[route(DELETE, "/channels/{channel_id}/messages/{message_id}/reactions/{emoji}/{user_id}")]
    DeleteUserReaction(u64, u64, String, u64),

    #[route(DELETE, "/channels/{channel_id}/permissions/{overwrite_id}")]
    DeleteChannelPermission(u64, u64),

    #[route(PUT, "/channels/{channel_id}/permissions/{overwrite_id}")]
    EditChannelPermissions(u64, u64),

    #[route(GET, "/channels/{channel_id}/pins")]
    GetPinnedMessages(u64),

    #[route(DELETE, "/channels/{channel_id}/pins/{message_id}")]
    DeletePinnedChannelMessage(u64, u64),

    #[route(PUT, "/channels/{channel_id}/pins/{message_id}")]
    AddPinnedChannelMessage(u64, u64),

    #[route(DELETE, "/channels/{channel_id}/recipients/{user_id}")]
    GroupDMRemoveRecipient(u64, u64),

    #[route(PUT, "/channels/{channel_id}/recipients/{user_id}")]
    GroupDMAddRecipient(u64, u64),

    #[route(POST, "/channels/{channel_id}/typing")]
    TriggerTypingIndicator(u64),

    #[route(GET, "/channels/{channel_id}/webhooks")]
    GetChannelWebhooks(u64),

    #[route(POST, "/channels/{channel_id}/webhooks")]
    CreateWebhook(u64),

    #[route(GET, "/gateway")]
    GetGateway,

    #[route(GET, "/gateway/bot")]
    GetGatewayBot,

    #[route(POST, "/guilds")]
    CreateGuild,

    #[route(GET, "/guilds/templates/{template_code}")]
    GetTemplate(String),

    #[route(POST, "/guilds/templates/{template_code}")]
    CreateGuildfromTemplate(String),

    #[route(DELETE, "/guilds/{guild_id}")]
    DeleteGuild(u64),

    #[route(GET, "/guilds/{guild_id}")]
    GetGuild(u64),

    #[route(PATCH, "/guilds/{guild_id}")]
    ModifyGuild(u64),

    #[route(GET, "/guilds/{guild_id}/audit-logs")]
    GetGuildAuditLog(u64),

    #[route(GET, "/guilds/{guild_id}/bans")]
    GetGuildBans(u64),

    #[route(DELETE, "/guilds/{guild_id}/bans/{user_id}")]
    RemoveGuildBan(u64, u64),

    #[route(GET, "/guilds/{guild_id}/bans/{user_id}")]
    GetGuildBan(u64, u64),

    #[route(PUT, "/guilds/{guild_id}/bans/{user_id}")]
    CreateGuildBan(u64, u64),

    #[route(GET, "/guilds/{guild_id}/channels")]
    GetGuildChannels(u64),

    #[route(PATCH, "/guilds/{guild_id}/channels")]
    ModifyGuildChannelPositions(u64),

    #[route(POST, "/guilds/{guild_id}/channels")]
    CreateGuildChannel(u64),

    #[route(GET, "/guilds/{guild_id}/emojis")]
    ListGuildEmojis(u64),

    #[route(POST, "/guilds/{guild_id}/emojis")]
    CreateGuildEmoji(u64),

    #[route(DELETE, "/guilds/{guild_id}/emojis/{emoji_id}")]
    DeleteGuildEmoji(u64, u64),

    #[route(GET, "/guilds/{guild_id}/emojis/{emoji_id}")]
    GetGuildEmoji(u64, u64),

    #[route(PATCH, "/guilds/{guild_id}/emojis/{emoji_id}")]
    ModifyGuildEmoji(u64, u64),

    #[route(GET, "/guilds/{guild_id}/integrations")]
    GetGuildIntegrations(u64),

    #[route(POST, "/guilds/{guild_id}/integrations")]
    CreateGuildIntegration(u64),

    #[route(DELETE, "/guilds/{guild_id}/integrations/{integration_id}")]
    DeleteGuildIntegration(u64, u64),

    #[route(PATCH, "/guilds/{guild_id}/integrations/{integration_id}")]
    ModifyGuildIntegration(u64, u64),

    #[route(POST, "/guilds/{guild_id}/integrations/{integration_id}/sync")]
    SyncGuildIntegration(u64, u64),

    #[route(GET, "/guilds/{guild_id}/invites")]
    GetGuildInvites(u64),

    #[route(GET, "/guilds/{guild_id}/members")]
    ListGuildMembers(u64),

    #[route(PATCH, "/guilds/{guild_id}/members/@me/nick")]
    ModifyCurrentUserNick(u64),

    #[route(DELETE, "/guilds/{guild_id}/members/{user_id}")]
    RemoveGuildMember(u64, u64),

    #[route(GET, "/guilds/{guild_id}/members/{user_id}")]
    GetGuildMember(u64, u64),

    #[route(PATCH, "/guilds/{guild_id}/members/{user_id}")]
    ModifyGuildMember(u64, u64),

    #[route(PUT, "/guilds/{guild_id}/members/{user_id}")]
    AddGuildMember(u64, u64),

    #[route(DELETE, "/guilds/{guild_id}/members/{user_id}/roles/{role_id}")]
    RemoveGuildMemberRole(u64, u64, u64),

    #[route(PUT, "/guilds/{guild_id}/members/{user_id}/roles/{role_id}")]
    AddGuildMemberRole(u64, u64, u64),

    #[route(GET, "/guilds/{guild_id}/preview")]
    GetGuildPreview(u64),

    #[route(GET, "/guilds/{guild_id}/prune")]
    GetGuildPruneCount(u64),

    #[route(POST, "/guilds/{guild_id}/prune")]
    BeginGuildPrune(u64),

    #[route(GET, "/guilds/{guild_id}/regions")]
    GetGuildVoiceRegions(u64),

    #[route(GET, "/guilds/{guild_id}/roles")]
    GetGuildRoles(u64),

    #[route(PATCH, "/guilds/{guild_id}/roles")]
    ModifyGuildRolePositions(u64),

    #[route(POST, "/guilds/{guild_id}/roles")]
    CreateGuildRole(u64),

    #[route(DELETE, "/guilds/{guild_id}/roles/{role_id}")]
    DeleteGuildRole(u64, u64),

    #[route(PATCH, "/guilds/{guild_id}/roles/{role_id}")]
    ModifyGuildRole(u64, u64),

    #[route(GET, "/guilds/{guild_id}/templates")]
    GetGuildTemplates(u64),

    #[route(POST, "/guilds/{guild_id}/templates")]
    CreateGuildTemplate(u64),

    #[route(DELETE, "/guilds/{guild_id}/templates/{template_code}")]
    DeleteGuildTemplate(u64, String),

    #[route(PATCH, "/guilds/{guild_id}/templates/{template_code}")]
    ModifyGuildTemplate(u64, String),

    #[route(PUT, "/guilds/{guild_id}/templates/{template_code}")]
    SyncGuildTemplate(u64, String),

    #[route(GET, "/guilds/{guild_id}/vanity-url")]
    GetGuildVanityURL(u64),

    #[route(GET, "/guilds/{guild_id}/webhooks")]
    GetGuildWebhooks(u64),

    #[route(GET, "/guilds/{guild_id}/widget")]
    GetGuildWidgetSettings(u64),

    #[route(PATCH, "/guilds/{guild_id}/widget")]
    ModifyGuildWidget(u64),

    #[route(GET, "/guilds/{guild_id}/widget.json")]
    GetGuildWidget(u64),

    #[route(GET, "/guilds/{guild_id}/widget.png")]
    GetGuildWidgetImage(u64),

    #[route(POST, "/interactions/{interaction_id}/{interaction_token}/callback")]
    CreateInteractionResponse(u64, String),

    #[route(DELETE, "/invites/{invite_code}")]
    DeleteInvite(String),

    #[route(GET, "/invites/{invite_code}")]
    GetInvite(String),

    #[route(GET, "/oauth2/applications/@me")]
    GetCurrentApplicationInformation,

    #[route(DELETE, "/store/skus/{sku_id}/discounts/{user_id}")]
    DeletePurchaseDiscount(u64, u64),

    #[route(PUT, "/store/skus/{sku_id}/discounts/{user_id}")]
    CreatePurchaseDiscount(u64, u64),

    #[route(GET, "/users/@me")]
    GetCurrentUser,

    #[route(PATCH, "/users/@me")]
    ModifyCurrentUser,

    #[route(GET, "/users/@me/channels")]
    GetUserDMs,

    #[route(POST, "/users/@me/channels")]
    CreateDM,

    #[route(POST, "/users/@me/channels")]
    CreateGroupDM,

    #[route(GET, "/users/@me/connections")]
    GetUserConnections,

    #[route(GET, "/users/@me/guilds")]
    GetCurrentUserGuilds,

    #[route(DELETE, "/users/@me/guilds/{guild_id}")]
    LeaveGuild(u64),

    #[route(GET, "/users/{user_id}")]
    GetUser(u64),

    #[route(GET, "/voice/regions")]
    ListVoiceRegions,

    #[route(POST, "/webhooks/application_id/{interaction_token}")]
    CreateFollowupMessage(String),

    #[route(DELETE, "/webhooks/application_id/{interaction_token}/messages/@original")]
    DeleteOriginalInteractionResponse(String),

    #[route(PATCH, "/webhooks/application_id/{interaction_token}/messages/@original")]
    EditOriginalInteractionResponse(String),

    #[route(DELETE, "/webhooks/application_id/{interaction_token}/messages/{message_id}")]
    DeleteFollowupMessage(String, u64),

    #[route(PATCH, "/webhooks/application_id/{interaction_token}/messages/{message_id}")]
    EditFollowupMessage(String, u64),

    #[route(DELETE, "/webhooks/{webhook_id}")]
    DeleteWebhook(u64),

    #[route(GET, "/webhooks/{webhook_id}")]
    GetWebhook(u64),

    #[route(PATCH, "/webhooks/{webhook_id}")]
    ModifyWebhook(u64),

    #[route(DELETE, "/webhooks/{webhook_id}/{webhook_token}")]
    DeleteWebhookwithToken(u64, String),

    #[route(GET, "/webhooks/{webhook_id}/{webhook_token}")]
    GetWebhookwithToken(u64, String),

    #[route(PATCH, "/webhooks/{webhook_id}/{webhook_token}")]
    ModifyWebhookwithToken(u64, String),

    #[route(POST, "/webhooks/{webhook_id}/{webhook_token}")]
    ExecuteWebhook(u64, String),

    #[route(POST, "/webhooks/{webhook_id}/{webhook_token}/github")]
    ExecuteGitHubCompatibleWebhook(u64, String),

    #[route(PATCH, "/webhooks/{webhook_id}/{webhook_token}/messages/{message_id}")]
    EditWebhookMessage(u64, String, u64),

    #[route(POST, "/webhooks/{webhook_id}/{webhook_token}/slack")]
    ExecuteSlackCompatibleWebhook(u64, String),
}
