use ::{Color, Property, Tech};
use std::collections::HashMap;


#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CrystalMapErrs<C, T>
  where C: Color,
        T: Tech,
{
  InconsistencyPropMappedToTech(C, C::Prop, T),
  InconsistencyTechMappedToProp(C, C::Prop, T),
  NoTechMappedToColor(C),
  NoTechMappedToProp(C, C::Prop),
  TechAlreadyMappedToProperty(C, C::Prop, T),
  TechNotFound(T),
}


pub struct CrystalMap<C, T>
  where C: Color,
        T: Tech,
{
  property_to_tech: HashMap<C, HashMap<C::Prop, Vec<T>> >,
  tech_to_properties: HashMap<T, HashMap<C, Vec<C::Prop>> >,
}

impl<C, T> CrystalMap<C, T>
  where C: Color,
        T: Tech,
{
  pub fn new() -> CrystalMap<C, T> {
    CrystalMap::<C, T> {
      property_to_tech: HashMap::with_capacity(C::num_colors() ),
      tech_to_properties: HashMap::with_capacity(T::init_size() ),
    }
  }

  pub fn add_mapping(&mut self, color: C, property: C::Prop, tech: T) -> Result<(), CrystalMapErrs<C, T> > {
    // Possible Errors:
    //   - InconsistencyPropMappedToTech(C, C::Prop, T) > This 'tech: T' has already been mapped to
    //                                                    (color: C, property: C::Prop) pair in the
    //                                                    C::Prop to T mapping but not in the
    //                                                    T to C::Prop mapping; this should never happen
    //   - InconsistencyTechMappedToProp(C, C::Prop, T) > This 'tech: T' has already been mapped to
    //                                                    (color: C, property: C::Prop) pair in the
    //                                                    T to C::Prop mapping but not in the
    //                                                    C::Prop to T mapping; this should never happen
    //   - TechAlreadyMappedToProperty(C, C::Prop, T)   > This 'tech: T' has already been mapped to the
    //                                                    (color: C, property: C::Prop) pair
    let rc1 = self.add_mapping_prop_tech(color, property, tech);
    let rc2 = self.add_mapping_tech_prop(color, property, tech);

    if rc1 == rc2 {
      rc1
    } else {
      rc1.or(Err(CrystalMapErrs::InconsistencyPropMappedToTech(color, property, tech)) ).
      and(Err(CrystalMapErrs::InconsistencyTechMappedToProp(color, property, tech)) )
    }
  }

  fn add_mapping_prop_tech(&mut self, color: C, property: C::Prop, tech: T) -> Result<(), CrystalMapErrs<C, T> > {
    // Possible Errors:
    //   - TechAlreadyMappedToProperty(C, C::Prop, T) > This 'tech: T' has already been mapped to the
    //                                                  (color: C, property: C::Prop) pair
    if !self.property_to_tech.contains_key(&color) {self.property_to_tech.insert(color, HashMap::new() );}
    let rmhm_p = self.property_to_tech.get_mut(&color).unwrap();

    if !rmhm_p.contains_key(&property) {rmhm_p.insert(property, Vec::new() );}
    let rmv_t = rmhm_p.get_mut(&property).unwrap();

    rmv_t.binary_search_by_key(&tech, |&t| t).
    and(Ok(Err(CrystalMapErrs::TechAlreadyMappedToProperty(color, property, tech))) ).
    or_else(|i| {rmv_t.insert(i, tech); Ok(Ok(()) )})?
  }

  fn add_mapping_tech_prop(&mut self, color: C, property: C::Prop, tech: T) -> Result<(), CrystalMapErrs<C, T> > {
    // Possible Errors:
    //   - TechAlreadyMappedToProperty(C, C::Prop, T) > This 'tech: T' has already been mapped to the
    //                                                  (color: C, property: C::Prop) pair
    if !self.tech_to_properties.contains_key(&tech) {self.tech_to_properties.insert(tech, HashMap::new() );}
    let rmhm_c = self.tech_to_properties.get_mut(&tech).unwrap();

    if !rmhm_c.contains_key(&color) {rmhm_c.insert(color, Vec::new() );}
    let rmv_p = rmhm_c.get_mut(&color).unwrap();

    rmv_p.binary_search_by_key(&property, |&p| p).
    and(Ok(Err(CrystalMapErrs::TechAlreadyMappedToProperty(color, property, tech))) ).
    or_else(|i| {rmv_p.insert(i, property); Ok(Ok(()) )})?
  }

  pub fn get_tech_list_from_property(&self, color: C, property: C::Prop) -> Result<&[T], CrystalMapErrs<C, T> > {
    // Possible Errors:
    //   - NoTechMappedToColor(C)         > There are no T associated with this 'color: C'
    //   - NoTechMappedToProp(C, C::Prop) > There are no T associated with this
    //                                      (color: C, property: C::Prop) pair
    self.property_to_tech.get(&color).
    ok_or(CrystalMapErrs::NoTechMappedToColor(color) ).
    and_then(|rhm_p|
      rhm_p.get(&property).
      ok_or(CrystalMapErrs::NoTechMappedToProp(color, property) ).
      map(|rv_t|
        &**rv_t
      )
    )
  }

  pub fn get_properties_for_tech(&self, tech: T) -> Result<&HashMap<C, Vec<C::Prop> >, CrystalMapErrs<C, T> > {
    // Possible Errors:
    //   - TechNotFound(T) > There is no listing for this 'tech: T' in the T to C::Prop mapping
    self.tech_to_properties.get(&tech).
    ok_or(CrystalMapErrs::TechNotFound(tech) )
  }
}


#[cfg(test)]
mod tests;
