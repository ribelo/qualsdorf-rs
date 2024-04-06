use statrs::statistics::Statistics;

use crate::{Indicator, ReturnExt};

#[derive(Debug)]
pub struct SharpeRatio {
    pub freq: usize,
    pub risk_free: f64,
    pub input: Vec<f64>,
    pub values: Vec<Option<f64>>,
}

impl SharpeRatio {
    pub fn new(freq: usize, risk_free: f64) -> Self {
        Self {
            freq,
            risk_free,
            input: Vec::with_capacity(freq),
            values: Vec::with_capacity(freq),
        }
    }
}

impl Indicator for SharpeRatio {
    type Input = f64;
    type Output = f64;
    fn feed(&mut self, ret: Self::Input) {
        self.input.push(ret);
        if self.input.len() >= self.freq {
            let xs = &self.input[self.input.len() - self.freq..];
            let risk_free_per_period = (1.0 + self.risk_free).powf(1.0 / self.freq as f64) - 1.0;
            let value = (xs.iter().mean() - risk_free_per_period) / xs.iter().std_dev();
            self.values.push(Some(value));
        } else {
            self.values.push(None);
        }
    }

    fn last(&self) -> Option<&Self::Output> {
        self.values.last().and_then(|v| v.as_ref())
    }

    fn iter(&self) -> Box<dyn Iterator<Item = Option<&Self::Output>> + '_> {
        Box::new(self.values.iter().map(Option::as_ref))
    }
}

pub trait SharpeRatioExt {
    fn sharpe_ratio(&self, freq: usize, risk_free: f64) -> Option<SharpeRatio>;
}

impl<T> SharpeRatioExt for T
where
    T: ReturnExt,
{
    fn sharpe_ratio(&self, freq: usize, risk_free: f64) -> Option<SharpeRatio> {
        let mut indicator = SharpeRatio::new(freq, risk_free);
        if let Some(ret) = self.ret() {
            ret.iter().for_each(|&v| indicator.feed(v));
            Some(indicator)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod test {
    use float_cmp::assert_approx_eq;

    use crate::{sharpe_ratio::SharpeRatio, Indicator};

    static XS: [f64; 10] = [
        0.003, 0.026, 0.015, -0.009, 0.014, 0.024, 0.015, 0.066, -0.014, 0.039,
    ];
    #[test]
    fn sharpe_ratio() {
        let mut indicator = SharpeRatio::new(10, 0.0);
        XS.iter().for_each(|x| indicator.feed(*x));
        assert_approx_eq!(
            f64,
            0.7705391,
            *indicator.last().unwrap(),
            epsilon = 0.0000001
        );
    }
}
