use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
pub enum Category {
    Length,
    Weight,
    Temperature,
}

#[derive(Deserialize, Debug)]
pub struct ConvertRequest {
    pub category: Category,
    pub from: String,
    pub to: String,
    pub value: f64,
}

#[derive(Serialize, Debug)]
pub struct ConvertResponse {
    pub result: f64,
}

impl ConvertRequest {
    pub fn convert_length(&self) -> Option<f64> {
        let conversions: HashMap<(&str, &str), f64> = [
            (("km", "mile"), 0.621371),
            (("mile", "km"), 1.60934),
            (("m", "ft"), 3.28084),
            (("ft", "m"), 0.3048),
        ]
        .iter()
        .cloned()
        .collect();
        
        conversions.get(&(self.from.as_str(), self.to.as_str()))
            .map(|factor| self.value * factor)
    }

    pub fn convert_temperature(&self) -> Option<f64> {
        match (self.from.as_str(), self.to.as_str()) {
            ("C", "F") => Some(self.value * 9.0 / 5.0 + 32.0),
            ("F", "C") => Some((self.value - 32.0) * 5.0 / 9.0),
            ("C", "K") => Some(self.value + 273.15),
            ("K", "C") => Some(self.value - 273.15),
            ("F", "K") => Some((self.value - 32.0) * 5.0 / 9.0 + 273.15),
            ("K", "F") => Some((self.value - 273.15) * 9.0 / 5.0 + 32.0),
            _ => None,
        }
    }

    pub fn convert_weight(&self) -> Option<f64> {
        let conversions: HashMap<(&str, &str), f64> = [
            (("kg", "lb"), 2.20462),
            (("lb", "kg"), 0.453592),
            (("g", "oz"), 0.035274),
            (("oz", "g"), 28.3495),
        ]
        .iter()
        .cloned()
        .collect();

        conversions.get(&(self.from.as_str(), self.to.as_str()))
            .map(|factor| self.value * factor)
    }
}
