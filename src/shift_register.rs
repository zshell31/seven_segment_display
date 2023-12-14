use ferrum_hdl::{
    array::{Array, ArrayExt},
    const_helpers::ConstConstr,
    domain::{Clock, ClockDomain},
    index::idx_constr,
    signal::{reg_en, Bundle, Enable, Reset, Signal, SignalValue},
};

pub fn shift_register<D: ClockDomain, const N: usize, T: SignalValue>(
    clk: Clock<D>,
    rst: &Reset<D>,
    next: &Enable<D>,
    init: T,
    data: &Signal<D, T>,
) -> Signal<D, Array<N, T>>
where
    ConstConstr<{ idx_constr(N) }>:,
{
    let mut prev = data.clone();

    let sr = Array::<N, Signal<D, T>>::make(|_| {
        let block =
            prev.and_then(|data| reg_en(clk, rst, next, &init, move |_| data.value()));
        prev = block.clone();
        block
    })
    .bundle();
    sr
}
