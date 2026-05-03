pub struct Timer {
    curr: f64, // Elapsed time since timer start
    limit: f64, // Time at which a function is performed and curr is reset
    // Spacing
    factor_a: f64, // Factor that limit is multiplied by after function call
    factor_b: f64, // Factor that is add to limit after function call
    reset: f64, // Initial limit value
    max: f64, // Max value that limit is capped to
}

const DEFAULTMAXTIMER: f64 = 1.0;

impl Default for Timer {
    fn default() -> Self {
        Self {
            curr: 0.0,
            limit: DEFAULTMAXTIMER,
            factor_a: 1.0,
            factor_b: 0.0,
            reset: DEFAULTMAXTIMER,
            max: DEFAULTMAXTIMER,
        }
    }
}

impl Timer {
    pub fn new(elapse_time: f64) -> Timer {
        Timer {
            limit: elapse_time,
            reset: elapse_time,
            max: elapse_time,
            ..Default::default()
        }
    }

    pub fn new_exponential(base_elapse_time: f64, max_elapse_time: f64, factor: f64) -> Timer {
        Timer {
            limit: base_elapse_time,
            reset: base_elapse_time,
            max: max_elapse_time,
            factor_a: factor,
            ..Default::default()
        }
    }

    pub fn new_quadratic(base_elapse_time: f64, max_elapse_time: f64, factor: f64) -> Timer {
        Timer {
            limit: base_elapse_time,
            reset: base_elapse_time,
            max: max_elapse_time,
            factor_b: factor,
            ..Default::default()
        }
    }

    pub fn advance(&mut self, dt: f64) {
        self.curr += dt;
    }

    pub fn is_elapsed(&self) -> bool {
        self.curr > self.limit
    }

    pub fn partial_reset(&mut self) {
        self.curr = 0.0;
        self.limit *= self.factor_a;
        self.limit += self.factor_b;
        self.limit = self.limit.min(self.max);
    }

    pub fn full_reset(&mut self) {
        self.curr = 0.0;
        self.limit = self.reset;
    }
}