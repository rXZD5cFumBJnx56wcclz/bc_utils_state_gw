use bc_utils::other::procedure_used;
use bc_utils_lg::{
    structs::{
        settings::{SETTINGS_UTIL_STATE, SETTINGS_UTILS_STATE},
        signals::Signal,
        trade::TradeState,
    },
    types::maps::{FUNCS_EXTRACT_ARGS_TYPE, MAP},
};
use bc_utils_state::main_trait::UtilState;

pub struct UtilsStateGateway<'a> {
    pub utils_state: *const MAP<&'a str, Box<dyn UtilState>>,
    pub s: &'a SETTINGS_UTILS_STATE,
}

pub fn get_map<'a>(
    s: &'a SETTINGS_UTILS_STATE,
    fa: &FUNCS_EXTRACT_ARGS_TYPE<SETTINGS_UTIL_STATE, Box<dyn UtilState>>,
) -> MAP<&'a str, Box<dyn UtilState>> {
    s.iter()
        .map(|(k, v)| (k.as_str(), fa[v.key.as_str()](v)))
        .collect()
}

fn get_src(
    s: &SETTINGS_UTIL_STATE,
    buffer: &[Vec<f64>],
    indications: &MAP<&str, f64>,
) -> Vec<f64> {
    let mut res = Vec::with_capacity(s.used_ind.len() + s.used_src.len());
    for used_src in &s.used_src {
        res.push(buffer[buffer.len() - 1 - used_src.sub_from_last_i][used_src.index]);
    }
    for used_ind in &s.used_ind {
        res.push(indications[used_ind.as_str()]);
    }
    if !s.procedure_used_src.is_empty() {
        res = procedure_used(res, &s.procedure_used_src);
    }
    res
}

impl<'a> UtilsStateGateway<'a> {
    pub fn new(
        utils_state: *const MAP<&'a str, Box<dyn UtilState>>,
        s: &'a SETTINGS_UTILS_STATE,
    ) -> Self {
        Self { utils_state, s }
    }
}

impl<'a> UtilsStateGateway<'a> {
    pub fn series(
        &self,
        state: &TradeState,
        buffer: &[Vec<f64>],
        indications: &MAP<&str, f64>,
        signals: &MAP<&str, Signal>,
    ) -> MAP<&'a str, f64> {
        self.s
            .iter()
            .map(|(k, setting)| {
                (
                    k.as_str(),
                    unsafe { &*self.utils_state }[k.as_str()].util(
                        state,
                        &get_src(setting, buffer, indications),
                        setting
                            .used_signals
                            .iter()
                            .map(|v| signals[v.as_str()])
                            .collect::<Vec<Signal>>()
                            .as_slice(),
                    ),
                )
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bc_pack_utils_state::FUNCS_EXTRACT_ARGS;
    use pretty_assertions::assert_eq as assert_eq_pr;

    #[test]
    fn series_res_1() {
        let s = MAP::from_iter([(
            "qty_1".to_string(),
            SETTINGS_UTIL_STATE {
                key: "qty".to_string(),
                kwargs_f64: MAP::from_iter([
                    ("amount".to_string(), 1.),
                    ("percent_of_capital".to_string(), 0.1),
                ]),
                ..Default::default()
            },
        )]);
        let map = get_map(&s, &FUNCS_EXTRACT_ARGS);
        assert_eq_pr!(
            UtilsStateGateway::new(&map, &s).series(
                &TradeState::new(100., Default::default(), Default::default()),
                &[],
                &Default::default(),
                &Default::default(),
            )["qty_1"],
            11.,
        );
    }
}
