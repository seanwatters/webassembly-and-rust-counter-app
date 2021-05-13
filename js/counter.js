import { CounterState } from 'wasm';

const counter_state = CounterState.new();

document.getElementById("counter").textContent = counter_state.get_counter();

document.getElementById("increment").addEventListener("click", () => {
    document.getElementById("counter").textContent = counter_state.increment_counter();
});
document.getElementById("decrement").addEventListener("click", () => {
    document.getElementById("counter").textContent = counter_state.decrement_counter();
});