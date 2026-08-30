use reqwest;
use reqwest::blocking as req;
use serde::Deserialize;
use std;
const URL: &str = "https://www.bankofcanada.ca/valet/observations/FXUSDCAD/json?recent=1";

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

#[derive(Deserialize, Debug)]
struct Val {
    v: String,
}
#[derive(Deserialize, Debug)]
struct Observation {
    FXUSDCAD: Val,
}
#[derive(Deserialize, Debug)]
struct Response {
    observations: Vec<Observation>,
}

fn get_forex() -> Result<f64, Box<dyn std::error::Error>> {
    let client = req::Client::new();
    let resp = client.get(URL).send()?;
    if !resp.status().is_success() {
        // use cached value
        return Err(format!("forex api returned: {}", resp.status()).into());
    }
    let r: Response = resp.json()?;
    let ex: f64 = r.observations[0].FXUSDCAD.v.parse::<_>().unwrap();
    Ok(ex)
}
pub fn cad_usd(cad: f64) -> Result<f64, String> {
    let ex = get_forex();
    match ex {
        Ok(x) => Ok(cad / x),
        Err(e) => Err(e.to_string()),
    }
}
pub fn usd_cad(usd: f64) -> Result<f64, String> {
    let ex = get_forex();
    match ex {
        Ok(x) => Ok(usd * x),
        Err(e) => Err(e.to_string()),
    }
}

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
