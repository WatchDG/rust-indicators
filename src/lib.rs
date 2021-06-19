pub trait IndicatorSMA {
    type Input;
    type Period;
    type Limit;
    type Output;
    fn sma_indicator(input: Self::Input, period: Self::Period, limit: Self::Limit) -> Self::Output;
}
