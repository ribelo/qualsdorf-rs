use crate::{ror::RoR, Indicator, ReturnExt};

#[derive(Debug)]
pub struct CAGR {
    pub freq: usize,
    pub p: f64,
    input: Vec<f64>,
    ror: RoR,
    pub values: Vec<Option<f64>>,
}

impl CAGR {
    pub fn new(freq: usize, p: f64) -> Self {
        CAGR {
            freq,
            p,
            input: Vec::with_capacity(freq),
            ror: RoR::new(freq),
            values: Vec::with_capacity(freq),
        }
    }
}

impl Indicator for CAGR {
    type Input = f64;
    type Output = f64;

    fn feed(&mut self, ret: Self::Input) {
        self.input.push(ret);
        self.ror.feed(ret);
        if let Some(ror) = self.ror.last() {
            let value = (1.0 + ror).powf(self.p) - 1.0;
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

pub trait CagrExt {
    fn cagr(&self, freq: usize, p: f64) -> Option<CAGR>;
}

impl<T> CagrExt for T
where
    T: ReturnExt,
{
    fn cagr(&self, freq: usize, p: f64) -> Option<CAGR> {
        let mut indicator = CAGR::new(freq, p);
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

    use crate::{cagr::CAGR, Indicator};

    static XS: [f64; 10] = [
        0.003, 0.026, 0.015, -0.009, 0.014, 0.024, 0.015, 0.066, -0.014, 0.039,
    ];
    #[test]
    fn cagr() {
        let mut indicator = CAGR::new(10, 12.0 / 10.0);
        XS.iter().for_each(|&x| indicator.feed(x));
        assert_approx_eq!(
            f64,
            0.229388,
            *indicator.last().unwrap(),
            epsilon = 0.000001
        );
    }
}
