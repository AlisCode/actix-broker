//! messages.
use actix::prelude::*;

use broker::Broker;
use msgs::*;

/// The `BrokerSubscribe` trait has functions to register an actor's interest in different
/// messages.
pub trait BrokerSubscribe: Actor<Context = Context<Self>> {
    /// Asynchronously subscribe to a message.
    /// ```
    /// self.subscribe_async::<MessageType>(ctx);
    /// ```
    fn subscribe_async<M: BrokerMsg>(&self, ctx: &mut Self::Context)
    where
        <M as Message>::Result: Send,
        Self: Handler<M>,
    {
        let broker = Broker::from_registry();
        let recipient = ctx.address().recipient::<M>();
        broker.do_send(SubscribeAsync(recipient));
    }

    /// Synchronously subscribe to a message.
    /// ```
    /// self.subscribe_sync::<MessageType>(ctx);
    /// ```
    /// This actor will do nothing else until its interest is registered.
    /// If messages of that type have been sent to the broker previously, a copy of the latest
    /// message is sent to the calling actor after it has subscribed.
    fn subscribe_sync<M: BrokerMsg>(&self, ctx: &mut Self::Context)
    where
        <M as Message>::Result: Send,
        Self: Handler<M>,
    {
        let broker = Broker::from_registry();
        let recipient = ctx.address().recipient::<M>();

        broker
            .send(SubscribeSync(recipient))
            .into_actor(self)
            .map_err(|_, _, _| ())
            .map(move |m, _, ctx| {
                if let Some(msg) = m {
                    ctx.notify(msg);
                }
            })
            .wait(ctx);
    }
}

impl<A> BrokerSubscribe for A
where
    A: Actor<Context = Context<A>>,
{
}
