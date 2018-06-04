pub trait Experience
  where Self: Add<Self>
{
  fn base_exp() -> Self;
}


pub trait Level {
  fn base_lvl() -> Self;

  fn lvl_up(&mut self, new: Self) {
    *self = new;
  }
}


pub trait LevelUp {
  type Exp: Experience;
  type Lvl: Level;

  fn can_add_exp(&self) -> bool;
  fn get_lvl_state_mut(&mut self) -> &mut LevelState<Self>;
  fn get_lvl_state_ref(&self) -> &LevelState<Self>;
  fn lvl_up(&mut self) -> Self::Lvl;

  fn new_lvl_state<F>(f: F) -> LevelState<Self>
    where F: Fn(&Self::Exp, &Self::Lvl) -> Option<u32>
  {
    LevelState::<Self>::new(Self::Lvl::base_lvl(), Self::Exp::base_exp(), f)
  }

  fn add_exp(&mut self, exp: Self::Exp) {
    if self.can_add_exp() {
      self.get_lvl_state_mut().add_exp(exp).
      map(|count| {
        for _ in 0..count {

        }
      })
    }
  }
}


pub struct LevelState<T, F>
  where T: LevelUp,
        F: Fn(&T::Exp, &T::Lvl) -> Option<u32>
{
  exp: T::Exp,
  level: T::Lvl,
  exp_fn: F,
}

impl<T, F> LevelState<T, F>
  where T: LevelUp,
        F: Fn(&T::Exp, &T::Lvl) -> Option<u32>
{
  fn new(lvl: T::Lvl, exp: T::Exp, f: F) -> LevelState<T> {
    exp: exp,
    lvl: lvl,
    exp_fn: f,
  }

  fn add_exp(&mut self, exp: T::Exp -> Option<u32> {
    self.exp = self.exp + exp;
    self.exp_fn(self.get_exp(), self.get_lvl() )
  }

  #[inline(always)]
  pub fn get_exp(&self) -> &T::Exp {
    &self.exp
  }

  #[inline(always)]
  pub fn get_lvl(&self) -> &T::Lvl {
    &self.lvl
  }

  #[inline(always)]
  fn set_lvl(&mut self, new_lvl: T::Lvl) {
    self.lvl = new_lvl;
  }
}
