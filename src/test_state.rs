use bc_utils_lg::prelude::*;

pub static UTILS_STATE_STATE: LazyLock<MAP<&str, f64>> =
    LazyLock::new(|| MAP::from_iter([("qty_1", 110.), ("direction_1", 1.)]));
