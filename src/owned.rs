pub type OwnedCapability = crate::messages::Capability<'static>;
pub type OwnedClearChat = crate::messages::ClearChat<'static>;
pub type OwnedClearChatTarget = crate::messages::ClearChatTarget<'static>;
pub type OwnedClearMsg = crate::messages::ClearMsg<'static>;
pub type OwnedGlobalUserState = crate::messages::GlobalUserState<'static>;
pub type OwnedHostTarget = crate::messages::HostTarget<'static>;
pub type OwnedHostMode = crate::messages::HostMode;
pub type OwnedIrcReady = crate::messages::IrcReady<'static>;
pub type OwnedNotice = crate::messages::Notice<'static>;
pub type OwnedPing = crate::messages::Ping<'static>;
pub type OwnedPong = crate::messages::Pong<'static>;
pub type OwnedPrivMsg = crate::messages::Privmsg<'static>;
pub type OwnedReady = crate::messages::Ready<'static>;
pub type OwnedReconnect = crate::messages::Reconnect<'static>;
pub type OwnedRoomState = crate::messages::RoomState<'static>;
pub type OwnedUserState = crate::messages::UserState<'static>;
pub type OwnedWhisper = crate::messages::Whisper<'static>;
