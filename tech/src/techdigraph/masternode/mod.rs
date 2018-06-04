use ::Tech;
use super::node::*;
use super::node::MasterNode as MNode;


pub struct MasterNode<T>
  where T: Tech
{
  tech: T,
  prereq: bool,
  in_edges: Option<Vec<T> >,
  out_edges: Option<Vec<T> >,
}


impl<T> MasterNode<T>
  where T: Tech
{
  #[inline(always)]
  fn get_in_edges_mut(&mut self) -> &mut Option<Vec<T> > {
    &mut self.in_edges
  }

  #[inline(always)]
  fn get_out_edges_mut(&mut self) -> &mut Option<Vec<T> > {
    &mut self.out_edges
  }
}


impl<T> Node for MasterNode<T>
  where T: Tech
{
  type Tech = T;
  type InEdgeArgs = T;

  fn new(tech: T, prereq: bool) -> MasterNode<T> {
    MasterNode::<T> {
      tech: tech,
      prereq: prereq,
      in_edges: None,
      out_edges: None,
    }
  }

  fn add_in_edge(&mut self, tech: Self::InEdgeArgs) -> Result<(), NodeErrs<T> > {
    // Possible Errors:
    //   - AttemptToLinkToPrereq(T, T)    > This node 'self' is marked as a prereq and cannot
    //                                      have anything in its 'in_edges' list
    //   - AttemptToLinkToSelf(T)         > Attempting to link from some node with the same
    //                                      'tech_id' as 'self'
    //   - InEdgesTechAlreadyExists(T, T) > The input 'tech: T' has already been added to the
    //                                      'in_edges' list
    let t_name = self.get_tech_id();
    if t_name == tech {
      return Err(NodeErrs::AttemptToLinkToSelf(tech) );
    }

    if self.is_prereq() {
      return Err(NodeErrs::AttemptToLinkToPrereq(tech, t_name) );
    }

    let ui_edges: &mut Vec<T> = self.get_in_edges_mut().
                                get_or_insert(Vec::with_capacity(5) );
    ui_edges.binary_search(&tech).
    map(|_| true).
    or_else(|i| {
      ui_edges.insert(i, tech);
      Ok(false)
    }).
    and_then(|p|
      if p {
        Err(NodeErrs::InEdgesTechAlreadyExists(tech, t_name) )
      } else {
        Ok(() )
    })
  }

  fn add_out_edge(&mut self, tech: T) -> Result<(), NodeErrs<T> > {
    // Possible Errors:
    //   - AttemptToLinkToSelf(T)          > Attempting to link from this node 'self' to some other
    //                                       node with the same 'tech_id'
    //   - OutEdgesTechAlreadyExists(T, T) > The input 'tech: T' has already been added to the
    //                                       'out_edges' list
    let t_name = self.get_tech_id();
    if t_name == tech {
      return Err(NodeErrs::AttemptToLinkToSelf(tech) );
    }

    let o_edges: &mut Vec<T> = self.get_out_edges_mut().
                               get_or_insert(Vec::with_capacity(5) );
    o_edges.binary_search(&tech).
    map(|_| true).
    or_else(|i| {
      o_edges.insert(i, tech);
      Ok(false)
    }).
    and_then(|p|
      if p {
        Err(NodeErrs::OutEdgesTechAlreadyExists(t_name, tech) )
      } else {
        Ok(() )
    })
  }

  #[inline(always)]
  fn get_in_edge_args(&self) -> T {
    self.get_tech_id()
  }

  #[inline(always)]
  fn get_out_edges_ref(&self) -> &Option<Vec<T> > {
    &self.out_edges
  }

  #[inline(always)]
  fn get_tech_id(&self) -> T {
    self.tech
  }

  #[inline(always)]
  fn is_prereq(&self) -> bool {
    self.prereq
  }
}


impl<T> MNode for MasterNode<T>
  where T: Tech
{
  #[inline(always)]
  fn get_in_edges_ref(&self) -> &Option<Vec<T> > {
    &self.in_edges
  }

  fn remove_in_link(&mut self, t: Self::Tech) -> Result<(), NodeErrs<T> > {
    let t_name = self.get_tech_id();
    self.get_in_edges_mut().as_mut().
    and_then(|rm_ie| rm_ie.binary_search(&t).map(|i| rm_ie.remove(i) ).ok() ).
    map_or(Err(NodeErrs::NoLinkToRemove(t, t_name)), |_| Ok(()) )
  }

  fn remove_out_link(&mut self, t: Self::Tech) -> Result<(), NodeErrs<T> > {
    let t_name = self.get_tech_id();
    self.get_out_edges_mut().as_mut().
    and_then(|rm_oe| rm_oe.binary_search(&t).map(|i| rm_oe.remove(i) ).ok() ).
    map_or(Err(NodeErrs::NoLinkToRemove(t, t_name)), |_| Ok(()) )
  }
}


#[cfg(test)]
mod tests;
