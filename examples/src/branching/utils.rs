use iota_streams::{
    app::message::HasLink,
    app_channels::api::{
        tangle::{
            futures::TryStreamExt,
            IntoMessages,
        },
        Transport,
    },
    core::Result,
};

pub async fn fetch_next_messages<T, S>(streamable: &mut S) -> Result<u64>
where
    T: Transport,
    S: IntoMessages<T>,
{
    let mut count = 0;
    let mut msgs = streamable.messages();
    while let Some(msg) = msgs.try_next().await? {
        count += 1;
        println!("Message exists at {}... ", &msg.link.rel());
    }
    Ok(count)
}
