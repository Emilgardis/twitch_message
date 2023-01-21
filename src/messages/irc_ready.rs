use std::borrow::Cow;

use super::Message;

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct IrcReady<'a> {
    pub name: Cow<'a, str>,
    pub raw: Cow<'a, str>,
}

impl IrcReady<'_> {
    fn validate(value: &Message<'_>) -> bool {
        !value.args.is_empty()
    }
}

impl<'a> TryFrom<Message<'a>> for IrcReady<'a> {
    type Error = Message<'a>;

    fn try_from(mut value: Message<'a>) -> Result<Self, Self::Error> {
        if !Self::validate(&value) {
            return Err(value);
        }

        Ok(Self {
            name: value.args.remove(0),
            raw: value.raw,
        })
    }
}

impl<'a, 'b> TryFrom<&'b Message<'a>> for IrcReady<'a> {
    type Error = &'b Message<'a>;

    fn try_from(value: &'b Message<'a>) -> Result<Self, Self::Error> {
        if !Self::validate(value) {
            return Err(value);
        }

        Ok(Self {
            name: value.args[0].clone(),
            raw: value.raw.clone(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_util;

    #[test]
    fn irc_ready() {
        let input = ":tmi.twitch.tv 001 museun :Welcome, GLHF!\r\n";
        let raw = test_util::raw(input);
        assert_eq!(
            test_util::parse_as::<IrcReady>(input),
            IrcReady {
                name: Cow::from("museun"),
                raw,
            }
        );
    }
}
