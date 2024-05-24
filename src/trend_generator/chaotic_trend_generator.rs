use std::sync::Mutex;
use lazy_static::lazy_static;

use crate::market::market::{buy_market, sell_market};
use crate::market::order::Stock;

const LORENZ_ITERATIONS: u64 = 100;
const ACTION_ITERATIONS: u64 = 500;
const VOLUME_MULTIPLIER: u64 = 5;

struct LorenzState {
    x: f64,
    y: f64,
    z: f64 
}

lazy_static! {
    static ref _state: Mutex<LorenzState> = Mutex::new(LorenzState {x: 1.0, y: 1.0, z: 1.0});
    static ref _replace_ptr: Mutex<u32> = Mutex::new(0);
}

pub fn lorenz_dy() -> f64 {
    let mut state = _state.lock().unwrap();

    let sigma = 10.0;
    let rho = 28.0;
    let beta = 8.0/3.0;

    let dx = sigma * (state.y - state.x);
    let dy = state.x * (rho - state.z) - state.y;
    let dz = state.x * state.y - beta * state.z;

    let damping_factor = 10e2;

    // sliding window of the lorenz func
    state.x += dx/damping_factor;
    state.y += dy/damping_factor;
    state.z += dz/damping_factor;

    let mut x = state.x;
    let mut y = state.y;
    let mut z = state.z;
    let sum_dy: f64 = (0..LORENZ_ITERATIONS).map(|_| {
        let dx = sigma * (state.y - state.x);
        let dy = state.x * (rho - state.z) - state.y;
        let dz = state.x * state.y - beta * state.z;


        x += dx/damping_factor;
        y += dy/damping_factor;
        z += dz/damping_factor;
        
        dy
    }).sum();


    sum_dy/LORENZ_ITERATIONS as f64
}


pub fn chaotic_trend_generator(stock: Stock) {
    // let mut momentum = *_momentum.lock().unwrap();
    for _ in 0..ACTION_ITERATIONS {
        let mut trend = -lorenz_dy();
        
        // idea here is to add a component to the trend that inverses the momentum of the past MOMENTUM_MEMORY moves
        // this kind of acts the same way like the tuned mass damper in tipei 101

        //  ^ fun idea to play with, might need array of long and short term mass dampers for this to work
        // or just some equations

        // println!("momentum is {momentum}");
        let size = (trend * trend).sqrt() as u64 * VOLUME_MULTIPLIER;

        match trend > 0.0 {
            true => {
                // println!("CHAOS: Bought {size}");
                buy_market(stock, size);
                // sell_market(stock, (size as f64).sqrt() as u64);
            },
            false => {
                // println!("CHAOS: Sold market {size}");
                // buy_market(stock, (size as f64).sqrt() as u64);
                sell_market(stock, size)
            }
        }
    }
}

