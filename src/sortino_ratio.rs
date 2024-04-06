use statrs::statistics::Statistics;

use crate::{downside_risk::DownsideRisk, Indicator, ReturnExt};

#[derive(Debug)]
pub struct SortinoRatio {
    pub freq: usize,
    pub risk_free: f64,
    pub mar: f64,
    input: Vec<f64>,
    downside_risk: DownsideRisk,
    pub values: Vec<Option<f64>>,
}

impl SortinoRatio {
    pub fn new(freq: usize, risk_free: f64, mar: f64) -> Self {
        Self {
            freq,
            risk_free,
            mar,
            input: Vec::with_capacity(freq),
            downside_risk: DownsideRisk::new(freq, mar),
            values: Vec::with_capacity(freq),
        }
    }
}

impl Indicator for SortinoRatio {
    type Input = f64;
    type Output = f64;
    fn feed(&mut self, ret: Self::Input) {
        self.downside_risk.feed(ret);
        self.input.push(ret);
        if self.input.len() >= self.freq {
            let downside_risk = self.downside_risk.last().unwrap();
            let mean = self.input[self.input.len() - self.freq..].iter().mean();
            self.values
                .push(Some((mean - self.risk_free) / downside_risk));
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

pub trait SortinoRatioExt {
    fn sortino_ratio(&self, freq: usize, risk_free: f64, mar: f64) -> Option<SortinoRatio>;
}

impl<T> SortinoRatioExt for T
where
    T: ReturnExt,
{
    fn sortino_ratio(&self, freq: usize, risk_free: f64, mar: f64) -> Option<SortinoRatio> {
        let mut indicator = SortinoRatio::new(freq, risk_free, mar);
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

    use crate::{sortino_ratio::SortinoRatio, Indicator};

    static XS: [f64; 10] = [
        0.003, 0.026, 0.015, -0.009, 0.014, 0.024, 0.015, 0.066, -0.014, 0.039,
    ];
    #[test]
    fn sortino_ratio() {
        let mut indicator = SortinoRatio::new(10, 0.0, 0.0);
        XS.iter().for_each(|x| indicator.feed(*x));
        assert_approx_eq!(
            f64,
            3.401051,
            *indicator.last().unwrap(),
            epsilon = 0.0000001
        );
    }
}
