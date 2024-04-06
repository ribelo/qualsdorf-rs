use crate::{Indicator, ReturnExt};

#[derive(Debug)]
pub struct UpsidePotential {
    pub freq: usize,
    pub mar: f64,
    input: Vec<f64>,
    pub values: Vec<Option<f64>>,
}

impl UpsidePotential {
    pub fn new(freq: usize, mar: f64) -> Self {
        Self {
            freq,
            mar,
            input: Vec::new(),
            values: Vec::new(),
        }
    }
}

impl Indicator for UpsidePotential {
    type Input = f64;
    type Output = f64;

    fn feed(&mut self, ret: Self::Input) {
        self.input.push(ret);
        if self.input.len() >= self.freq {
            let value = self.input[self.input.len() - self.freq..]
                .iter()
                .fold(0.0, |acc, x| {
                    acc + (x - self.mar).max(0.0) / self.input.len() as f64
                });
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

pub trait UpsidePotentialExt {
    fn upside_potential(&self, freq: usize, mar: f64) -> Option<UpsidePotential>;
}

impl<T> UpsidePotentialExt for T
where
    T: ReturnExt,
{
    fn upside_potential(&self, freq: usize, mar: f64) -> Option<UpsidePotential> {
        let mut indicator = UpsidePotential::new(freq, mar);
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

    use crate::{upside_potential::UpsidePotential, Indicator};

    static XS: [f64; 10] = [
        0.003, 0.026, 0.015, -0.009, 0.014, 0.024, 0.015, 0.066, -0.014, 0.039,
    ];
    #[test]
    fn downside_potential() {
        let mut indicator = UpsidePotential::new(10, 0.1 / 100.0);
        XS.iter().for_each(|x| indicator.feed(*x));
        assert_approx_eq!(f64, 0.0194, *indicator.last().unwrap(), epsilon = 0.0000001);
    }
}
