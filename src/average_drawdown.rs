use statrs::statistics::Statistics;

use crate::{continuous_drawdown::ContinousDrawdown, Indicator, ReturnExt};

#[derive(Debug)]
pub struct AverageDrawdown {
    pub freq: usize,
    continuous_drawdown: ContinousDrawdown,
    pub values: Vec<Option<f64>>,
}

impl AverageDrawdown {
    pub fn new(freq: usize) -> Self {
        Self {
            freq,
            continuous_drawdown: ContinousDrawdown::new(freq),
            values: Vec::with_capacity(freq),
        }
    }
}

impl Indicator for AverageDrawdown {
    type Input = f64;
    type Output = f64;

    fn feed(&mut self, ret: Self::Input) {
        self.continuous_drawdown.feed(ret);
        if let Some(xs) = &self.continuous_drawdown.last() {
            let value = xs.iter().mean();
            self.values.push(Some(value));
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

pub trait AverageDrawdownExt {
    fn average_drawdown(&self, freq: usize) -> Option<AverageDrawdown>;
}

impl<T> AverageDrawdownExt for T
where
    T: ReturnExt,
{
    fn average_drawdown(&self, freq: usize) -> Option<AverageDrawdown> {
        let mut indicator = AverageDrawdown::new(freq);
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
    use super::AverageDrawdown;
    use crate::Indicator;
    use float_cmp::assert_approx_eq;

    static XS: [f64; 10] = [
        0.003, 0.026, 0.015, -0.009, 0.014, 0.024, 0.015, 0.066, -0.014, 0.039,
    ];

    #[test]
    fn average_drawdown() {
        let mut indicator = AverageDrawdown::new(10);
        XS.iter().for_each(|x| indicator.feed(*x));
        dbg!(indicator.last().unwrap());
        assert_approx_eq!(f64, 0.0115, *indicator.last().unwrap(), epsilon = 0.0000001)
    }
}
