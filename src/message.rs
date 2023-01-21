use std::borrow::Cow;

use crate::{
    typed_messages::TypedMessageMarker, Error, IntoStatic, MessageKind, Parse, Prefix, Tags,
};

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct Message<'a> {
    pub raw: Cow<'a, str>,
    pub tags: Tags<'a>,
    pub prefix: Prefix<'a>,
    pub kind: MessageKind<'a>,
    pub args: Vec<Cow<'a, str>>,
    pub data: Option<Cow<'a, str>>,
}

impl<'a> From<Cow<'a, Message<'a>>> for Message<'static> {
    fn from(value: Cow<'a, Message<'a>>) -> Self {
        match value {
            Cow::Borrowed(msg) => msg.into_static(),
            Cow::Owned(msg) => msg.into_static(),
        }
    }
}

impl<'a> Message<'a>
where
    'static: 'a,
{
    pub fn as_typed_message<T>(&self) -> Option<T>
    where
        T: TypedMessageMarker<'a>,
    {
        if !T::is_kind(&self.kind) {
            return None;
        }

        T::try_from(self).ok()
    }
}

impl Message<'static> {
    #[allow(clippy::result_large_err)]
    pub fn into_typed_message<T>(self) -> Result<<T as IntoStatic>::Output, Message<'static>>
    where
        Self: 'static,
        T: TypedMessageMarker<'static>,
        <T as TryFrom<Self>>::Error: Into<Self>,
    {
        if !T::is_kind(&self.kind) {
            return Err(self);
        }

        T::try_from(self)
            .map(crate::IntoStatic::into_static)
            .map_err(Into::into)
    }
}

impl<'a> Parse<'a> for Message<'a> {
    type Output = Result<Self, Error>;

    fn parse(input: &mut &'a str) -> Self::Output {
        fn parse_args<'a>(input: &mut &'a str) -> Vec<Cow<'a, str>> {
            if let Some(tail) = input.strip_prefix(':') {
                *input = tail;
                return vec![];
            }

            if let Some(end) = input.find(" :") {
                let args = input[..end]
                    .split_ascii_whitespace()
                    .map(Cow::from)
                    .collect();
                *input = &input[end + 2..];
                return args;
            }

            let args = vec![Cow::from(*input)];
            *input = "";
            args
        }

        fn parse_data<'a>(input: &mut &'a str) -> Option<Cow<'a, str>> {
            (!input.is_empty()).then(|| Cow::from(std::mem::take(input)))
        }

        Ok(Self {
            raw: Cow::from(&**input),
            tags: Tags::parse(input).unwrap_or_default(),
            prefix: Prefix::parse(input),
            kind: MessageKind::parse(input)?,
            args: parse_args(input),
            data: parse_data(input),
        })
    }
}

#[derive(Debug)]
#[non_exhaustive]
pub enum PrivmsgBuilderError {
    MissingSender,
    MissingChannel,
    MissingData,
}

impl std::fmt::Display for PrivmsgBuilderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MissingSender => f.write_str("Missing sender"),
            Self::MissingChannel => f.write_str("Missing channel"),
            Self::MissingData => f.write_str("Missing data"),
        }
    }
}

impl std::error::Error for PrivmsgBuilderError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

#[derive(Default)]
pub struct PrivmsgBuilder {
    tags: Option<Tags<'static>>,
    sender: Option<Cow<'static, str>>,
    channel: Option<Cow<'static, str>>,
    data: Option<Cow<'static, str>>,
}

impl PrivmsgBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn tags(mut self, tags: Tags<'_>) -> Self {
        self.tags.replace(tags.into_static());
        self
    }

    pub fn sender(mut self, sender: &str) -> Self {
        self.sender.replace(sender.into_static());
        self
    }

    pub fn channel(mut self, channel: &str) -> Self {
        self.channel.replace(Cow::from(channel.to_string()));
        self
    }

    pub fn data(mut self, data: &str) -> Self {
        self.data.replace(Cow::from(data.to_string()));
        self
    }

    pub fn finish(self) -> Result<Message<'static>, PrivmsgBuilderError> {
        let tags = self.tags.unwrap_or_default();

        let get =
            |field: Option<Cow<'static, str>>, err| field.filter(|s| !s.is_empty()).ok_or(err);

        let prefix = get(self.sender, PrivmsgBuilderError::MissingSender)?;
        let channel = get(self.channel, PrivmsgBuilderError::MissingChannel)?;
        let data = get(self.data, PrivmsgBuilderError::MissingData)?;

        let raw = format!(
            "{tags}{space}:{prefix}!{prefix}@{prefix}.tmi.twitch.tv PRIVMSG {channel} :{data}\r\n",
            tags = tags.to_raw(),
            space = if tags.inner.is_empty() { "" } else { " " },
            prefix = prefix,
            channel = channel,
            data = data,
        );

        Ok(Message {
            raw: Cow::from(raw),
            tags,
            prefix: Prefix::User {
                name: prefix.clone(),
                user: prefix.clone(),
                host: prefix,
            },
            kind: MessageKind::PrivMsg,
            args: vec![channel],
            data: Some(data),
        })
    }
}
