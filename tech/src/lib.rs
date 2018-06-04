extern crate rand;


use std::cmp::{PartialEq, Eq, PartialOrd, Ord};
use std::hash::Hash;
use std::fmt::Debug;


pub mod crystalmap;
pub mod mastertechstate;
pub mod techdigraph;


pub trait Color
  where Self: Clone + Copy + Debug + Hash + PartialEq + Eq + PartialOrd + Ord
{
  type Prop: Property;

  fn num_colors() -> usize;
}


pub trait Property
  where Self: Clone + Copy + Debug + Hash + PartialEq + Eq + PartialOrd + Ord
{

}


pub trait Tech
  where Self: Clone + Copy + Debug + Hash + PartialEq + Eq + PartialOrd + Ord
{
  fn init_size() -> usize;
}


impl Property for usize {}


#[cfg(test)]
mod tests;
