/// Formula
/// (°F − 32) × 5/9 = °C
pub fn ftoc(t: f64) -> f64 {
    return (t - 32.0) * 5.0 / 9.0;
}

/// Formula
///(°C × 9/5) + 32 = °F
pub fn ctof(t: f64) -> f64 {
    return t * 9.0 / 5.0 + 32.0;
}

pub fn pow(x: f64, y: f64) -> f64 {
    x.powf(y)
}

// sqrt(...)
// abs(...)
// round(...)
// floor(...)
// ceil(...)

// THEORETICALLY CLEANER

// pub type NativeFn = fn(&[f64]) -> Result<f64, String>;
// static FUNCS: &[(&str, NativeFn)] = &[
//     ("ftoc", ftoc),
//     // ("ctof", ctof),
//     // ("pow", pow),
// ];
//
// pub fn func(func_name: &str) -> Result<NativeFn, String> {
//     let func = FUNCS
//         .iter()
//         .find(|(name, _)| *name == func_name)
//         .ok_or("function not found")?
//         .1;
//     // let func = FUNCS
//     //     .iter()
//     //     .find(|(name, _)| *name == func_name)
//     //     .map(|(_, func)| func);
//     return Ok(func);
// }
//
// /// Formula
// /// (°F − 32) × 5/9 = °C
// // pub fn ftoc(t: f64) -> f64 {
// fn ftoc(t: &[f64]) -> Result<f64, String> {
//     let n_args = get_arg_count(node)?;
//     if n_args != 1 {
//         return Err(format!("ftoc expects one arg. Got {}", n_args));
//     }
//     return Ok((t[0] - 32.0) * 5.0 / 9.0);
// }
//
// /// Formula
// ///(°C × 9/5) + 32 = °F
// fn ctof(t: f64) -> f64 {
//     return t * 9.0 / 5.0 + 32.0;
// }
//
// fn pow(x: f64, y: f64) -> f64 {
//     //         if n_args != 2 {
//     //             return Err(format!("pow expects two args. Got {}", n_args));
//     //         }
//     x.powf(y)
// }
