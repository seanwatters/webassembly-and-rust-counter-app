use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct CounterState {
  counter: i32
}

#[wasm_bindgen]
impl CounterState {
    pub fn new() -> CounterState {
        let counter = 0;

        CounterState {
            counter,
        }
    }

    pub fn increment_counter(&mut self) -> i32 {
        self.counter = self.counter + 1;
        self.counter
    }

    pub fn decrement_counter(&mut self) -> i32 {
        self.counter = self.counter - 1;
        self.counter
    }

    pub fn get_counter(&self) -> i32 {
        self.counter
    }
}