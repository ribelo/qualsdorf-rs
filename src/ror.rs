use crate::{Indicator, ReturnExt};

#[derive(Debug)]
pub struct RoR {
    pub freq: usize,
    input: Vec<f64>,
    pub values: Vec<Option<f64>>,
}

impl RoR {
    pub fn new(freq: usize) -> Self {
        RoR {
            freq,
            input: Vec::with_capacity(freq),
            values: Vec::with_capacity(freq),
        }
    }
}

impl Indicator for RoR {
    type Input = f64;
    type Output = f64;

    fn feed(&mut self, ret: Self::Input) {
        self.input.push(ret);
        if self.input.len() >= self.freq {
            let arr: Vec<f64> = self.input[self.input.len() - self.freq..]
                .iter()
                .map(|x| x + 1.0)
                .scan(1.0, |acc, x| {
                    *acc *= x;
                    Some(*acc)
                })
                .collect();
            let (x, y) = (arr.first().unwrap(), arr.last().unwrap());
            self.values.push(Some(y / x - 1.0));
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

pub trait RoRExt {
    fn ror(&self, freq: usize) -> Option<RoR>;
}

impl<T> RoRExt for T
where
    T: ReturnExt,
{
    fn ror(&self, freq: usize) -> Option<RoR> {
        let mut indicator = RoR::new(freq);
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

    use crate::{ror::RoR, Indicator};

    static XS: [f64; 10] = [
        0.003, 0.026, 0.015, -0.009, 0.014, 0.024, 0.015, 0.066, -0.014, 0.039,
    ];
    #[test]
    fn ror() {
        let mut indicator = RoR::new(10);
        XS.iter().for_each(|x| indicator.feed(*x));
        assert_approx_eq!(
            f64,
            0.187793,
            *indicator.last().unwrap(),
            epsilon = 0.000001
        );
    }
}
