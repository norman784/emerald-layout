use enums::Unit;

#[derive(Copy, Clone)]
pub struct Value {
    value: f32,
    unit: Unit,
}

impl Value {
    pub fn new(value: f32, unit: Unit) -> Value {
        Value {value, unit}
    }

    pub fn default() -> Value {
        Value::new(0.0, Unit::Undefined)
    }

    pub fn undefined() -> Value {
        Value::new(f32::MAX, Unit::Undefined)
    }

    pub fn zero() -> Value {
        Value::new(0.0, Unit::Point)
    }

    pub fn get_value(&self) -> f32 {
        self.value
    }

    pub fn get_unit(&self) -> Unit {
        self.unit
    }
}