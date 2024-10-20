#[macro_export]
macro_rules! impl_dist {
    ( $( $x:ty ),* ) => {
        $(impl Dist<f64> for $x {
            fn density(&self) -> Box<dyn Fn(f64) -> f64 + '_> {
                Box::new(|x| self.pdf(x))
            }
        })*
    }
}

// #[macro_export]
// macro_rules! impl_dist_disc {
// ( $( ($x: ty, $it: ty)),* ) => {
//         $(impl Dist for $x {
//             type VariateT = $it;
//             fn density(&self) -> Box<dyn Fn(Self::VariateT) -> f64 + '_> {
//                 Box::new(|x| self.pmf(x))
//             }
//         })*
//     };
//     ( $( $x: ty ),* ) => {
//         $(impl Dist for $x {
//             type VariateT = u64;
//             fn density(&self) -> Box<dyn Fn(Self::VariateT) -> f64 + '_> {
//                 Box::new(|x| self.pmf(x))
//             }
//         })*
//     };

// }
