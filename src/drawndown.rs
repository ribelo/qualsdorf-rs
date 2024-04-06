use crate::{Indicator, ReturnExt};

#[derive(Debug)]
pub struct Drawndown {
    pub freq: usize,
    input: Vec<f64>,
    pub values: Vec<Option<f64>>,
}

impl Drawndown {
    pub fn new(freq: usize) -> Self {
        Self {
            freq,
            input: Vec::with_capacity(freq),
            values: Vec::with_capacity(freq),
        }
    }
}

impl Indicator for Drawndown {
    type Input = f64;
    type Output = f64;

    fn feed(&mut self, ret: Self::Input) {
        self.input.push(ret);
        if self.input.len() >= self.freq {
            let mut s = 1.0;
            let mut mx = 1.0;
            let mut r = Vec::with_capacity(self.freq);
            for x in self.input[self.input.len() - self.freq..].iter() {
                let v = (1.0 + x) * s;
                mx = v.max(mx);
                s = v;
                let dr = (mx - v) / mx;
                r.push(dr);
            }
            self.values.push(r.last().copied())
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

pub trait DrawdownExt {
    fn drawdown(&self, freq: usize) -> Option<Drawndown>;
}

impl<T> DrawdownExt for T
where
    T: ReturnExt,
{
    fn drawdown(&self, freq: usize) -> Option<Drawndown> {
        let mut indicator = Drawndown::new(freq);
        if let Some(ret) = self.ret() {
            ret.iter().for_each(|v| indicator.feed(*v));
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

    use super::Drawndown;
    static XS: [f64; 10] = [
        0.003, 0.026, 0.015, -0.009, 0.014, 0.024, 0.015, 0.066, -0.014, 0.039,
    ];

    #[test]
    fn drawdown() {
        let mut indicator = Drawndown::new(10);
        XS.iter().for_each(|x| indicator.feed(*x));
        assert_approx_eq!(f64, 0.0, *indicator.last().unwrap(), epsilon = 0.0000001)
    }
}
