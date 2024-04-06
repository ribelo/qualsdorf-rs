use crate::{mode, Indicator, ReturnExt};
use statrs::{self, statistics::Statistics};

#[derive(Debug)]
pub struct AnnualizedReturn<T> {
    pub mode: T,
    pub freq: usize,
    input: Vec<f64>,
    pub values: Vec<Option<f64>>,
}

impl<T> AnnualizedReturn<T> {
    pub fn new(mode: T, freq: usize) -> Self {
        AnnualizedReturn {
            mode,
            freq,
            input: Vec::with_capacity(freq),
            values: Vec::with_capacity(freq),
        }
    }
}

impl Indicator for AnnualizedReturn<mode::Geometric> {
    type Input = f64;
    type Output = f64;
    fn feed(&mut self, ret: Self::Input) {
        self.input.push(ret);
        if self.input.len() >= self.freq {
            let n = self.input.len().min(self.freq);
            let ret = self.input[self.input.len() - self.freq..]
                .iter()
                .map(|x| x + 1.0)
                .fold(1.0, |acc, x| acc * x);
            let annret = ret.powf(self.freq as f64 / n as f64) - 1.0;
            self.values.push(Some(annret));
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

pub trait AnnualizedReturnExt<T> {
    fn annualized_return(&self, mode: T, freq: usize) -> Option<AnnualizedReturn<T>>;
}

impl<T> AnnualizedReturnExt<mode::Geometric> for T
where
    T: ReturnExt,
{
    fn annualized_return(
        &self,
        mode: mode::Geometric,
        freq: usize,
    ) -> Option<AnnualizedReturn<mode::Geometric>> {
        let mut indicator = AnnualizedReturn::new(mode, freq);
        if let Some(ret) = self.ret() {
            ret.iter().for_each(|&v| indicator.feed(v));
            Some(indicator)
        } else {
            None
        }
    }
}

impl Indicator for AnnualizedReturn<mode::Simple> {
    type Input = f64;
    type Output = f64;
    fn feed(&mut self, ret: Self::Input) {
        self.input.push(ret);
        if self.input.len() >= self.freq {
            let mean = self.input[self.input.len() - self.freq..].iter().mean();
            self.values.push(Some(mean * self.freq as f64));
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

impl<T> AnnualizedReturnExt<mode::Simple> for T
where
    T: ReturnExt,
{
    fn annualized_return(
        &self,
        mode: mode::Simple,
        freq: usize,
    ) -> Option<AnnualizedReturn<mode::Simple>> {
        let mut indicator = AnnualizedReturn::new(mode, freq);
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
    use crate::{
        annualized_return::{mode, AnnualizedReturn},
        Indicator,
    };
    use float_cmp::assert_approx_eq;

    static XS: [f64; 10] = [
        0.003, 0.026, 0.015, -0.009, 0.014, 0.024, 0.015, 0.066, -0.014, 0.039,
    ];
    #[test]
    fn geometric() {
        let mut indicator = AnnualizedReturn::new(mode::Geometric, 10);
        XS.iter().for_each(|x| indicator.feed(*x));
        assert_approx_eq!(
            f64,
            0.19135615147149543,
            *indicator.last().unwrap(),
            epsilon = 0.0000001
        );
    }
    #[test]
    fn simple() {
        let mut indicator = AnnualizedReturn::new(mode::Simple, 10);
        XS.iter().for_each(|x| indicator.feed(*x));
        assert_approx_eq!(f64, 0.179, *indicator.last().unwrap(), epsilon = 0.0000001);
    }
}
