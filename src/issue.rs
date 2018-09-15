use actix::prelude::*;

use broker::Broker;
use msgs::*;

/// The `BrokerIssue` provides functions to issue messages to any subscribers.
pub trait BrokerIssue: Actor<Context = Context<Self>> {
    /// Asynchronously issue a message.
    /// ```
    /// self.issue_async(MessageType);
    /// ```
    fn issue_async<M: BrokerMsg>(&self, msg: M)
    where
        <M as Message>::Result: Send,
    {
        let broker = Broker::from_registry();
        broker.do_send(IssueAsync(msg));
    }

    /// Synchronously issue a message.
    /// ```
    /// self.issue_sync(MessageType, ctx);
    /// ```
    /// This also causes the broker to synchronously forward those messages on to any subscribers
    /// before handling any other messages.
    fn issue_sync<M: BrokerMsg>(&self, msg: M, ctx: &mut Self::Context)
    where
        <M as Message>::Result: Send,
    {
        let broker = Broker::from_registry();
        broker
            .send(IssueSync(msg))
            .into_actor(self)
            .map_err(|_, _, _| ())
            .map(|_, _, _| ())
            .wait(ctx);
    }
}

impl<A> BrokerIssue for A
where
    A: Actor<Context = Context<A>>,
{
}
