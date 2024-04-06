use erfurt::candle::CandlesExt;
use statrs::statistics::Statistics;

use crate::Indicator;

#[derive(Debug)]
pub struct RollingEconomicDrawdown {
    pub freq: usize,
    input: Vec<f64>,
    pub values: Vec<Option<f64>>,
}

impl RollingEconomicDrawdown {
    pub fn new(freq: usize) -> Self {
        Self {
            freq,
            input: Vec::with_capacity(freq),
            values: Vec::with_capacity(freq),
        }
    }
}

impl Indicator for RollingEconomicDrawdown {
    type Input = f64;
    type Output = f64;

    fn feed(&mut self, close: Self::Input) {
        self.input.push(close);
        if self.input.len() >= self.freq {
            let mx = Statistics::max(self.input[self.input.len() - self.freq..].iter());
            if let (Some(x), true) = (self.input.last(), !mx.is_nan()) {
                self.values.push(Some(1.0 - (x / mx)));
            }
        } else {
            self.values.push(None)
        }
    }

    fn last(&self) -> Option<&Self::Output> {
        self.values.last().and_then(|v| v.as_ref())
    }

    fn iter(&self) -> Box<dyn Iterator<Item = Option<&Self::Output>> + '_> {
        Box::new(self.values.iter().map(Option::as_ref))
    }
}

pub trait RollingEconomicDrawdownExt {
    fn rolling_economic_drawndown(&self, freq: usize) -> Option<RollingEconomicDrawdown>;
}

impl<T> RollingEconomicDrawdownExt for T
where
    T: CandlesExt,
{
    fn rolling_economic_drawndown(&self, freq: usize) -> Option<RollingEconomicDrawdown> {
        if !self.close().is_empty() {
            let mut indicator = RollingEconomicDrawdown::new(freq);
            self.close().iter().for_each(|&v| indicator.feed(v));
            Some(indicator)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod test {
    use float_cmp::assert_approx_eq;

    use crate::Indicator;

    use super::RollingEconomicDrawdown;
    static XS: [f64; 10] = [
        0.003, 0.026, 0.015, -0.009, 0.014, 0.024, 0.015, 0.066, -0.014, 0.039,
    ];

    #[test]
    fn rolling_economic_drawndown() {
        let mut indicator = RollingEconomicDrawdown::new(10);
        XS.iter().for_each(|x| indicator.feed(*x));
        assert_approx_eq!(
            f64,
            0.40909090909090917,
            *indicator.last().unwrap(),
            epsilon = 0.0000001
        )
    }
}
