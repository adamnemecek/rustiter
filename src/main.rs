
pub struct Vec1 {
    data: Vec<u32>
}

impl Vec1 {
    pub fn new() -> Self {
        Self {
            data: vec![0,1,2,3,4,5]
        }
    }
}

impl std::ops::Deref for Vec1 {
    type Target = [u32];
    fn deref(&self) -> &Self::Target {
        self.data.deref()
    }
}

impl std::ops::DerefMut for Vec1 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.data.deref_mut()
    }
}

pub struct IntoIter {
    inner: Vec1
}

impl Iterator for IntoIter {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

impl IntoIterator for Vec1 {
    type Item = u32;
    type IntoIter = IntoIter;
    fn into_iter(self) -> Self::IntoIter {
        todo!()
    }

}

fn main() {
    println!("Hello, world!");
}
