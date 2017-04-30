use std::sync::Mutex;
use std::time::{Duration, Instant};

use libmetrics::metrics::{Meter, StdMeter};
use hdrsample::Histogram;

use stats::*;

pub struct WorkerMetrics {
    pub handler: HandlerMetrics,
    pub stream: StreamMetrics,
    pub checkpointing: CheckpointingMetrics,
}

impl WorkerMetrics {
    pub fn new() -> WorkerMetrics {
        WorkerMetrics {
            handler: HandlerMetrics::new(),
            stream: StreamMetrics::new(),
            checkpointing: CheckpointingMetrics::new(),
        }
    }

    pub fn tick(&self) {
        self.handler.tick();
        self.stream.tick();
        self.checkpointing.tick();
    }

    pub fn stats(&self) -> WorkerStats {
        WorkerStats {
            handler: self.handler.stats(),
            stream: self.stream.stats(),
            checkpointing: self.checkpointing.stats(),
        }
    }
}

pub struct HandlerMetrics {
    batches_per_second: StdMeter,
    bytes_per_batch: Mutex<Histogram<u64>>,
    processing_durations: Mutex<Histogram<u64>>,
    bytes_per_second: StdMeter,
}

impl HandlerMetrics {
    pub fn new() -> HandlerMetrics {
        HandlerMetrics {
            batches_per_second: StdMeter::default(),
            bytes_per_batch: Mutex::new(Histogram::new_with_max(1_000_000, 3).unwrap()),
            processing_durations: Mutex::new(Histogram::new_with_max(10*60*1_000_000, 3).unwrap()),
            bytes_per_second: StdMeter::default(),
        }
    }

    pub fn batch_received(&self, batch: &str) {
        self.batches_per_second.mark(1);
        let bytes = batch.len() as u64;
        update_histogram(bytes, &self.bytes_per_batch);
        self.batches_per_second.mark(bytes as i64);
    }

    pub fn batch_processed(&self, start: Instant) {
        let micros = duration_to_micros(Instant::now() - start);
        update_histogram(micros, &self.processing_durations);
    }

    pub fn tick(&self) {
        self.batches_per_second.tick();
        self.bytes_per_second.tick();
    }

    pub fn stats(&self) -> HandlerStats {
        HandlerStats {
            batches_per_second: new_meter_snapshot(&self.batches_per_second),
            bytes_per_batch: new_histogram_snapshot_from_mutex(&self.bytes_per_batch),
            processing_durations: new_histogram_snapshot_from_mutex(&self.processing_durations),
            bytes_per_second: new_meter_snapshot(&self.bytes_per_second),
        }
    }
}

pub struct StreamMetrics {
    lines_per_second: StdMeter,
    bytes_per_line: Mutex<Histogram<u64>>,
    bytes_per_second: StdMeter,
    keep_alives_per_second: StdMeter,
    connection_duration: Mutex<Histogram<u64>>,
    no_connection_duration: Mutex<Histogram<u64>>,
    lines_per_connection: Mutex<Histogram<u64>>,
    batches_dropped_per_second: StdMeter,
}

impl StreamMetrics {
    pub fn new() -> StreamMetrics {
        StreamMetrics {
            lines_per_second: StdMeter::default(),
            bytes_per_line: Mutex::new(Histogram::new_with_max(1_000_000, 3).unwrap()),
            bytes_per_second: StdMeter::default(),
            keep_alives_per_second: StdMeter::default(),
            connection_duration: Mutex::new(Histogram::new_with_max(2*24*60*60, 3).unwrap()),
            no_connection_duration: Mutex::new(Histogram::new_with_max(10*60*60*1_000, 3).unwrap()),
            lines_per_connection: Mutex::new(Histogram::new_with_max(10_000_000, 3).unwrap()),
            batches_dropped_per_second: StdMeter::default(),
        }
    }

    pub fn batch_skipped(&self) {
        self.batches_dropped_per_second.mark(1);
    }

    pub fn connection_ended(&self, start: Instant, lines: u64) {
        update_histogram(lines, &self.lines_per_connection);
        update_histogram((Instant::now() - start).as_secs(),
                         &self.connection_duration);
    }

    pub fn connected(&self, no_connection_since: Instant) {
        let d = duration_to_millis(Instant::now() - no_connection_since);
        update_histogram(d, &self.no_connection_duration);
    }

    pub fn keep_alive_received(&self) {
        self.keep_alives_per_second.mark(1);
    }

    pub fn line_received(&self, line: &str) {
        self.lines_per_second.mark(1);
        let bytes = line.len() as u64;
        self.bytes_per_second.mark(bytes as i64);
        update_histogram(bytes, &self.bytes_per_line);
    }

    pub fn tick(&self) {
        self.lines_per_second.tick();
        self.bytes_per_second.tick();
        self.keep_alives_per_second.tick();
        self.batches_dropped_per_second.tick();
    }

    pub fn stats(&self) -> StreamStats {
        StreamStats {
            lines_per_second: new_meter_snapshot(&self.lines_per_second),
            bytes_per_line: new_histogram_snapshot_from_mutex(&self.bytes_per_line),
            bytes_per_second: new_meter_snapshot(&self.bytes_per_second),
            keep_alives_per_second: new_meter_snapshot(&self.keep_alives_per_second),
            lines_per_connection: new_histogram_snapshot_from_mutex(&self.lines_per_connection),
            connection_duration: new_histogram_snapshot_from_mutex(&self.connection_duration),
            no_connection_duration: new_histogram_snapshot_from_mutex(&self.connection_duration),
            batches_dropped_per_second: new_meter_snapshot(&self.batches_dropped_per_second),
        }
    }
}

pub struct CheckpointingMetrics {
    checkpoints_per_second: StdMeter,
    checkpointing_durations: Mutex<Histogram<u64>>,
    checkpointing_errors_per_second: StdMeter,
    checkpointing_failures_per_second: StdMeter,
    checkpointing_failures_durations: Mutex<Histogram<u64>>,
}

impl CheckpointingMetrics {
    pub fn new() -> CheckpointingMetrics {
        CheckpointingMetrics {
            checkpoints_per_second: StdMeter::default(),
            checkpointing_durations: Mutex::new(Histogram::new_with_max(61*60*1_000, 3).unwrap()),
            checkpointing_errors_per_second: StdMeter::default(),
            checkpointing_failures_per_second: StdMeter::default(),
            checkpointing_failures_durations: Mutex::new(Histogram::new_with_max(61*60*1_000, 3).unwrap()),
        }
    }

    pub fn checkpointed(&self, start: Instant) {
        self.checkpoints_per_second.mark(1);
        let time = duration_to_millis(Instant::now() - start);
        update_histogram(time, &self.checkpointing_durations);
    }

    pub fn checkpointing_failed(&self, start: Instant) {
        self.checkpointing_failures_per_second.mark(1);
        let time = duration_to_millis(Instant::now() - start);
        update_histogram(time, &self.checkpointing_failures_durations);
    }

    pub fn checkpointing_error(&self) {
        self.checkpointing_errors_per_second.mark(1);
    }

    pub fn tick(&self) {
        self.checkpoints_per_second.tick();
        self.checkpointing_errors_per_second.tick();
        self.checkpointing_failures_per_second.tick();
    }

    pub fn stats(&self) -> CheckpointingStats {
        CheckpointingStats {
            checkpoints_per_second: new_meter_snapshot(&self.checkpoints_per_second),
            checkpointing_durations:
                new_histogram_snapshot_from_mutex(&self.checkpointing_durations),
            checkpointing_errors_per_second:
                new_meter_snapshot(&self.checkpointing_errors_per_second),
            checkpointing_failures_per_second:
                new_meter_snapshot(&self.checkpointing_failures_per_second),
            checkpointing_failures_durations:
                new_histogram_snapshot_from_mutex(&self.checkpointing_failures_durations),
        }
    }
}

fn new_meter_snapshot(meter: &Meter) -> MeterSnapshot {
    let snapshot = meter.snapshot();
    MeterSnapshot {
        count: snapshot.count,
        one_minute_rate: snapshot.rates[0],
        five_minute_rate: snapshot.rates[1],
        fifteen_minute_rate: snapshot.rates[2],
        mean: snapshot.mean,
    }
}

fn new_histogram_snapshot(histogram: &Histogram<u64>) -> HistogramSnapshot {
    HistogramSnapshot {
        count: histogram.count(),
        min: histogram.min(),
        max: histogram.max(),
        mean: histogram.mean(),
        std_dev: histogram.stdev(),
        low: histogram.low(),
        high: histogram.high(),
        sigfig: histogram.sigfig(),
        len: histogram.len(),
        percentiles: HistogramPercentiles {
            p75: histogram.value_at_percentile(0.75),
            p95: histogram.value_at_percentile(95.0),
            p98: histogram.value_at_percentile(98.0),
            p99: histogram.value_at_percentile(99.0),
            p999: histogram.value_at_percentile(99.9),
            p9999: histogram.value_at_percentile(99.99),
        }
    }
}

fn new_histogram_snapshot_from_mutex(histogram: &Mutex<Histogram<u64>>) -> HistogramSnapshot {
    let guard = histogram.lock().unwrap();
    new_histogram_snapshot(&guard)
}

pub fn duration_to_millis(d: Duration) -> u64 {
    d.as_secs() * 1000 + d.subsec_nanos() as u64 / 1_000_000
}

fn duration_to_micros(d: Duration) -> u64 {
    d.as_secs() * 1_000_000 + d.subsec_nanos() as u64 / 1000
}

fn update_histogram(v: u64, h: &Mutex<Histogram<u64>>) {
    match h.lock() {
        Ok(mut h) => {
           let high = h.high();
           if let Err(()) = h.record(v) {
                info!("Could not update histogram with value {} because it is \
                       higher than {}. Recording the highest possible value {}.", v, high, high);
                if let Err(()) = h.record(high) {
                    warn!("Could not record the highest possible value of {}. Skipping value.", high);
                }
            }
        }
        Err(err) => warn!("Could not update histogram with value {}: {}", v, err),
    }
}