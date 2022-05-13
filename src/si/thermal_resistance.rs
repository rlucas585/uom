//! Thermal Resistance (base unit kelvin per watt)

quantity! {
    /// Thermal resistance (base unit Kelvin/Watt)
    quantity: ThermalResistance; "thermal resistance";
      /// Dimension of thermal resistance, K/W (base unit K)
      dimension: ISQ<
          N2,     // length
          N1,     // mass
          P3,     // time
          Z0,     // electric current
          P1,     // thermodynamic temperature
          Z0,     // amount of substance
          Z0>;    // luminous intensity
          units {
      @kelvin_watt: prefix!(none); "K/W", "kelvin/watt", "kelvin/watts";

      @kelvin_kilowatt: 1.0_E0 / 1.0_E3; "K/kW", "kelvin/kilowatt", "kelvin/kilowatts";
    }
}
