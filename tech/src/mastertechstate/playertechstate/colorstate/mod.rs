use ::Property;


pub struct PropertyState<P>
  where P: Property,
{
  properties_discovered: Option<Vec<P> >,
  properties_undiscovered: Option<Vec<P> >,
}

impl<P> PropertyState<P>
  where P: Property,
{
  fn new(props: Vec<P>) -> PropertyState<P> {
    PropertyState::<P> {
      properties_discovered: None,
      properties_undiscovered: Some(props),
    }
  }

  pub fn get_discovered(&self) -> Option<&Vec<P> > {
    self.properties_undiscovered.as_ref()
  }

  pub fn get_undiscovered(&self) -> Option<&Vec<P> > {
    self.properties_undiscovered.as_ref()
  }

  fn can_lvl_up(&self) -> bool {
    self.get_undiscovered().and_then(|rv| if rv.is_empty() {None} else {Some(() )}).is_some()
  }

  fn lvl_up(&mut self, i: usize) -> Option<&P> {
    self.properties_undiscovered.as_mut().
    map(|rmv| {
      let ret_val = if rmv.len() > i {Some(rmv.remove(i) )} else {None};
      (rmv.is_empty(), ret_val)
    }).
    and_then(|(empty, prop)| {
      if empty {self.properties_undiscovered = None;}
      prop
    }).
    map(move |prop| {
      let rmv = self.properties_discovered.get_or_insert(Vec::new() );
      let j = rmv.binary_search(&prop).
              or_else(|j| -> Result<usize, usize> {rmv.insert(j, prop); Ok(j)}).
              unwrap();
      &rmv[j]
    })
  }
}


/*
pub struct LevelState<F1, F2, F3, P>
  where F1: Fn(&PropertyState<P>, u64) -> bool,
        F2: Fn(&PropertyState<P>, u64) -> Option<(u64, usize)>,
        F3: Fn(&mut PropertyState<P>, usize) -> Option<P>,
        P: Property,
{
  exp: u64,
  can_lvl_up: F1,
  cost_fn: F2,
  lvl_up_fn: F3,
  phantom: PhantomData<P>
}

impl<F1, F2, F3, P> LevelState<F1, F2, F3, P>
  where F1: Fn(&PropertyState<P>, u64) -> bool,
        F2: Fn(&PropertyState<P>, u64) -> Option<(u64, usize)>,
        F3: Fn(&mut PropertyState<P>, usize) -> Option<P>,
        P: Property,
{
  pub fn new(exp: u64, f1: F1, f2: F2, f3: F3) -> LevelState<F1, F2, F3, P> {
    LevelState::<F1, F2, F3, P> {
      exp: exp,
      can_lvl_up: f1,
      cost_fn: f2,
      lvl_up_fn: f3,
      phantom: PhantomData,
    }
  }

  pub fn add_exp(&mut self, exp: u64, cs: &PropertyState<P>) -> bool {
    if (self.can_lvl_up)(cs, self.exp) {
      self.exp += exp;
      true
    } else {
      false
    }
  }

  pub fn lvl_up(&self, cs: &mut PropertyState<P>) -> Option<P> {
    (self.cost_fn)(cs, self.exp).
    and_then(|(cost, i)| {
      if self.exp > cost {
        (self.lvl_up_fn)(cs, i)
      } else {
        None
      }
    })
  }
}
*/


struct PropertyLevelState<P>
  where P: Property
{
  exp:        u64,
  can_lvl_up: fn(&PropertyState<P>) -> bool,
  cost_fn:    fn(&PropertyState<P>, u64) -> Option<(u64, usize)>,
  lvl_up_fn:  fn(&mut PropertyState<P>, usize) -> Option<&P>,
}

impl<P> PropertyLevelState<P>
  where P: Property
{
  fn new(exp: u64, f: fn(&PropertyState<P>, u64) -> Option<(u64, usize)>) -> Self {
    PropertyLevelState::<P> {
      exp:        exp,
      can_lvl_up: PropertyState::<P>::can_lvl_up,
      cost_fn:    f,
      lvl_up_fn:  PropertyState::<P>::lvl_up,
    }
  }

  fn add_exp(&mut self, exp: u64, cs: &PropertyState<P>) -> bool {
    if (self.can_lvl_up)(cs) {
      self.exp += exp;
      true
    } else {
      false
    }
  }

  fn lvl_up<'a>(&self, cs: &'a mut PropertyState<P>) -> Option<&'a P> {
    (self.cost_fn)(cs, self.exp).
    and_then(move |(cost, i)| {
      if self.exp > cost {
        (self.lvl_up_fn)(cs, i)
      } else {
        None
      }
    })
  }
}


pub struct ColorState<P>
  where P: Property,
{
  propstate: PropertyState<P>,
  lvl: PropertyLevelState<P>,
}

impl<P> ColorState<P>
  where P: Property,
{
  pub fn new(exp: u64, props: Vec<P>, f: fn(&PropertyState<P>, u64) -> Option<(u64, usize)>) -> Self {
    ColorState {
      propstate: PropertyState::new(props),
      lvl: PropertyLevelState::new(exp, f),
    }
  }

  pub fn add_exp(&mut self, exp: u64) -> bool {
    self.lvl.add_exp(exp, &self.propstate)
  }

  pub fn lvl_up(&mut self) -> Option<&P> {
    self.lvl.lvl_up(&mut self.propstate)
  }
}
