use statrs::statistics::Statistics;

use crate::{Indicator, ReturnExt};

#[derive(Debug)]
pub struct Std {
    pub freq: usize,
    input: Vec<f64>,
    pub values: Vec<Option<f64>>,
}

impl Std {
    pub fn new(freq: usize) -> Self {
        Self {
            freq,
            input: Vec::with_capacity(freq),
            values: Vec::with_capacity(freq),
        }
    }
}

impl Indicator for Std {
    type Input = f64;
    type Output = f64;

    fn feed(&mut self, ret: Self::Input) {
        self.input.push(ret);
        if self.input.len() >= self.freq {
            let value = self.input[self.input.len() - self.freq..].iter().std_dev();
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

pub trait StdExt {
    fn std(&self, freq: usize) -> Option<Std>;
}

impl<T> StdExt for T
where
    T: ReturnExt,
{
    fn std(&self, freq: usize) -> Option<Std> {
        if let Some(ret) = self.ret() {
            let mut indicator = Std::new(freq);
            ret.iter().for_each(|&v| indicator.feed(v));
            Some(indicator)
        } else {
            None
        }
    }
}
