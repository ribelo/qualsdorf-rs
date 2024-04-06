use erfurt::candle::CandlesExt;
use itertools::Itertools;
pub mod active_return;
pub mod annualized_return;
pub mod annualized_risk;
pub mod average_drawdown;
pub mod cagr;
pub mod continuous_drawdown;
pub mod downside_potential;
pub mod downside_risk;
pub mod drawndown;
pub mod maximum_drawdown;
pub mod prelude;
pub mod rolling_economic_drawdown;
pub mod ror;
pub mod rsi;
pub mod sharpe_ratio;
pub mod sortino_ratio;
pub mod std;
pub mod upside_potential;

pub trait Indicator {
    type Input;
    type Output;
    fn feed(&mut self, first: Self::Input);
    fn last(&self) -> Option<&Self::Output>;
    fn iter(&self) -> Box<dyn Iterator<Item = Option<&Self::Output>> + '_>;
}

pub mod mode {
    #[derive(Clone, Debug)]
    pub struct Geometric;

    #[derive(Clone, Debug)]
    pub struct Simple;
}

pub trait ReturnExt {
    fn ret(&self) -> Option<Vec<f64>>;
}

pub trait Value<'a> {
    type Output;
    fn value(&'a self) -> Self::Output;
}

impl<T> ReturnExt for T
where
    T: CandlesExt,
{
    fn ret(&self) -> Option<Vec<f64>> {
        if !self.time().is_empty() {
            let mut ret = vec![0.0];
            for (x, y) in self.close().iter().tuple_windows() {
                ret.push(y / x - 1.0)
            }
            Some(ret)
        } else {
            None
        }
    }
}
