use std::borrow::Cow;

use crate::{escape, Color, HashMap, Parse};

#[derive(Clone, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(::serde::Serialize, ::serde::Deserialize))]
pub struct Tags<'a> {
    pub(crate) inner: HashMap<Cow<'a, str>, Cow<'a, str>>,
}

impl<'a> std::fmt::Debug for Tags<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_map().entries(&self.inner).finish()
    }
}

impl<'a> Tags<'a> {
    pub fn get(&self, key: &str) -> Option<&str> {
        self.inner.get(key).map(|v| &**v).filter(|s| !s.is_empty())
    }

    pub fn parsed<T>(&self, key: &str) -> Option<Result<T, T::Err>>
    where
        T: std::str::FromStr,
        T::Err: std::fmt::Display,
    {
        self.get(key).map(<str>::parse)
    }

    pub fn bool(&self, key: &str) -> bool {
        self.get(key).filter(|&s| s == "1").is_some()
    }

    // tmi-ts
    // badges
    // emotes
    // room-id
    // user-id
    //
    // subscriptions?
    // clearmsg?
    // clearchat?
    //
    // reply stuff

    pub fn color(&self) -> Option<Color> {
        self.parsed("color").transpose().ok().flatten()
    }

    pub fn builder() -> TagsBuilder {
        TagsBuilder::default()
    }

    pub fn to_raw(&self) -> String {
        if self.inner.is_empty() {
            return String::new();
        }

        let cap = self
            .inner
            .iter()
            .map(|(k, v)| k.len() + escape::estimate_escape_size(v) + v.len() + 2)
            .sum::<usize>();

        self.inner
            .iter()
            .enumerate()
            .fold(String::with_capacity(cap), |mut s, (i, (k, v))| {
                if i == 0 {
                    s.push('@');
                }
                if i > 0 && i != self.inner.len() {
                    s.push(';')
                }
                s.push_str(k);
                s.push('=');
                s.push_str(&escape::escape_tag(v));
                s
            })
    }
}

impl<'a> Parse<'a> for Tags<'a> {
    type Output = Option<Self>;

    fn parse(input: &mut &'a str) -> Self::Output {
        if !input.starts_with('@') {
            return None;
        }

        let (head, tail) = input.split_once(' ')?;
        *input = tail;

        let inner = head[1..]
            .split_terminator(';')
            .flat_map(|tag| tag.split_once('='))
            .map(|(k, v)| (Cow::from(k), escape::unescape_tag(v)))
            .collect();

        Some(Self { inner })
    }
}

#[derive(Default, Debug)]
pub struct TagsBuilder(HashMap<String, String>);

impl TagsBuilder {
    pub fn add(mut self, k: impl ToString, v: impl ToString) -> Self {
        let k = k.to_string();
        let v = v.to_string();
        let v = escape::unescape_tag(&v).to_string();
        self.0.insert(k, v);
        self
    }

    pub fn finish(self) -> Tags<'static> {
        Tags {
            inner: self
                .0
                .into_iter()
                .map(|(k, v)| (Cow::from(k), Cow::from(v)))
                .collect(),
        }
    }
}
