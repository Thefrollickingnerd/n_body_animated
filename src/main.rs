use itertools::Itertools;
use na::RealField;
use n_body_animated::data_structs::{load_data, Data};
use n_body_animated::{solve_forward, animate_motion};
use n_body_animated::utils::vecs_to_y;
use nalgebra as na;
fn main() {
    
    let data: Data = match load_data("solar_system.json") {
        Ok(data) => data,
        Err(e) => panic!("Problem opening the file: {:?}", e),
    };
    let n_bodies = data.system_vec.len();
    let r_vecs: Vec<na::Matrix<f64, na::Const<3>, na::Const<1>, na::ArrayStorage<f64, 3, 1>>> = data.get_field_mat("pos");
    let v_vecs = data.get_field_mat("vel");
    let masses = data.get_field_mass();

    let max_radius = r_vecs.iter().fold(0.0, |a, r| a.max(r.amax())) * 1.02;
    
    let y_0 = vecs_to_y(r_vecs, v_vecs, n_bodies);
    
    let (stats, _x, y_vecs) = match solve_forward(y_0, &masses, n_bodies) {
        Ok((stats, x, y_vecs)) => (stats, x, y_vecs),
        Err(e) => panic!("{}",e)
    };
    println!("{}", stats);

    let y_tuples: Vec<Vec<(f64,f64,f64)>> = y_vecs.into_iter().map(|line| {
        line
        .iter()
        .take(3*n_bodies)
        .map(|x| x)
        .cloned()
            .batching(|it| {
                match it.next() {
                    None => None,
                    Some(x) => match it.next() {
                        None => None,
                        Some(y) => match it.next() {
                            None => None,
                            Some(z) => Some((x,y,z))
                        }
                    }
                }
            }).collect::<Vec<(f64,f64,f64)>>()
    }).collect();

    animate_motion(y_tuples, max_radius)
}
