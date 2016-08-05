use typenum::{Z0, P1};
use ::{Quantity};
use ::si::{SI};

pub type Dimensions = SI<Z0, Z0, Z0, Z0, P1, Z0, Z0>;
pub type ThermodynamicTemperature<B, V> = Quantity<Dimensions, B, V>;

// Needs temperature specific conversion implentation for Fahrenheit/Rankine.
//subunits!(thermodynamic_temperature; Units: ThermodynamicTemperature {
//    yottakelvin: prefix!(yotta);
//    zettakelvin: prefix!(zetta);
//    exakelvin: prefix!(exa);
//    petakelvin: prefix!(peta);
//    terakelvin: prefix!(tera);
//    megakelvin: prefix!(mega);
//    kilokelvin: prefix!(kilo);
//    hectokelvin: prefix!(hecto);
//    decakelvin: prefix!(deca);
//    kelvin: 1.0E0;
//    decikelvin: prefix!(deci);
//    centikelvin: prefix!(centi);
//    millikelvin: prefix!(milli);
//    microkelvin: prefix!(micro);
//    nanokelvin: prefix!(nano);
//    picokelvin: prefix!(pico);
//    femtokelvin: prefix!(femto);
//    attokelvin: prefix!(atto);
//    zeptokelvin: prefix!(zepto);
//    yoctokelvin: prefix!(yocto);
//});