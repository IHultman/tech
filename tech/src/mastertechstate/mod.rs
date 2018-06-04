use ::{Color, Property, Tech};
use ::crystalmap::*;
use ::crystalmap::CrystalMapErrs as CMErrs;
use ::techdigraph::{MasterGraph, PlayerGraph};
use ::techdigraph::TechDiGraphErrs as TDGErrs;
use self::MasterTechStateErrs as MTSErrs;
use self::playertechstate::PlayerTechState;
use std::collections::HashMap;


pub mod builder;
pub mod playertechstate;


#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MasterTechStateErrs<C, T>
  where C: Color,
        T: Tech,
{
  CycleDetected,
  IllegalLink(T, T),                          //builder
  LinkToTechAlreadyInsertedToGraph(T, T),     //builder
  NoTechMappedToColor(C),                     //raised
  NoTechMappedToProp(C, C::Prop),             //raised
  PrereqAlreadyInsertedToGraph(T),            //builder
  PropertyAlreadyInsertedToList(C, C::Prop),  //builder
  PropertyNotFound(C, C::Prop),               //builder
  TechAlreadyMappedToProperty(C, C::Prop, T), //builder
  TechNotFound(T),                            //raised
}

impl<C, T> From<CMErrs<C, T> > for MasterTechStateErrs<C, T>
  where C: Color,
        T: Tech,
{
  fn from(err: CMErrs<C, T>) -> Self {
    match err {
      CMErrs::NoTechMappedToColor(c)               => MTSErrs::NoTechMappedToColor(c),
      CMErrs::NoTechMappedToProp(c, p)             => MTSErrs::NoTechMappedToProp(c, p),
      CMErrs::TechAlreadyMappedToProperty(c, p, t) => MTSErrs::TechAlreadyMappedToProperty(c, p, t),
      CMErrs::TechNotFound(t)                      => MTSErrs::TechNotFound(t),
      _                                            => panic!(),
    }
  }
}

impl<C, T> From<TDGErrs<T> > for MasterTechStateErrs<C, T>
  where C: Color,
        T: Tech,
{
  fn from(err: TDGErrs<T>) -> Self {
    match err {
      TDGErrs::IllegalLink(t1, t2)                      => MTSErrs::IllegalLink(t1, t2),
      TDGErrs::LinkToTechAlreadyInsertedToGraph(t1, t2) => MTSErrs::LinkToTechAlreadyInsertedToGraph(t1, t2),
      TDGErrs::PrereqAlreadyInsertedToGraph(t)          => MTSErrs::PrereqAlreadyInsertedToGraph(t),
      TDGErrs::TechNotFound(t)                          => MTSErrs::TechNotFound(t),
      _                                                 => panic!(),
    }
  }
}


pub struct MasterTechState<C, T>
  where C: Color,
        T: Tech,
{
  crystalmap: CrystalMap<C, T>,
  properties: HashMap<C, Vec<C::Prop>>,
  mastergraph: MasterGraph<T>,
}

impl<C, T> MasterTechState<C, T>
  where C: Color,
        T: Tech,
{
  pub fn get_advanced_tech_list(&self, tech: T) -> Option<&[T]> {
    self.mastergraph.get_out_edges(tech)
  }

  pub fn get_prereq_tech_list(&self, tech: T) -> Option<&[T]> {
    self.mastergraph.get_in_edges(tech)
  }

  pub fn get_properties_for_tech(&self, tech: T) -> Result<&HashMap<C, Vec<C::Prop> >, MTSErrs<C, T> > {
    // Possible Errors:
    //   - TechNotFound(T) > This 'tech: T' is not listed as being associated with any (C, C::Prop)
    self.crystalmap.get_properties_for_tech(tech).map_err(|e| MTSErrs::<C, T>::from(e) )
  }

  pub fn get_properties(&self) -> &HashMap<C, Vec<C::Prop> > {
    &self.properties
  }

  pub fn get_tech_list_from_property(&self, color: C, property: C::Prop) -> Result<&[T], MTSErrs<C, T> > {
    // Possible Errors:
    //   - NoTechMappedToColor(C)         > There are no T associated with this 'color: C'
    //   - NoTechMappedToProp(C, C::Prop) > There are no T associated with this
    //                                      (color: C, property: C::Prop) pair
    self.crystalmap.get_tech_list_from_property(color, property).map_err(|e| MTSErrs::<C, T>::from(e) )
  }

  pub fn mk_new_player(&self) -> PlayerTechState<C, T> {
    PlayerTechState::<C, T>::new(self)
  }
}


#[cfg(test)]
mod tests;
