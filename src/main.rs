use rustc_hash::FxHashMap;
use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;

struct WeatherData {
    _total: f64,
    _max: f64,
    _min: f64,
    _mean: f64,
    _count: i32,
}

impl Default for WeatherData {
    fn default() -> Self {
        Self {
            _total: 0.0,
            _max: f64::MIN,
            _min: f64::MAX,
            _mean: 0.0,
            _count: 0,
        }
    }
}
impl WeatherData {
    fn update(&mut self, temperature: f64) {
        self._total += temperature;
        self._max = self._max.max(temperature);
        self._min = self._min.min(temperature);
        self._count += 1;
        self._mean = self._total / self._count as f64;
    }
}
impl fmt::Debug for WeatherData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("")
            .field("total", &self._total)
            .field("max", &self._max)
            .field("mean", &self._mean)
            .field("min", &self._min)
            .field("count", &self._count)
            .finish()
    }
}
fn main() {
    let _ = weather_parser("Measurements.csv");
}

fn weather_parser(filepath: &str) -> Result<FxHashMap<String, WeatherData>, std::io::Error> {
    println!("[Success] Operation Started");
    let now = Instant::now();
    let file = File::open(filepath)?;
    let mut reader = BufReader::with_capacity(16384, file);
    let mut buffer = Vec::with_capacity(32);
    let mut measurements: FxHashMap<String, WeatherData> = FxHashMap::default();

    while reader.read_until(b'\n', &mut buffer)? > 0 {
        let line = String::from_utf8_lossy(&buffer);
        let (station, value) = line
            .split_once(';')
            .expect("[Error] String Split Was Unsuccessful.");
        let temperature = value[0..value.len() - 1].parse::<f64>().unwrap_or(0.0);
        let entry = measurements
            .entry(station.into())
            .or_insert(WeatherData::default());
        entry.update(temperature);

        buffer.clear();
    }
    println!("[Success] Operation Finished In {:.2?}", now.elapsed());
    Ok(measurements)
}
