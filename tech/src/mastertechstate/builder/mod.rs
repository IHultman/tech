use ::{Color, Property, Tech};
use ::crystalmap::CrystalMap;
use ::techdigraph::MasterGraph;
use super::*;
use super::MasterTechStateErrs as MTSErrs;
use std::collections::HashMap;


pub struct MasterTechStateBuilder<C, T>
  where C: Color,
        T: Tech,
{
  crystalmap: CrystalMap<C, T>,
  properties: HashMap<C, Vec<C::Prop> >,
  mastergraph: MasterGraph<T>,
}

impl<C, T> MasterTechStateBuilder<C, T>
  where C: Color,
        T: Tech,
{
  pub fn new() -> Self {
    MasterTechStateBuilder {
      crystalmap: CrystalMap::new(),
      properties: HashMap::with_capacity(C::num_colors() ),
      mastergraph: MasterGraph::new(),
    }
  }

  fn add_property(&mut self, color: C, property: C::Prop) -> Result<(), MTSErrs<C, T> > {
    // Possible Errors:
    //   - PropertyAlreadyInsertedToList(C, C::Prop) > This (color: C, property: C::Prop) pair has already been
    //                                                 added to the list of 'properties'
    if !self.properties.contains_key(&color) {self.properties.insert(color, Vec::new() );}
    let rmv_p = self.properties.get_mut(&color).unwrap();
    rmv_p.binary_search_by_key(&property, |&p| p).
    and(Ok(Err(MTSErrs::PropertyAlreadyInsertedToList(color, property))) ).
    or_else(|i| -> Result<Result<(), MTSErrs<C, T>>, MTSErrs<C, T> > {
      rmv_p.insert(i, property); Ok(Ok(()) )
    })?
  }

  pub fn add_property_mapping(&mut self, color: C, property: C::Prop, tech: T) -> Result<(), MTSErrs<C, T> > {
    // Possible Errors:
    //   - TechAlreadyMappedToProperty(C, C::Prop, T) >
    self.is_property(color, property).or_else(|_| self.add_property(color, property) );
    self.mastergraph.add_prereq(tech);
    self.crystalmap.add_mapping(color, property, tech).map_err(|e| MTSErrs::from(e) )
  }

  pub fn add_tech_link(&mut self, t1: T, t2: T) -> Result<(), MTSErrs<C, T> > {
    // Possible Errors:
    //   - IllegalLink(T, T)                      > Attempting to link from some 't1: T' to an illegal
    //                                              target 't2: T'
    //   - LinkToTechAlreadyInsertedToGraph(T, T) > A link from some 't1: T' to another 't2: T' already
    //                                              exists
    //   - TechNotFound(T)                        > A node with tech_id 't1: T' cannot be found
    self.detect_cycle(t1, t2)?;
    self.mastergraph.add_advanced_link(t1, t2).
    map(|_| self.simplify(t2) ).
    map_err(|e| MTSErrs::<C, T>::from(e) )
  }

  pub fn build(self) -> MasterTechState<C, T> {
    MasterTechState::<C, T> {
      crystalmap: self.crystalmap,
      properties: self.properties,
      mastergraph: self.mastergraph,
    }
  }

  fn detect_cycle(&self, t1: T, t2: T) -> Result<(), MTSErrs<C, T> > {
    self.mastergraph.get_out_edges(t2).
    and_then(|rtl| {
      let mut vt_to_check = Vec::<T>::from(rtl);
      let mut vt_checked = Vec::<T>::from(rtl);
      while !vt_to_check.is_empty() {
        let t = vt_to_check.remove(0);
        if t == t1 {
          return Some(MTSErrs::CycleDetected);
        } else {
          self.mastergraph.get_out_edges(t).
          map(|rntl| {
            for &nt in rntl {
              vt_checked.binary_search(&nt).
              map_err(|j| {vt_checked.insert(j, nt); vt_to_check.push(nt); });
            }
          });
        }
      }
      None
    }).
    map_or(Ok(() ), |e| Err(e) )
  }

  fn is_property(&self, color: C, property: C::Prop) -> Result<(), MTSErrs<C, T> > {
    // Possible Errors:
    //   - PropertyNotFound(C, C::Prop) > There is no 'property: C::Prop' found to be associated
    //                                    with this 'color: C'
    self.properties.get(&color).
    and_then(|rv_p| rv_p.binary_search_by_key(&property, |&p| p).ok().map(|_| ()) ).
    ok_or(MTSErrs::PropertyNotFound(color, property) )
  }

  fn leads_to(&self, t1: T, t2: T) -> bool {
    self.mastergraph.get_out_edges(t1).
    and_then(|rtl_t1| {
      let mut vt_to_check = Vec::from(rtl_t1);
      while !vt_to_check.is_empty() {
        let nt = vt_to_check.remove(0);
        if t2 == nt {
          return Some(() );
        } else {
          self.mastergraph.get_out_edges(nt).
          map(|rtl_nt| {
            for &t in rtl_nt {
              vt_to_check.push(t);
            }
          });
        }
      }
      None
    }).is_some()
  }

  fn simplify(&mut self, tech: T) {
    let mut vt_to_simplify = vec![tech];
    let mut vt_checked = vec![tech];
    let mut i: usize = 0;
    while i < vt_to_simplify.len() {
      let t = vt_to_simplify[i];
      self.mastergraph.get_out_edges(t).
      map(|rtl_t| {
        for &nt in rtl_t {
          vt_checked.binary_search(&nt).
          map_err(|j| {
            vt_checked.insert(j, nt);
            vt_to_simplify.push(nt);
          });
        }
      });
      i += 1;
    }
    for t in vt_to_simplify {
      self.simplify_back(t);
    }
  }

  fn simplify_back(&mut self, tech: T) {
    self.mastergraph.get_in_edges(tech).
    map(|rtl_tech|
      rtl_tech.into_iter().
      filter(|&&pt|
        self.mastergraph.get_out_edges(pt).
        and_then(|rtl_pt| {
          for &ptnt in rtl_pt.into_iter().filter(|&&nt| nt != tech) {
            if self.leads_to(ptnt, tech) {return Some(() );}
          }
          None
        }).is_some()
      ).map(|&t| t).collect::<Vec<T> >()
    ).
    map(|to_destroy| {
      for pt in to_destroy {
        self.mastergraph.destroy_link(pt, tech);
      }
    });
  }
}


#[cfg(test)]
mod tests;
