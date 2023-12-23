use serde::Deserialize;
use std::{io, path::Path, fs::read_to_string};
use nalgebra::Vector3;

#[derive(Deserialize, Debug)]
pub struct Body {
    pub id: u8,
    pub label: String,
    pub mass: f64,
    pub pos: Vec<f64>,
    pub vel: Vec<f64>,
    pub obj_col: f64,
}

impl PartialEq for Body {
    fn eq(&self, other: &Self) -> bool {
        (self.id == other.id) &&
        (self.label == other.label) &&
        (self.pos == other.pos) &&
        (self.vel == other.vel) &&
        (self.obj_col == other.obj_col) &&
        approx::relative_eq!(self.mass, other.mass, epsilon = f64::EPSILON)
    }
}

#[derive(Debug)]
pub enum SemantixError {
    IO(io::Error), 
    Parse(serde_json::Error)
}

impl From<io::Error> for SemantixError {
    fn from(err: io::Error) -> Self {
        SemantixError::IO(err)
    }
}

impl From<serde_json::Error> for SemantixError {
    fn from(err: serde_json::Error) -> Self {
        SemantixError::Parse(err)
    }
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct Data {
    pub system_vec: Vec<Body>,
}

impl Data {
    pub fn get_field_mat(self: &Self, field_type: &str) -> Vec<Vector3<f64>> {

        self.system_vec
                    .iter()
                    .map(|body| match field_type{
                                        "pos" => na::Vector3::<f64>::from_vec(
                                            body.pos.clone()),
                                        "vel" => na::Vector3::<f64>::from_vec(
                                            body.vel.clone()),
                                        _ => panic!("Unexpected field type")
                        }).collect::<Vec<Vector3<f64>>>().clone()
    }
    pub fn get_field_mass(self: Self) -> Vec<f64> {
        self.system_vec
                        .iter()
                        .map(|body| body.mass).collect::<Vec<f64>>().clone()
    }
}

pub fn load_data<P: AsRef<Path>>(path: P) -> Result<Data, SemantixError> {
    let contents = read_to_string(path)?;
    let data: Data = serde_json::from_str(&contents)?;
    Ok(data)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_data() {
        let test_struct = Data { 
            system_vec: 
            vec![Body {
                    id: 0,
                    label: "Sun".to_string(),
                    mass: 1.9891e30,
                    obj_col: 1.0,
                    pos: vec![0.0, 0.0, 0.0],
                    vel: vec![0.0, 0.0, 0.0 ]
                }]};
        let test_result = load_data("test_f.json").unwrap();
        assert_eq!(test_struct, test_result);
    }
}