#![deny(rustdoc::missing_crate_level_docs)]
#![warn(missing_docs)]
#![deny(missing_debug_implementations)]
#![doc = include_str!("../Readme.md")]
#![doc(html_logo_url = "https://raw.githubusercontent.com/oovm/shape-rs/dev/projects/images/Trapezohedron.svg")]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/oovm/shape-rs/dev/projects/images/Trapezohedron.svg")]

mod chebyshev;
mod damerau;
mod euclidean;
mod hamming;
mod levenshtein;
mod manhattan;

pub use crate::{
    chebyshev::ChebyshevDistance, euclidean::EuclideanDistance, hamming::HammingDistance, levenshtein::LevenshteinDistance,
    manhattan::ManhattanDistance,
};
