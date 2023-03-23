use crate::entities::character;

impl character::Model {
    pub fn get_skill_pages(&self) -> &[u8; 10] {
        self.skill_points.as_slice().try_into().unwrap()
    }

    pub fn get_skill_pages_mut(&mut self) -> &mut [u8; 10] {
        self.skill_points.as_mut_slice().try_into().unwrap()
    }
}