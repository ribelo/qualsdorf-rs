use crate::{Indicator, ReturnExt};

#[derive(Debug)]
pub struct ContinousDrawdown {
    pub freq: usize,
    input: Vec<f64>,
    pub values: Vec<Option<Vec<f64>>>,
}

impl ContinousDrawdown {
    pub fn new(freq: usize) -> Self {
        Self {
            freq,
            input: Vec::with_capacity(freq),
            values: Vec::with_capacity(freq),
        }
    }
}

impl Indicator for ContinousDrawdown {
    type Input = f64;
    type Output = Vec<f64>;

    fn feed(&mut self, ret: Self::Input) {
        self.input.push(ret);
        if self.input.len() >= self.freq {
            let mut xs = Vec::with_capacity(self.freq);
            let mut s = 1.0;
            for (i, &x) in self.input[self.input.len() - self.freq..]
                .iter()
                .enumerate()
            {
                if i == 0 && x < 0.0 {
                    s = x + 1.0;
                    continue;
                } else if i == 0 && x > 0.0 {
                    s = 1.0;
                    continue;
                } else if i > 0 && x < 0.0 {
                    s *= x + 1.0;
                    continue;
                } else if i > 0 && x > 0.0 {
                    let dd = 1.0 - s;
                    if dd != 0.0 {
                        xs.push(dd);
                        s = 1.0;
                    }
                };
            }
            if s < 1.0 {
                let dd = 1.0 - s;
                if dd != 0.0 {
                    xs.push(dd)
                }
            }
            self.values.push(Some(xs));
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

pub trait ContinousDrawdownExt {
    fn continuous_drawdown(&self, freq: usize) -> Option<ContinousDrawdown>;
}

impl<T> ContinousDrawdownExt for T
where
    T: ReturnExt,
{
    fn continuous_drawdown(&self, freq: usize) -> Option<ContinousDrawdown> {
        let mut indicator = ContinousDrawdown::new(freq);
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

    use crate::Indicator;

    use super::ContinousDrawdown;
    static XS: [f64; 10] = [
        0.003, 0.026, 0.015, -0.009, 0.014, 0.024, 0.015, 0.066, -0.014, 0.039,
    ];

    #[test]
    fn drawdown() {
        let mut indicator = ContinousDrawdown::new(10);
        XS.iter().for_each(|x| indicator.feed(*x));
        let valid = [0.009, 0.014];
        valid
            .iter()
            .zip(indicator.last().unwrap().iter())
            .for_each(|(x, y)| assert_approx_eq!(f64, *x, *y, epsilon = 0.0000001))
    }
}
