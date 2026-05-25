use std::{env, time::Duration};

use criterion::Criterion;

#[derive(Debug, Clone, Copy)]
pub(crate) enum BenchProfile {
    Quick,
    Report,
}

impl BenchProfile {
    #[inline]
    fn current() -> Self {
        match env::var("BENCH_PROFILE").as_deref() {
            Ok("report") => Self::Report,
            Ok("quick") | Err(_) => Self::Quick,
            Ok(other) => panic!("invalid BENCH_PROFILE={other:?}; expected `quick` or `report`"),
        }
    }

    #[inline]
    fn baseline(self) -> String {
        match self {
            Self::Quick => "quick".into(),
            Self::Report => "report".into(),
        }
    }

    #[inline]
    fn criterion(self) -> Criterion {
        match self {
            Self::Quick => Criterion::default()
                .sample_size(20)
                .warm_up_time(Duration::from_millis(100))
                .measurement_time(Duration::from_millis(300))
                .nresamples(10_000)
                .without_plots()
                .save_baseline(self.baseline()),

            Self::Report => Criterion::default().save_baseline(self.baseline()),
        }
    }
}

/// Shared Criterion configuration.
///
/// `BENCH_PROFILE=report` enables the formal report configuration.
/// Missing `BENCH_PROFILE` defaults to the local quick configuration.
#[inline]
pub(crate) fn config() -> Criterion {
    BenchProfile::current().criterion()
}
