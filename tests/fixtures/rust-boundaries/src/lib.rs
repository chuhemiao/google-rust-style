#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(all(feature = "backend-a", feature = "backend-b"))]
compile_error!("backend-a and backend-b are mutually exclusive");

/// Parse a nonzero port, including the maximum port 65535.
pub fn parse_port(input: &str) -> Result<u16, &'static str> {
    let value = input.parse::<u16>().map_err(|_| "invalid port")?;
    if value == 0 {
        return Err("zero port");
    }
    Ok(value)
}

#[cfg(feature = "std")]
pub mod lifecycle {
    use std::{cell::Cell, future::Future};

    struct Reservation<'a>(&'a Cell<bool>);

    impl Drop for Reservation<'_> {
        fn drop(&mut self) {
            self.0.set(false);
        }
    }

    /// The caller owns and must complete or drop this future before shutdown.
    pub async fn update(busy: &Cell<bool>, response: impl Future<Output = ()>) {
        busy.set(true);
        let _reservation = Reservation(busy);
        response.await;
    }
}

#[cfg(feature = "std")]
pub mod foreign {
    use std::{marker::PhantomData, ptr::NonNull, rc::Rc, slice};

    extern "C" {
        fn fixture_new() -> *mut u8;
        fn fixture_free(pointer: *mut u8);
    }

    /// Owns four initialized immutable foreign bytes; confined to its thread.
    ///
    /// ```compile_fail,E0277
    /// use rust_skill_boundaries::foreign::Buffer;
    /// fn require_send<T: Send>() {}
    /// require_send::<Buffer>();
    /// ```
    ///
    /// ```compile_fail,E0597
    /// use rust_skill_boundaries::foreign::Buffer;
    /// let bytes;
    /// {
    ///     let buffer = Buffer::new().unwrap();
    ///     bytes = buffer.bytes(4).unwrap();
    /// }
    /// assert_eq!(bytes.len(), 4);
    /// ```
    pub struct Buffer {
        pointer: NonNull<u8>,
        _thread: PhantomData<Rc<()>>,
    }

    impl Buffer {
        pub fn new() -> Option<Self> {
            // SAFETY: the foreign constructor takes no arguments and transfers
            // a four-byte allocation, or returns null on allocation failure.
            let pointer = NonNull::new(unsafe { fixture_new() })?;
            Some(Self {
                pointer,
                _thread: PhantomData,
            })
        }

        pub fn bytes(&self, length: usize) -> Result<&[u8], &'static str> {
            if length > 4 {
                return Err("length exceeds foreign allocation");
            }
            // SAFETY: fixture_new supplies four initialized aligned bytes;
            // length is bounded above, the allocation remains live for self,
            // and the foreign contract forbids mutation during this borrow.
            Ok(unsafe { slice::from_raw_parts(self.pointer.as_ptr(), length) })
        }
    }

    impl Drop for Buffer {
        fn drop(&mut self) {
            // SAFETY: this is the unique owner; borrowing prevents drop during
            // access, and the matching foreign deallocator runs on this thread.
            unsafe { fixture_free(self.pointer.as_ptr()) };
        }
    }
}

#[cfg(all(test, feature = "std"))]
mod tests {
    use super::{foreign::Buffer, lifecycle, parse_port};
    use std::{
        cell::Cell,
        future::Future,
        pin::Pin,
        sync::Arc,
        task::{Context, Poll, Wake, Waker},
    };

    #[test]
    fn parser_normal_invalid_and_boundaries() {
        assert_eq!(parse_port("80"), Ok(80));
        assert_eq!(parse_port("1"), Ok(1));
        assert_eq!(parse_port("65535"), Ok(65535));
        for value in ["", "0", "65536", "-1", "12x"] {
            assert!(parse_port(value).is_err(), "accepted {value:?}");
        }
    }

    struct NoopWake;
    impl Wake for NoopWake {
        fn wake(self: Arc<Self>) {}
    }

    // Explicit release controls the suspension point; no sleeps or runtime.
    struct Response<'a>(&'a Cell<bool>);
    impl Future for Response<'_> {
        type Output = ();
        fn poll(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<()> {
            if self.0.get() {
                Poll::Ready(())
            } else {
                Poll::Pending
            }
        }
    }

    #[test]
    fn cancellation_after_progress_releases_state_before_shutdown() {
        let busy = Cell::new(false);
        let released = Cell::new(false);
        let waker = Waker::from(Arc::new(NoopWake));
        let mut context = Context::from_waker(&waker);
        let mut operation = Box::pin(lifecycle::update(&busy, Response(&released)));
        assert!(operation.as_mut().poll(&mut context).is_pending());
        assert!(busy.get());
        drop(operation); // the caller cancels and owns synchronous cleanup
        assert!(!busy.get(), "shutdown would leave a reservation behind");

        let mut next = Box::pin(lifecycle::update(&busy, Response(&released)));
        assert!(next.as_mut().poll(&mut context).is_pending());
        released.set(true);
        assert!(next.as_mut().poll(&mut context).is_ready());
        assert!(!busy.get());
        drop(next);
    }

    #[test]
    fn ffi_validates_lengths_and_pairs_foreign_allocation() {
        extern "C" {
            fn fixture_live() -> usize;
        }
        // SAFETY: fixture counter is read on the only test thread using FFI.
        assert_eq!(unsafe { fixture_live() }, 0);
        let buffer = Buffer::new().expect("fixture allocation");
        assert_eq!(buffer.bytes(0), Ok(&[][..]));
        assert_eq!(buffer.bytes(4), Ok(&[0, 1, 2, 3][..]));
        assert!(buffer.bytes(5).is_err());
        assert!(buffer.bytes(usize::MAX).is_err());
        drop(buffer);
        // SAFETY: no allocation or other FFI call is outstanding.
        assert_eq!(unsafe { fixture_live() }, 0);
    }
}
