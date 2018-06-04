use ::Tech;
use super::node::*;
use super::node::PlayerNode as PNode;


pub struct PlayerNode<T>
  where T: Tech
{
  tech: T,
  available: bool,
  prereq: bool,
  researched: bool,
  acquired_in_edges: Option<Vec<T> >,
  unacquired_in_edges: Option<Vec<T> >,
  out_edges: Option<Vec<T> >,
}


impl<T> PlayerNode<T>
  where T: Tech
{
  fn add_in_edge_acquired(&mut self, tech: T) -> Result<(), NodeErrs<T> > {
    // Possible Errors:
    //   - AttemptToLinkToPrereq(T, T)    > This node 'self' is marked as a prereq and cannot
    //                                      have anything in its 'in_edges' lists
    //   - AttemptToLinkToSelf(T)         > Attempting to link from some node with the same
    //                                      'tech_id' as 'self'
    //   - InEdgesTechAlreadyExists(T, T) > The input 'tech: T' has already been added to the
    //                                      'in_edges' lists
    let t_name = self.get_tech_id();
    if t_name == tech {
      return Err(NodeErrs::AttemptToLinkToSelf(tech) );
    }

    if self.is_prereq() {
      return Err(NodeErrs::AttemptToLinkToPrereq(tech, t_name) );
    }

    self.get_unacquired_in_edges_ref().as_ref().
    and_then(|rv_ui_edges| rv_ui_edges.binary_search(&tech).ok().and(Some(true)) ).
    or(Some(false) ).
    and_then(|unacquired|
      if unacquired {self.move_link_acquired(tech); None}
      else {Some(() )}
    ).
    ok_or(NodeErrs::InEdgesTechAlreadyExists(tech, t_name) )?;

    let rmv_ai_edges = self.get_acquired_in_edges_mut().get_or_insert(Vec::with_capacity(5) );
    rmv_ai_edges.binary_search(&tech).
    and(Ok(true) ).
    or_else(|i| {rmv_ai_edges.insert(i, tech); Ok(false)}).
    and_then(|already_acquired|
      if already_acquired {
        Err(NodeErrs::InEdgesTechAlreadyExists(tech, t_name) )
      } else {
        Ok(() )
    })
  }

  fn add_in_edge_unacquired(&mut self, tech: T) -> Result<(), NodeErrs<T> > {
    // Possible Errors:
    //   - AttemptToLinkToPrereq(T, T)    > This node 'self' is marked as a prereq and cannot
    //                                      have anything in its 'in_edges' lists
    //   - AttemptToLinkToSelf(T)         > Attempting to link from some node with the same
    //                                      'tech_id' as 'self'
    //   - InEdgesTechAlreadyExists(T, T) > The input 'tech: T' has already been added to the
    //                                      'in_edges' lists
    let t_name = self.get_tech_id();
    if t_name == tech {
      return Err(NodeErrs::AttemptToLinkToSelf(tech) );
    }

    if self.is_prereq() {
      return Err(NodeErrs::AttemptToLinkToPrereq(tech, t_name) );
    }

    self.get_acquired_in_edges_ref().as_ref().
    and_then(|rv_ai_edges| rv_ai_edges.binary_search(&tech).ok().and(Some(true)) ).
    or(Some(false) ).
    and_then(|acquired| if acquired {None} else {Some(() )}).
    ok_or(NodeErrs::InEdgesTechAlreadyExists(tech, t_name) )?;

    *self.get_availability_mut() = false;
    let rmv_ui_edges: &mut Vec<T> = self.get_unacquired_in_edges_mut().get_or_insert(Vec::with_capacity(5) );
    rmv_ui_edges.binary_search(&tech).
    and(Ok(true) ).
    or_else(|i| {
      rmv_ui_edges.insert(i, tech);
      Ok(false)
    }).
    and_then(|already_present|
      if already_present {
        Err(NodeErrs::InEdgesTechAlreadyExists(tech, t_name) )
      } else {
        Ok(() )
    })
  }

  #[inline(always)]
  pub fn get_acquired_in_edges_ref(&self) -> &Option<Vec<T> > {
    &self.acquired_in_edges
  }

  #[inline(always)]
  fn get_acquired_in_edges_mut(&mut self) -> &mut Option<Vec<T> > {
    &mut self.acquired_in_edges
  }

  #[inline(always)]
  fn get_availability_mut(&mut self) -> &mut bool {
    &mut self.available
  }

  #[inline(always)]
  fn get_out_edges_mut(&mut self) -> &mut Option<Vec<T> > {
    &mut self.out_edges
  }

  #[inline(always)]
  fn get_researched_mut(&mut self) -> &mut bool {
    &mut self.researched
  }

  #[inline(always)]
  pub fn get_unacquired_in_edges_ref(&self) -> &Option<Vec<T> > {
    &self.unacquired_in_edges
  }

  #[inline(always)]
  fn get_unacquired_in_edges_mut(&mut self) -> &mut Option<Vec<T> > {
    &mut self.unacquired_in_edges
  }

  #[inline(always)]
  pub fn is_available(&self) -> bool {
    self.available
  }
}


impl<T> Node for PlayerNode<T>
  where T: Tech
{
  type Tech = T;
  type InEdgeArgs = (T, bool);

  fn new(tech: T, prereq: bool) -> PlayerNode<T> {
    PlayerNode::<T> {
      tech: tech,
      available: true,
      prereq: prereq,
      researched: false,
      acquired_in_edges: None,
      unacquired_in_edges: None,
      out_edges: None,
    }
  }

  fn add_in_edge(&mut self, (tech, is_researched): Self::InEdgeArgs) -> Result<(), NodeErrs<T> > {
    if is_researched {
      self.add_in_edge_acquired(tech)
    } else {
      self.add_in_edge_unacquired(tech)
    }
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

    let o_edges: &mut Vec<T> = self.get_out_edges_mut().get_or_insert(Vec::with_capacity(5) );
    o_edges.binary_search(&tech).
    and(Ok(true) ).
    or_else(|i| {
      o_edges.insert(i, tech);
      Ok(false)
    }).
    and_then(|already_present|
      if already_present {
        Err(NodeErrs::OutEdgesTechAlreadyExists(t_name, tech) )
      } else {
        Ok(() )
    })
  }

  #[inline(always)]
  fn get_in_edge_args(&self) -> (T, bool) {
    (self.get_tech_id(), self.is_researched() )
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


impl<T> PNode for PlayerNode<T>
  where T: Tech
{
  fn is_researched(&self) -> bool {
    self.researched
  }

  fn mark_researched(&mut self) -> Result<(), NodeErrs<T> > {
    // Possible Errors:
    //   - IllegallyMarkedAvailable(T)    > This node 'self' was marked available with its
    //                                      'unacquired' list in an 'Option::Some(Vec<T>)' state;
    //                                      it should be 'Option::None' before being marked available
    //   - TechAlreadyResearched(T)       > This node 'self' has already been marked as researched
    //   - TechNotAvailable(T)            > This node 'self' is not marked available and cannot
    //                                      be researched at this time
    let t_name = self.get_tech_id();
    if *self.get_availability_mut() {
      if self.get_unacquired_in_edges_ref().as_ref().
        map(|rv| rv.is_empty() ).
        and_then(|empty|
          if empty {*self.get_unacquired_in_edges_mut() = None; None}
          else {*self.get_availability_mut() = false; Some(() )}
        ).is_some()
      {
        return Err(NodeErrs::IllegallyMarkedAvailable(t_name) );
      }

      if !*self.get_researched_mut() {
        *self.get_researched_mut() = true;
      } else {
        return Err(NodeErrs::TechAlreadyResearched(t_name) );
      }
    } else {
      return Err(NodeErrs::TechNotAvailable(t_name) );
    }

    Ok(() )
  }

  fn move_link_acquired(&mut self, tech: T) -> Result<(), NodeErrs<T> > {
    // Possible Errors:
    //   - LinkAlreadyAcquired(T, T) > The input 'tech: T' has already been added to the
    //                                 'acquired' list
    //   - LinkToTechNotFound(T, T)  > There is no listing for the input 'tech: T' linking in to
    //                                 this node 'self'
    let t_name = self.get_tech_id();
    self.get_unacquired_in_edges_mut().as_mut().
    ok_or(NodeErrs::LinkToTechNotFound(tech, t_name) ).
    and_then(|rmv|
      rmv.binary_search(&tech).
      or(Err(NodeErrs::LinkToTechNotFound(tech, t_name) ) ).
      map(|i| {
        let t = rmv.remove(i);
        (t, rmv.is_empty() )
      })
    ).
    map(|(t, empty)| {
      if empty {
        *self.get_unacquired_in_edges_mut() = None;
        *self.get_availability_mut() = true;
      }
      Ok(t)
    }).
    or_else(|e|
      self.get_acquired_in_edges_ref().as_ref().
      ok_or(e).
      and_then(|rv|
        rv.binary_search(&tech).
        or(Err(e) ).
        and(Err(NodeErrs::LinkAlreadyAcquired(tech, t_name)) )
      )
    )?.
    and_then(|t| {
      let rmv = self.get_acquired_in_edges_mut().get_or_insert(Vec::with_capacity(5) );
      rmv.binary_search(&t).
      and(Ok(true) ).
      or_else(|i| {
        rmv.insert(i, t);
        Ok(false)
      }).
      and_then(|already_present|
        if !already_present {
          Ok(() )
        } else {
          Err(NodeErrs::LinkAlreadyAcquired(tech, t_name) )
      })
    })
  }
}


#[cfg(test)]
mod tests;
