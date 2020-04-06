
pub struct Vec1 {
    data: Vec<u32>
}

impl Vec1 {
    pub fn new() -> Self {
        Self {
            data: vec![1,2,3,4]
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

// pub struct IntoIter {
//     idx: usize,
//     inner: Vec1
// }

// impl Iterator for IntoIter {
//     type Item = u32;
//     fn next(&mut self) -> Option<Self::Item> {
//         if self.idx == self.inner.len() {
//             None
//         }
//         else {
//             let result = self.inner[self.idx];
//             self.idx += 1;
//             Some(result)
//         }
//     }
// }

// impl IntoIterator for Vec1 {
//     type Item = u32;
//     type IntoIter = IntoIter;
//     fn into_iter(self) -> Self::IntoIter {
//         todo!()
//     }

// }

fn main() {
    let v1 = Vec1::new();
    let mut v1_iter = v1.iter();

    // iter() returns an iterator of slices.
    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    // assert_eq!(v1_iter.next(), Some(&4));
    println!("{}", v1_iter.next().unwrap());
}
