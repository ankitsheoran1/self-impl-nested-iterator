fn main() {
    println!("Hello, world!");
}

// pub fn flatten<I>(iter: I) -> Flatten<I> 
// where 
// I: Iterator,
// I::Item:IntoIterator {
//    Flatten::new(iter)
// }


// pub struct Flatten<O> 
// where 
// O: Iterator,
// O::Item:IntoIterator {
//     outer: O,
//     inner:  Option<<O::Item as IntoIterator>::IntoIter>,
// } 

// impl<O> Flatten<O> 
// where 
// O:Iterator,
// O::Item:IntoIterator {
//     fn new(iter: O) -> Self {
//         Flatten { outer: iter, inner: None }
//     }
// }

// impl <O> Iterator for Flatten<O>
// where 
// O: Iterator, 
// O::Item: IntoIterator, {
//     type Item = <O::Item as IntoIterator>::Item;
//     fn next(&mut self) -> Option<Self::Item> {
//         loop {
//         if let Some(ref mut inner_iter) = self.inner {
//             if let Some(i) = inner_iter.next() {
//                 return Some(i);
//             }
//         }
//         let next_inner_iter = self.outer.next()?.into_iter();
//         self.inner = Some(next_inner_iter);
//     }
//         //self.inner.as_mut().unwrap().next();
//         // let mut inner_iter = inner_item.into_iter();
//         // inner_iter.next()
//         // self.outer.next().and_then(|inner| inner.into_iter().next())
//         //None 
//     }

// }


// 2

pub fn flatten<I>(iter: I) -> Flatten<I> 
where 
I: Iterator,
I::Item:IntoIterator {
   Flatten::new(iter)
}


pub struct Flatten<O> 
where 
O: Iterator,
O::Item:IntoIterator {
    outer: O,
    next_iter:  Option<<O::Item as IntoIterator>::IntoIter>,
    back_iter:  Option<<O::Item as IntoIterator>::IntoIter>,
} 

impl<O> Flatten<O> 
where 
O:Iterator,
O::Item:IntoIterator {
    fn new(iter: O) -> Self {
        Flatten { outer: iter, next_iter: None, back_iter: None }
    }
}

impl <O> Iterator for Flatten<O>
where 
O: Iterator, 
O::Item: IntoIterator, {
    type Item = <O::Item as IntoIterator>::Item;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
        if let Some(ref mut inner_iter) = self.next_iter {
            if let Some(i) = inner_iter.next() {
                return Some(i);
            }
            self.next_iter = None;
        }
        if let Some(next_inner_iter) = self.outer.next() {
            self.next_iter = Some(next_inner_iter.into_iter());
        } else {
            self.back_iter.as_mut()?.next();
        }
        // let next_inner_iter = self.outer.next()?.into_iter();
        // self.inner = Some(next_inner_iter);
    }
        //self.inner.as_mut().unwrap().next();
        // let mut inner_iter = inner_item.into_iter();
        // inner_iter.next()
        // self.outer.next().and_then(|inner| inner.into_iter().next())
        //None 
    }

}

impl <O> DoubleEndedIterator for Flatten<O> 
where 
O: DoubleEndedIterator,
O::Item:IntoIterator,
<O::Item as IntoIterator>::IntoIter: DoubleEndedIterator    {

    fn next_back(&mut self) -> Option<Self::Item> {
        
            loop {
            if let Some(ref mut back_iter) = self.back_iter {
                if let Some(i) = back_iter.next_back() {
                    return Some(i);
                }
                self.back_iter=None;
            }
            if let Some(back_inner_iter) = self.outer.next_back() {
                self.back_iter = Some(back_inner_iter.into_iter());
            } else {
                self.next_iter.as_mut()?.next_back();
            }
            // self.next_iter = Some(next_inner_iter);
        }
            //self.inner.as_mut().unwrap().next();
            // let mut inner_iter = inner_item.into_iter();
            // inner_iter.next()
            // self.outer.next().and_then(|inner| inner.into_iter().next())
            //None 
        
    } 

}


