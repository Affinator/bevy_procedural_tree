/*
* Inspiration taken with great thanks from: https://github.com/dgreenheck/ez-tree
*/

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BarkType {
  Birch,
  Oak,
  Pine,
  Willow
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LeafBillboard {
  Single,
  Double,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LeafType {
  Ash,
  Aspen,
  Pine,
  Oak,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TreeType {
  Deciduous,
  Evergreen,
}