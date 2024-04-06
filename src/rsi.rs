use erfurt::candle::CandlesExt;

use crate::Indicator;

#[derive(Debug)]
pub struct RSI {
    pub freq: usize,
    input: Vec<f64>,
    gains: Vec<f64>,
    losses: Vec<f64>,
    pub values: Vec<Option<f64>>,
}

impl RSI {
    pub fn new(freq: usize) -> RSI {
        RSI {
            freq,
            input: Vec::with_capacity(freq),
            gains: Vec::with_capacity(freq),
            losses: Vec::with_capacity(freq),
            values: Vec::with_capacity(freq),
        }
    }
}

impl Indicator for RSI {
    type Input = f64;
    type Output = f64;

    fn feed(&mut self, price: Self::Input) {
        if let Some(last_price) = self.input.last() {
            if price > *last_price {
                self.gains.push(price - last_price);
                self.losses.push(0.0);
            } else {
                self.gains.push(0.0);
                self.losses.push(last_price - price);
            }
        } else {
            self.gains.push(0.0);
            self.losses.push(0.0);
        }
        self.input.push(price);
        if self.input.len() >= self.freq {
            let avg_gain = self.gains[self.gains.len() - self.freq..]
                .iter()
                .sum::<f64>()
                / self.freq as f64;
            let avg_loss = self.losses[self.losses.len() - self.freq..]
                .iter()
                .sum::<f64>()
                / self.freq as f64;
            let rs = if avg_loss != 0.0 {
                avg_gain / avg_loss
            } else {
                0.0
            };
            let rsi = 100.0 - 100.0 / (1.0 + rs);
            self.values.push(Some(rsi));
        } else {
            self.values.push(None);
        }
    }

    fn last(&self) -> Option<&Self::Output> {
        self.values.last().and_then(|v| v.as_ref())
    }

    fn iter(&self) -> Box<dyn Iterator<Item = Option<&Self::Output>> + '_> {
        Box::new(self.values.iter().map(Option::as_ref))
    }
}

pub trait RsiExt {
    fn rsi(&self, freq: usize) -> Option<RSI>;
}

impl<T> RsiExt for T
where
    T: CandlesExt,
{
    fn rsi(&self, freq: usize) -> Option<RSI> {
        let mut indicator = RSI::new(freq);
        self.close().iter().for_each(|v| indicator.feed(*v));
        Some(indicator)
    }
}
