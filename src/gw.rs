use bc_utils::other::procedure_used;
use bc_utils_lg::{
    structs::{
        settings::{SETTINGS_TRADE, SETTINGS_UTIL_STATE, SETTINGS_UTILS_STATE}, signals::Signal, trade::TradeState,
    }, types::maps::{MAP, PACK},
};
use bc_utils_state::main_trait::UtilState;

pub fn get_map<'a>(
    s: &'a SETTINGS_UTILS_STATE,
    fa: &PACK<SETTINGS_UTIL_STATE, Box<dyn UtilState>>,
) -> MAP<&'a str, Box<dyn UtilState>> {
    s.iter()
        .map(|(k, v)| (k.as_str(), fa[v.key.as_str()](v)))
        .collect()
}

fn get_src(s: &SETTINGS_UTIL_STATE, buffer: &[Vec<f64>], indications: &MAP<&str, f64>) -> Vec<f64> {
    let mut res = Vec::with_capacity(s.used_ind.len() + s.used_src.len());
    for used_src in &s.used_src {
        res.push(buffer[used_src.index][buffer.len() - 1 - used_src.sub_from_last_i]);
    }
    for used_ind in &s.used_ind {
        res.push(indications[used_ind.as_str()]);
    }
    if !s.procedure_used_src.is_empty() {
        res = procedure_used(res, &s.procedure_used_src);
    }
    res
}

#[derive(Default)]
pub struct UtilsState<'a>(pub MAP<&'a str, Box<dyn UtilState>>);

impl<'a> UtilsState<'a> {
    pub fn init(
        &mut self,
        s: &'a SETTINGS_UTILS_STATE,
        fa: &PACK<SETTINGS_UTIL_STATE, Box<dyn UtilState>>,
    ) {
        *self = Self(get_map(s, fa))
    }
}

impl<'a> UtilsState<'a> {
    pub fn series(
        &self,
        state: &TradeState,
        buffer: &[Vec<f64>],
        s_trade: &SETTINGS_TRADE,
        s: &'a SETTINGS_UTILS_STATE,
        indications: &MAP<&str, f64>,
        signals: &MAP<&str, Signal>,
    ) -> MAP<&'a str, f64> {
        s.iter()
            .map(|(k, setting)| {
                (
                    k.as_str(),
                    self.0[k.as_str()].util(
                        state,
                        &get_src(setting, buffer, indications),
                        setting
                            .used_signals
                            .iter()
                            .map(|v| signals[v.as_str()])
                            .collect::<Vec<Signal>>()
                            .as_slice(),
                        s_trade,
                    ),
                )
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bc_packs::PACK_UTIL;
    use bc_test_kit::prelude::*;

    #[test]
    fn series_res_1() {
        let mut utils = UtilsState::default();
        utils.init(&UTILS_STATE, &PACK_UTIL);
        assert_eq_pr!(
            &utils.series(
                &TradeState::new(100.,),
                &[],
                &TRADE,
                &*UTILS_STATE,
                &Default::default(),
                &Default::default(),
            ),
            &*UTILS_STATE_STATE,
        );
    }
}
