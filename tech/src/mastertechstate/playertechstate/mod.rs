use ::{Color, Property, Tech};
use ::techdigraph::PlayerGraph;
use ::techdigraph::TechDiGraphErrs as TDGErrs;
use self::PlayerTechStateErrs as PTSErrs;
use super::{MasterTechState, MasterTechStateErrs};
use std::collections::HashMap;


mod colorstate;


#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PlayerTechStateErrs<C, T>
  where C: Color,
        T: Tech,
{
  AllPropertiesDiscovered,
  AllPropertiesToColorDiscovered(C),
  OtherEvent,
  PropertyNotDiscovered(C, C::Prop), //suppressed
  TechAlreadyResearched(T),
  TechNotAvailable(T),
  TechNotFound(T),
}

impl<C, T> From<TDGErrs<T> > for PlayerTechStateErrs<C, T>
  where C: Color,
        T: Tech,
{
  fn from(err: TDGErrs<T>) -> Self {
    match err {
      TDGErrs::TechAlreadyResearched(t) => PTSErrs::TechAlreadyResearched(t),
      TDGErrs::TechNotAvailable(t)      => PTSErrs::TechNotAvailable(t),
      TDGErrs::TechNotFound(t)          => PTSErrs::TechNotFound(t),
      _                                 => panic!(),
    }
  }
}


pub struct PlayerTechState<'a, C, T>
  where C: Color + 'a,
        T: Tech + 'a,
{
  mts: &'a MasterTechState<C, T>,
  properties_discovered: Option<HashMap<C, Vec<C::Prop>> >,
  properties_undiscovered: Option<HashMap<C, Vec<C::Prop>> >,
  player_graph: PlayerGraph<T>,
}

impl<'a, C, T> PlayerTechState<'a, C, T>
  where C: Color + 'a,
        T: Tech + 'a,
{
  pub fn new(mts: &MasterTechState<C, T>) -> PlayerTechState<C, T> {
    PlayerTechState::<C, T> {
      mts: mts,
      properties_discovered: None,
      properties_undiscovered: Some(mts.get_properties().clone() ),
      player_graph: PlayerGraph::<T>::new(),
    }
  }

  fn add_tech_from_prereq(player_graph: &mut PlayerGraph<T>, mts: &MasterTechState<C, T>, prereq: T) {
    player_graph.add_prereq(prereq).unwrap();
    mts.get_advanced_tech_list(prereq).
    map(|rtl| {
      let mut vt_to_check = Vec::<T>::from(rtl);
      while !vt_to_check.is_empty() {
        let t = vt_to_check.remove(0);
        mts.get_prereq_tech_list(t).
        map(|rtl_p| {
          if rtl_p.iter().fold(true, |reqs_met, &t| player_graph.check_tech(t).is_ok() && reqs_met) {
            for &tp in rtl_p {
              player_graph.add_advanced_link(tp, t).unwrap();
            }
            mts.get_advanced_tech_list(t).
            map(|rtl_a| {
              for &ta in rtl_a {
                vt_to_check.push(ta);
              }
            });
          }
        });
      }
    });
  }

  pub fn discover_prop_and_update(&mut self, color: C) -> Result<(), PTSErrs<C, T> > {
    // Possible Errors:
    //   - AllPropertiesDiscovered           > See `fn discover_property_rand()` for details
    //   - AllPropertiesToColorDiscovered(C) > See `fn discover_property_rand()` for details

    self.discover_property_rand(color).
    map(|prop| self.update_graph_to_property(prop) )
  }

  fn discover_property_rand(&mut self, color: C) -> Result<(C, C::Prop), PTSErrs<C, T> > {
    // Possible Errors:
    //   - AllPropertiesDiscovered               > All C::prop for all C have been discovered
    //   - AllPropertiesToColorDiscovered(C)     > All C::Prop for a certain C have been discovered
    let (rmp_discovered, rmp_undiscovered) = (&mut self.properties_discovered, &mut self.properties_undiscovered);
    rmp_undiscovered.as_mut().
    ok_or(PTSErrs::AllPropertiesDiscovered).
    and_then(|rmhm_pu| {
      if rmhm_pu.is_empty() {
        return Ok((true, Err(PTSErrs::AllPropertiesDiscovered)) );
      }
      rmhm_pu.get_mut(&color).
      ok_or(PTSErrs::AllPropertiesToColorDiscovered(color) ).
      map(|rmv_pu| {
          let mut result = (false, Err(PTSErrs::OtherEvent) );
          let mut found_new_or_none_left = false;
          while !found_new_or_none_left {
            let l = rmv_pu.len();
            result = if l < 1 {
              found_new_or_none_left = true;
              (true, Err(PTSErrs::AllPropertiesToColorDiscovered(color)) )
            } else {(
              (l == 1), {
                let prop_rand = rmv_pu.remove(::rand::random::<usize>() % l);
                let rmhm_pd = rmp_discovered.get_or_insert(HashMap::with_capacity(C::num_colors()) );
                if !rmhm_pd.contains_key(&color) {rmhm_pd.insert(color, Vec::new() );}
                let rmv_pd = rmhm_pd.get_mut(&color).unwrap();
                rmv_pd.binary_search_by_key(&prop_rand, |&p| p).
                or_else(|i| {rmv_pd.insert(i, prop_rand); found_new_or_none_left = true; Ok(i) }).
                and(Ok((color, prop_rand)) )
              }
            )};
          }
          result
      }).
      map(|(empty, result)| {
        if empty {rmhm_pu.remove(&color);}
        (rmhm_pu.is_empty(), result)
      })
    }).
    and_then(|(empty, mut result)| {
      if empty {
        *rmp_undiscovered = None;
        if result.is_err() {result = Err(PTSErrs::AllPropertiesDiscovered);}
      }
      result
    })
  }

  pub fn get_properties_discovered(&self, color: C) -> Option<&[C::Prop]> {
    self.properties_discovered.as_ref().
    and_then(|rhm| rhm.get(&color).map(|rvp| &**rvp) )
  }

  pub fn research_tech(&mut self, tech: T) -> Result<(), PTSErrs<C, T> > {
    //Possible Errors:
    //   - TechAlreadyResearched(T) > a
    //   - TechNotAvailable(T)      > a
    //   - TechNotFound(T)          > a

    //explicit UFCS
    //--------------------------------------------------vvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvvv
    self.player_graph.mark_researched(tech).map_err(|e| <PTSErrs<C, T> as From<TDGErrs<T>> >::from(e) )
    //this would result in ambiguity ---------------------->vvvvvvvvv
    //self.player_graph.mark_researched(prereq).map_err(|e| From::<TDGErrs<T> >::from(e) );
    //as both PTSErrs and MTSErrs impl `From<TDGErrs<T> >`
  }

  // interesting syntax ----------------------------------vvvvvvvvvvvvvv
  fn update_graph_to_property(&mut self, (color, property): (C, C::Prop) ) {
    let mts = self.mts;
    let player_graph = &mut self.player_graph;
    let rp_discovered = &self.properties_discovered;
    mts.get_tech_list_from_property(color, property).
    map(|rtl| {
      for &t in rtl {
        mts.get_properties_for_tech(t).
        map(|rhm_p|
          if {
            rhm_p.iter().fold(true, |reqs_met, (&c, rvp)|
              rvp.iter().fold(true, |reqs_met, &p|
                PlayerTechState::<C, T>::check_property_requisites(rp_discovered, c, p).is_ok() && reqs_met
              ) && reqs_met
            )
          } {
            PlayerTechState::add_tech_from_prereq(player_graph, mts, t);
          }
        );
      }
    });
  }

  fn check_property_requisites(properties_discovered: &Option<HashMap<C, Vec<C::Prop>> >, color: C, property: C::Prop) -> Result<(), PTSErrs<C, T> > {
    // Possible Errors:
    //   - PropertyNotDiscovered(C, C::Prop) > C::Prop cannot be found in the 'discovered' list
    properties_discovered.as_ref().
    and_then(|rhm_p|
      rhm_p.get(&color).
      and_then(|rvp| rvp.binary_search_by_key(&property, |&p| p).ok() )
    ).
    ok_or(PTSErrs::PropertyNotDiscovered(color, property) ).
    and(Ok(()) )
  }
}


#[cfg(test)]
mod tests;
