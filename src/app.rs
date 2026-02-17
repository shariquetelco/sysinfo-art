use crate::animations::{fire::Fire, ocean::Ocean, rain::Rain, water::Water, wind::Wind};
use crate::metrics::{Metrics, MetricsCollector};

pub struct App {
    pub metrics: Metrics,
    pub collector: MetricsCollector,
    pub ocean: Ocean,
    pub rain: Rain,
    pub fire: Fire,
    pub wind: Wind,
    pub water: Water,
    pub tick: u64,
    pub show_help: bool,
}

impl App {
    pub fn new() -> Self {
        let collector = MetricsCollector::new();
        Self {
            metrics: Metrics::default(),
            collector,
            ocean: Ocean::new(80, 10),
            rain: Rain::new(40, 10),
            fire: Fire::new(40, 10),
            wind: Wind::new(80, 10),
            water: Water::new(20, 10),
            tick: 0,
            show_help: false,
        }
    }

    pub fn update(&mut self) {
        self.tick = self.tick.wrapping_add(1);
        self.metrics = self.collector.collect();
    }

    pub fn resize_panels(
        &mut self,
        ocean_w: usize,
        ocean_h: usize,
        rain_w: usize,
        rain_h: usize,
        fire_w: usize,
        fire_h: usize,
        wind_w: usize,
        wind_h: usize,
        water_w: usize,
        water_h: usize,
    ) {
        self.ocean.resize(ocean_w, ocean_h);
        self.rain.resize(rain_w, rain_h);
        self.fire.resize(fire_w, fire_h);
        self.wind.resize(wind_w, wind_h);
        self.water.resize(water_w, water_h);
    }
}
