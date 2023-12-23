use nalgebra::Vector3;
use itertools::Itertools;
use crate::State;

pub fn vecs_to_y(
    r_vec: Vec<Vector3<f64>>, 
    v_vec: Vec<Vector3<f64>>, 
    n_objects: usize) -> State {
    // Reshape r and v vec of vecs into single y vector stacked 
    // in order of r -> v
    let mut y = r_vec.iter().flat_map(|v| v).cloned().collect::<Vec<f64>>();
    y.extend(v_vec.iter().flat_map(|v| v).cloned());
    
    State::from_iterator(2*3*n_objects, y.into_iter())
}

pub fn y_to_vecs(
    y: State,
    n_objects: usize) -> (Vec<Vector3<f64>>, Vec<Vector3<f64>>) {
    // Reshape r and v vec of vecs into single y vector stacked 
    // in order of r -> v
    let mut temp: Vec<Vector3<f64>> = y
            .iter()
            .map(|x| x)
            .cloned()
            .batching(|it| {
                match it.next() {
                    None => None,
                    Some(x) => match it.next() {
                        None => None,
                        Some(y) => match it.next() {
                            None => None,
                            Some(z) => Some(Vector3::new(x,y,z))
                        }
                    }
                }
            }).collect();
    let v = temp.split_off(n_objects);
    (temp,v)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vecs_to_y(){
        let r_vec = vec![Vector3::from_vec(vec![1.0,1.0,1.0]), Vector3::from_vec(vec![1.0,2.0,3.0])];
        let v_vec = vec![Vector3::from_vec(vec![2.0,0.0,0.0]), Vector3::from_vec(vec![1.0,0.0,0.0])];
        let n_bodies = 2;
        let y = vecs_to_y(r_vec, v_vec, n_bodies);
        dbg!(y);
    }

    #[test]
    fn test_y_to_vecs(){
        let y = State::from_vec(vec![1.0,1.0,1.0,1.0,2.0,3.0,2.0,0.0,0.0,1.0,0.0,0.0]);
        let n_bodies = 2;
        let (r,v) = y_to_vecs(y, n_bodies);
        dbg!(r);
        dbg!(v);
    }
}