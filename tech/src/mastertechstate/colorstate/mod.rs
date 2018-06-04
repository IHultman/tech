use ::{Color, Property, Tech};
use std::collections::HashMap;

pub struct ColorState<C, T, F>
  where C: Color,
        T: Tech,
        F: Fn(u64, &[C::Prop]) -> u8,
{
  exp_fn: F,
  exp_list: HashMap<C, u64>,
  properties_discovered: Option<HashMap<C, Vec<C::Prop>> >,
  properties_undiscovered: Option<HashMap<C, Vec<C::Prop>> >,
}

impl<C, T, F> ColorState<C, T, F>
  where C: Color,
        T: Tech,
        F: Fn(u64, &[C::Prop]) -> u8,
{
  pub fn new(props: HashMap<C, Vec<C::Prop> >, exp_fn: F) -> ColorState<C, T, F> {
    ColorState::<C, T> {
      exp_fn: exp_fn,
      exp_list: props.iter().map(|(&color, _)| (color, 0) ).collect(),
      properties_discovered: None,
      properties_undiscovered: Some(props).
    }
  }

  pub fn add_exp(&mut self, exp: u32, color: C) -> Option<u8> {
    let props_u = &self.properties_undiscovered;
    self.exp_list.get_mut(&color).
    and_then(|curr_exp| {
      properties_u.as_ref().
      and_then(|rhm|
        rhm.get(&color).and_then(|rv|
          if !rv.is_empty() {
            *curr_exp += exp as u64;
            Some(*curr_exp)
          } else {
            None
          }
        )
      )
    }).
    and_then(|curr_exp|
      self.properties_discovered.as_ref().
      and_then(|rhm|
        rhm.get(&color).
        map(|rv|
          self.exp_fn(curr_exp, rv)
        )
      ).
      or_else(|| Some(self.exp_fn(curr_exp, &[])) )
    )
  }
}
