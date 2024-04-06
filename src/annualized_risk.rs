use statrs::statistics::Statistics;

use crate::{Indicator, ReturnExt};

#[derive(Debug)]
pub struct AnnualizedRisk {
    pub freq: usize,
    input: Vec<f64>,
    pub values: Vec<Option<f64>>,
}

impl AnnualizedRisk {
    pub fn new(freq: usize) -> Self {
        Self {
            freq,
            input: Vec::with_capacity(freq),
            values: Vec::with_capacity(freq),
        }
    }
}

impl Indicator for AnnualizedRisk {
    type Input = f64;
    type Output = f64;

    fn feed(&mut self, ret: Self::Input) {
        self.input.push(ret);
        if self.input.len() >= self.freq {
            let value = self.input[self.input.len() - self.freq..].iter().std_dev()
                * (self.freq as f64).sqrt();
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

pub trait AnnualizedRiskExt {
    fn annualized_risk(&self, freq: usize) -> Option<AnnualizedRisk>;
}

impl<T> AnnualizedRiskExt for T
where
    T: ReturnExt,
{
    fn annualized_risk(&self, freq: usize) -> Option<AnnualizedRisk> {
        let mut indicator = AnnualizedRisk::new(freq);
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
    use crate::{annualized_risk::AnnualizedRisk, Indicator};
    use float_cmp::assert_approx_eq;

    static XS: [f64; 10] = [
        0.003, 0.026, 0.015, -0.009, 0.014, 0.024, 0.015, 0.066, -0.014, 0.039,
    ];
    #[test]
    fn annualized_risk() {
        let mut indicator = AnnualizedRisk::new(10);
        XS.iter().for_each(|x| indicator.feed(*x));
        assert_approx_eq!(
            f64,
            0.07346125206907078,
            *indicator.last().unwrap(),
            epsilon = 0.0000001
        );
    }
}
