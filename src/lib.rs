extern crate nalgebra as na;
extern crate approx;
pub mod data_structs;
pub mod mechanics;
pub mod constants;
pub mod utils;

use plotters::prelude::*;
use ode_solvers::dop_shared::Stats;
use mechanics::get_derivatives;
use ode_solvers::dopri5::*;

type State = ode_solvers::DVector<f64>;
type Time = f64;

pub struct NBody {
    masses: Vec<f64>,
    n_objects: usize
}

impl ode_solvers::System<State> for NBody {
    // Equations of motion of the system
    fn system(&self, _t: Time, y: &State, dy: &mut State) {
        let out = get_derivatives(y.clone(), &self.masses, self.n_objects);
        for i in 0..y.len() {dy[i] = out[i];}
    }
}

pub fn solve_forward(y: State, masses: &Vec<f64>, n_objects: usize) -> Result<(Stats, Vec<f64>, Vec<State>), &str> {
    let system = NBody {
        masses: masses.clone(),
        n_objects: n_objects
    };
    
    let mut stepper = Dopri5::new(system, 0.0, 5000.0, 10.0, y, 1.0e-10, 1.0e-10);
    let res = stepper.integrate();

    // Handle result
    match res {
        Ok(stats) => {            
            let x = stepper.x_out().clone();
            let y = stepper
                    .y_out()
                    .clone()
                    .iter()
                    .map(|line| State::from_iterator(n_objects*2*3, line.iter().cloned())).collect::<Vec<State>>();
            Ok((stats, x, y))

        },
        _ => Err("Integration issue")
    }
}

pub fn animate_motion(y: Vec<Vec<(f64,f64,f64)>>, max_rad: f64) {
    let area = BitMapBackend::gif("plots/solar_system_animated.png", (1024, 768), 100)
        .unwrap().into_drawing_area();

    let colours = [YELLOW, CYAN, MAGENTA, RED, BLUE, WHITE];
    for step in y.iter() {
        area.fill(&BLACK).unwrap();

        
        let mut chart = ChartBuilder::on(&area)
            .caption("Solar System Animated first 5 Planets", ("sans-serif", 40))
            .build_cartesian_3d(
                -max_rad..max_rad, 
                -max_rad..max_rad,
                -max_rad..max_rad)
            .unwrap();
        
        // Uncomment below to see axis 
        //chart.configure_axes().light_grid_style(WHITE.mix(0.2)).label_style(("Calibri", 10)).draw().unwrap();
        chart.configure_axes().draw().unwrap();

        chart
        .draw_series(
            step.iter().enumerate().map(|(i, point)| Circle::new(*point, 3, &colours[i]))
        ).unwrap();
        area.present().unwrap();
    }
    
}