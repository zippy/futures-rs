use {task, Future, Poll, Stream};

use either::Either;

impl<A, B> Future for Either<A, B>
    where A: Future,
          B: Future<Output = A::Output>
{
    type Output = A::Output;

    fn poll(&mut self, cx: &mut task::Context) -> Poll<A::Output> {
        match self {
            Either::Left(a) => a.poll(cx),
            Either::Right(b) => b.poll(cx),
        }
    }
}

impl<A, B> Stream for Either<A, B>
    where A: Stream,
          B: Stream<Item = A::Item>
{
    type Item = A::Item;

    fn poll_next(&mut self, cx: &mut task::Context) -> Poll<Option<A::Item>> {
        match self {
            Either::Left(a) => a.poll_next(cx),
            Either::Right(b) => b.poll_next(cx),
        }
    }
}
