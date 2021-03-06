pub enum Parameters {
    WSUrl {
        v: u32,
        encoding: &'static str,
        compress: &'static str,
    },
    GetGuildAuditLog {
        user_id: Option<u64>,
        action_type: Option<u32>,
        before: Option<u64>,
        limit: Option<u32>,
    },
    GetChannelMessages {
        around: Option<u64>,
        before: Option<u64>,
        after: Option<u64>,
        limit: Option<u32>,
    },
    GetReactions {
        before: Option<u64>,
        after: Option<u64>,
        limit: Option<u32>,
    },
    GetGuild {
        with_counts: Option<bool>,
    },
    ListGuildMembers {
        limit: Option<u32>,
        after: Option<u64>,
    },
    GetGuildPruneCount {
        days: Option<u32>,
        include_roles: Option<String>,
    },
    GetGuildWidgetImage {
        style: Option<String>,
    },
    GetInivte {
        with_counts: Option<bool>,
    },
    GetCurrentUserGuilds {
        before: Option<u64>,
        after: Option<u64>,
        limit: Option<u32>,
    },
    ExecuteWebhook {
        wait: Option<bool>,
    },
    ExecuteSlackCompatibleWebhook {
        wait: Option<bool>,
    },
    ExecuteGitHubCompatibleWebhook {
        wait: Option<bool>,
    },
}
