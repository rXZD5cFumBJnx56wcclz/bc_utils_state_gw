use bc_utils_lg::prelude::*;
use bc_utils_state::main_trait::UtilState;
use bc_gw_utils::prelude::*;

#[derive(Default)]
pub struct UtilsState<'a>(pub MAP<&'a str, Box<dyn UtilState>>);

impl<'a> UtilsState<'a> {
    pub fn init(
        &mut self,
        s: &'a SETTINGS_UTILS_STATE,
        fa: &PACK<SETTINGS_UTIL_STATE, Box<dyn UtilState>>,
    ) {
        *self = Self(s.iter()
        .map(|(k, v)| (k.as_str(), fa[v.key.as_str()](v)))
        .collect())
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
                let mut src = SrcGwSeries::default();
                src.push_vec(buffer, &setting.used_src);
                src.push_map(indications, &setting.used_ind);
                src.all_check(&setting.procedure_used_src);
                (
                    k.as_str(),
                    self.0[k.as_str()].util(
                        state,
                        &src,
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
    use crate::test_state::*;
    use bc_packs::PACK_UTIL;
    use bc_test_kit::prelude::*;
    use bc_utils_lg::test_state::prelude::*;

    #[test]
    fn series_res_1() {
        let mut utils = UtilsState::default();
        utils.init(&UTILS_STATE, &PACK_UTIL);
        assert_eq_pr!(
            &utils.series(
                &TradeState::new(Capital(100.),),
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
