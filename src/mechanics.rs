use crate::{
    State,
    constants::*, 
    utils::{y_to_vecs, vecs_to_y}};
use na::Vector3;

pub fn calculate_force_vec(r1: &Vector3<f64>, r2: &Vector3<f64>, m1: &f64, m2: &f64) -> Vector3<f64> {

    G_AU3_KG_DAY2 * m1 * m2 * (r2 - r1) / (r1 - r2).norm().powf(3.0)

}

pub fn get_derivatives(y: State, masses: &Vec<f64>, n_objects: usize) -> State {
    // This function will calculate the net forces on n bodies due to each other. 
    // Force matrices are symmetric, so only the lower triangle of the matrix is calculated. 
    let (r_vec, v) = y_to_vecs(y, n_objects);
    let mut net_forces = vec![Vector3::<f64>::zeros(); n_objects];
    r_vec.iter().zip(masses.iter()).enumerate().for_each(|(i, (r1, m1))| {
        let _ = r_vec.iter().take(i)
                            .zip(masses.iter().take(i))
                            .enumerate()
                            .map(|(j, (r2, m2))| {
                                let fij: Vector3<f64> = calculate_force_vec(r1, r2, m1, m2);
                                net_forces[i] = net_forces[i] + fij;
                                net_forces[j] = net_forces[j] + -fij;
                            }).collect::<Vec<_>>();
    });
    net_forces = net_forces.into_iter().zip(masses.iter()).map(|(f, m)| f / *m).collect::<Vec<Vector3<f64>>>();
    
    vecs_to_y(v, net_forces, n_objects)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_force_vec() {

        let v1 = Vector3::from_vec(vec![1.0,1.0,1.0]);
        let v2 = Vector3::from_vec(vec![1.0,2.0,3.0]);
        let m1 = 10.0e20;
        let m2 = 20.0e20;

        let test: Vector3<f64> = calculate_force_vec(&v1, &v2, &m1, &m2);
        let actual: Vector3<f64> = Vector3::from_vec(vec![0.0, 26614645.08182895, 53229290.1636579]);
        assert_eq!(actual, test)
    }

    #[test]
    fn test_get_net_forces() {

        let y = State::from_vec(vec![1.0,1.0,1.0,1.0,2.0,3.0,2.0,0.0,0.0,1.0,0.0,0.0]);
        let masses = vec![10.0e20, 20.0e20]; 
        let net_f = get_derivatives(y, &masses, 2);
        let act_net = State::from_vec(vec![
            2.0,0.0,0.0,1.0,0.0,0.0,0.0, 
            26614645.08182895 / 10.0e20, 53229290.1636579 / 10.0e20,0.0, 
            -26614645.08182895 / 20.0e20, -53229290.1636579 / 20.0e20]);
        assert_eq!(act_net, net_f)
    }
}