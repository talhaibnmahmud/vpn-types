pub mod flood {
    #[doc(hidden)]
    pub use log;

    const CALLS_INTERVAL: std::time::Duration = std::time::Duration::from_secs(5);
    const CALLS_THRESHOLD: usize = 1_000;

    pub struct DetectFlood {
        last_clear: Option<std::time::Instant>,
        counter: usize,
    }

    impl DetectFlood {
        pub const fn new() -> Self {
            DetectFlood {
                last_clear: None,
                counter: 0,
            }
        }

        pub fn bump(&mut self) -> bool {
            let now = std::time::Instant::now();
            let last_clear = self.last_clear.get_or_insert(now);

            if now.saturating_duration_since(*last_clear) >= CALLS_INTERVAL {
                self.last_clear = Some(now);
                self.counter = 0;
                false
            } else {
                self.counter = self.counter.saturating_add(1);
                self.counter == CALLS_THRESHOLD
            }
        }
    }

    impl Default for DetectFlood {
        fn default() -> Self {
            DetectFlood::new()
        }
    }

    #[macro_export]
    macro_rules! detect_flood {
        () => {{
            static FLOOD: ::std::sync::Mutex<$crate::flood::DetectFlood> =
                ::std::sync::Mutex::new($crate::flood::DetectFlood::new());
            if FLOOD.lock().unwrap().bump() {
                $crate::flood::log::warn!(
                    "Flood: {}, line {}, col {}",
                    file!(),
                    line!(),
                    column!()
                );
            }
        }};
    }
}
