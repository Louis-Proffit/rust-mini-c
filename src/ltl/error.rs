use crate::coloring::ColoringError;
use crate::interference::InterferenceError;
use crate::liveness::error::LivenessError;
use crate::rtl::structure::register::PseudoRegister;

#[derive(Debug)]
pub enum LtlError {
    MissingRegisterColor(PseudoRegister),
    LivenessError(LivenessError),
    InterferenceError(InterferenceError),
    ColoringError(ColoringError),
    Any(&'static str)
}