#![allow(unused)]
pub const KM_AU: f64 = 149597871.0;
pub const AU_PC: f64 = 206264.8;
pub const KG_MO: f64 = 1.98855e30;
pub const S_MIN: f64 = 60.0;
pub const MIN_HOUR: f64 = 60.0;
pub const HOUR_DAY: f64 = 24.0;
pub const DAY_YEAR: f64 = 365.25;
pub const YEAR_KYEAR: f64 = 1.0e3;
pub const KM_PC: f64 = KM_AU * AU_PC;
pub const S_KYEAR: f64 = S_MIN * MIN_HOUR * HOUR_DAY * DAY_YEAR * YEAR_KYEAR;
pub const AU: f64 = 1.4960e11; // m
pub const G_AU3_KG_DAY2: f64 = 1.48780389e-34; // AU^3 / (kg * day^2)
pub fn g_pc3_mo_year2() -> f64 {G_AU3_KG_DAY2 * f64::powi(AU_PC, -3) * KG_MO * f64::powi(DAY_YEAR, 2) }// pc^3 / Mo / year^2
pub fn g_pc3_mo_kyear2() -> f64 {g_pc3_mo_year2() * f64::powi(YEAR_KYEAR, 2)}