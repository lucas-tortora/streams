use alloc::{
    borrow::Cow,
    string::{String, ToString},
    vec::Vec,
};
use core::{
    convert::{TryFrom, TryInto},
    fmt::Formatter,
};
use spongos::{
    ddml::{
        commands::{sizeof, unwrap, wrap, Mask},
        io,
        types::{Bytes, NBytes},
    },
    KeccakF1600, Spongos, PRP,
};

#[derive(Clone, PartialEq, Eq, Debug, Default, Hash)]
pub struct Topic(String);

impl Topic {
    pub fn new(t: String) -> Self {
        Self(t)
    }
}

impl From<&str> for Topic {
    fn from(t: &str) -> Self {
        Self(t.to_string())
    }
}

impl From<String> for Topic {
    fn from(t: String) -> Self {
        Self(t)
    }
}

impl TryFrom<&[u8]> for Topic {
    type Error = anyhow::Error;
    fn try_from(t: &[u8]) -> Result<Self, Self::Error> {
        let topic = String::from_utf8(t.to_vec())?;
        Ok(Topic(topic))
    }
}

impl TryFrom<Vec<u8>> for Topic {
    type Error = anyhow::Error;
    fn try_from(t: Vec<u8>) -> Result<Self, Self::Error> {
        let topic = String::from_utf8(t)?;
        Ok(Topic(topic))
    }
}

impl core::fmt::Display for Topic {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", &self.0)
    }
}

impl AsRef<[u8]> for Topic {
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
    }
}

impl From<Topic> for Cow<'_, Topic> {
    fn from(topic: Topic) -> Self {
        Self::Owned(topic)
    }
}

impl<'a> From<&'a Topic> for Cow<'a, Topic> {
    fn from(topic: &'a Topic) -> Self {
        Self::Borrowed(topic)
    }
}

impl Mask<&Topic> for sizeof::Context {
    fn mask(&mut self, topic: &Topic) -> anyhow::Result<&mut Self> {
        self.mask(Bytes::new(topic))
    }
}

impl<OS, F> Mask<&Topic> for wrap::Context<OS, F>
where
    F: PRP,
    OS: io::OStream,
{
    fn mask(&mut self, topic: &Topic) -> anyhow::Result<&mut Self> {
        self.mask(Bytes::new(topic))
    }
}

impl<IS, F> Mask<&mut Topic> for unwrap::Context<IS, F>
where
    F: PRP,
    IS: io::IStream,
{
    fn mask(&mut self, topic: &mut Topic) -> anyhow::Result<&mut Self> {
        let mut topic_bytes = topic.as_ref().to_vec();
        self.mask(Bytes::new(&mut topic_bytes))?;
        *topic = topic_bytes.try_into()?;
        Ok(self)
    }
}

#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Debug, Default, Hash)]
pub struct TopicHash([u8; 16]);

impl From<&Topic> for TopicHash {
    fn from(topic: &Topic) -> Self {
        let topic_hash: [u8; 16] = Spongos::<KeccakF1600>::init().sponge(topic.as_ref());
        Self(topic_hash)
    }
}

impl AsRef<[u8]> for TopicHash {
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
    }
}

impl Mask<&TopicHash> for sizeof::Context {
    fn mask(&mut self, topic_hash: &TopicHash) -> anyhow::Result<&mut Self> {
        self.mask(NBytes::<[u8; 16]>::new(topic_hash.0))
    }
}

impl<OS, F> Mask<&TopicHash> for wrap::Context<OS, F>
where
    F: PRP,
    OS: io::OStream,
{
    fn mask(&mut self, topic_hash: &TopicHash) -> anyhow::Result<&mut Self> {
        self.mask(NBytes::<[u8; 16]>::new(topic_hash.0))
    }
}

impl<IS, F> Mask<&mut TopicHash> for unwrap::Context<IS, F>
where
    F: PRP,
    IS: io::IStream,
{
    fn mask(&mut self, topic_hash: &mut TopicHash) -> anyhow::Result<&mut Self> {
        self.mask(NBytes::<&mut [u8; 16]>::new(&mut topic_hash.0))?;
        Ok(self)
    }
}
