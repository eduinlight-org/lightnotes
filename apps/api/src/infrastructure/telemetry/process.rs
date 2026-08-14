#[cfg(unix)]
mod imp {
  use std::sync::Mutex;
  use std::time::{Duration, Instant};

  use opentelemetry::global;
  use opentelemetry_semantic_conventions::metric::{
    PROCESS_CPU_UTILIZATION, PROCESS_MEMORY_USAGE, PROCESS_MEMORY_VIRTUAL, PROCESS_UPTIME,
  };
  use sysinfo::{Pid, ProcessRefreshKind, ProcessesToUpdate, System};

  const SNAPSHOT_TTL: Duration = Duration::from_millis(500);

  #[derive(Default, Clone, Copy)]
  struct Snapshot {
    cpu_ratio: f64,
    memory_bytes: u64,
    virtual_bytes: u64,
    uptime_secs: f64,
  }

  struct Sampler {
    system: System,
    pid: Pid,
    cpus: f64,
    taken_at: Option<Instant>,
    snapshot: Snapshot,
  }

  impl Sampler {
    fn new() -> Self {
      let cpus = std::thread::available_parallelism().map(|value| value.get() as f64).unwrap_or(1.0);

      Self {
        system: System::new(),
        pid: Pid::from_u32(std::process::id()),
        cpus,
        taken_at: None,
        snapshot: Snapshot::default(),
      }
    }

    fn sample(&mut self) -> Snapshot {
      if let Some(taken_at) = self.taken_at {
        if taken_at.elapsed() < SNAPSHOT_TTL {
          return self.snapshot;
        }
      }

      self.system.refresh_processes_specifics(
        ProcessesToUpdate::Some(&[self.pid]),
        false,
        ProcessRefreshKind::nothing().with_cpu().with_memory(),
      );

      if let Some(process) = self.system.process(self.pid) {
        self.snapshot = Snapshot {
          cpu_ratio: f64::from(process.cpu_usage()) / 100.0 / self.cpus,
          memory_bytes: process.memory(),
          virtual_bytes: process.virtual_memory(),
          uptime_secs: process.run_time() as f64,
        };
      }

      self.taken_at = Some(Instant::now());
      self.snapshot
    }
  }

  pub fn register(meter_name: &'static str) {
    let meter = global::meter(meter_name);
    let sampler: &'static Mutex<Sampler> = Box::leak(Box::new(Mutex::new(Sampler::new())));

    let read = move || sampler.lock().map(|mut guard| guard.sample()).unwrap_or_default();

    meter
      .f64_observable_gauge(PROCESS_CPU_UTILIZATION)
      .with_unit("1")
      .with_description("CPU used by the process, as a fraction of the cores available to it")
      .with_callback(move |observer| observer.observe(read().cpu_ratio, &[]))
      .build();

    meter
      .u64_observable_gauge(PROCESS_MEMORY_USAGE)
      .with_unit("By")
      .with_description("Resident set size of the process")
      .with_callback(move |observer| observer.observe(read().memory_bytes, &[]))
      .build();

    meter
      .u64_observable_gauge(PROCESS_MEMORY_VIRTUAL)
      .with_unit("By")
      .with_description("Virtual memory size of the process")
      .with_callback(move |observer| observer.observe(read().virtual_bytes, &[]))
      .build();

    meter
      .f64_observable_gauge(PROCESS_UPTIME)
      .with_unit("s")
      .with_description("Seconds since the process started")
      .with_callback(move |observer| observer.observe(read().uptime_secs, &[]))
      .build();
  }
}

#[cfg(not(unix))]
mod imp {
  pub fn register(_meter_name: &'static str) {}
}

pub use imp::register;
