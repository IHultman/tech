use ::Tech;
use self::node::*;


mod node;
mod masternode;
mod playernode;


pub type MasterGraph<T> = TechDiGraph<masternode::MasterNode<T> >;
pub type PlayerGraph<T> = TechDiGraph<playernode::PlayerNode<T> >;


#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TechDiGraphErrs<T>
  where T: Tech
{
  IllegalLink(T, T),
  LinkToTechAlreadyAcquired(T, T),
  LinkToTechAlreadyInsertedToGraph(T, T),
  NoLinkToDestroy(T, T),
  PrereqAlreadyInsertedToGraph(T),
  TechAlreadyResearched(T),
  TechNotAvailable(T),
  TechNotFound(T),
}

impl<T> From<NodeErrs<T> > for TechDiGraphErrs<T>
  where T: Tech
{
  fn from(e: NodeErrs<T>) -> Self {
    match e {
      NodeErrs::AttemptToLinkToPrereq(t1, t2)     => TechDiGraphErrs::IllegalLink(t1, t2),
      NodeErrs::AttemptToLinkToSelf(t)            => TechDiGraphErrs::IllegalLink(t, t),
      NodeErrs::IllegallyMarkedAvailable(t)       => TechDiGraphErrs::TechNotAvailable(t),
      NodeErrs::InEdgesTechAlreadyExists(t1, t2)  => TechDiGraphErrs::LinkToTechAlreadyInsertedToGraph(t1, t2),
      NodeErrs::LinkAlreadyAcquired(t1, t2)       => TechDiGraphErrs::LinkToTechAlreadyAcquired(t1, t2),
      NodeErrs::LinkToTechNotFound(t1, t2)        => panic!(),
      NodeErrs::OutEdgesTechAlreadyExists(t1, t2) => TechDiGraphErrs::LinkToTechAlreadyInsertedToGraph(t1, t2),
      NodeErrs::NoLinkToRemove(t1, t2)            => TechDiGraphErrs::NoLinkToDestroy(t1, t2),
      NodeErrs::TechAlreadyResearched(t)          => TechDiGraphErrs::TechAlreadyResearched(t),
      NodeErrs::TechNotAvailable(t)               => TechDiGraphErrs::TechNotAvailable(t),
    }
  }
}

pub struct TechDiGraph<N>
  where N: Node
{
  tech_list: Vec<N>,
}

impl<N> TechDiGraph<N>
  where N: Node
{
  pub fn new() -> TechDiGraph<N> {
    //---------vv-- this is known as a path disambiguator
    TechDiGraph::<N> {
      tech_list: Vec::with_capacity(N::Tech::init_size() ),
    }
    // path disambiguator is used when you want to let the compiler know which version of an
    // instance of something (struct, fn, enum, etc) you want
  }

  pub fn add_prereq(&mut self, tech: N::Tech) -> Result<(), TechDiGraphErrs<N::Tech> > {
    // Possible Errors:
    //   - PrereqAlreadyInsertedToGraph(T) > A node with value 'tech: N::Tech' has already been
    //                                       inserted into the graph
    self.get_node_index(&tech).
    and(Ok(true) ).
    or_else(|i| {self.tech_list.insert(i, N::new(tech, true) ); Ok(false)}).
    and_then(|already_present|
      if already_present {Err(TechDiGraphErrs::PrereqAlreadyInsertedToGraph(tech) )} else {Ok(() )}
    )
  }

  pub fn add_advanced_link(&mut self, t1: N::Tech, t2: N::Tech) -> Result<(), TechDiGraphErrs<N::Tech> > {
    // Possible Errors:
    //   - IllegalLink(T, T)                      > Attempting to link from a node with a tech_id
    //                                              't1: N::Tech' to an illegal target node
    //   - LinkToTechAlreadyInsertedToGraph(T, T) > A link from a node with tech_id 't1: N::Tech' to
    //                                              another node with tech_id 't2: N::Tech' already exists
    //   - TechNotFound(T)                        > A node with tech_id 't1: N::Tech' cannot be found
    let mut it1 = self.get_node_index(&t1).or(Err(TechDiGraphErrs::TechNotFound(t1)) )?;
    self.get_node_index(&t2).
    or_else(|it2| {
      self.tech_list.insert(it2, N::new(t2, false) );
      if it2 <= it1 {it1 += 1;}
      Ok(it2)
    }).
    and_then(|it2| {
      let args = self.tech_list[it1].get_in_edge_args();
      self.tech_list[it2].add_in_edge(args).
      map_err(|e| TechDiGraphErrs::from(e) ).
      or_else(|e| if let TechDiGraphErrs::LinkToTechAlreadyInsertedToGraph(..) = e {Ok(() )} else {Err(e)} )
    }).
    and_then(|_| self.tech_list[it1].add_out_edge(t2).map_err(|e| TechDiGraphErrs::from(e)) )
  }

  pub fn check_tech(&self, tech: N::Tech) -> Result<(), TechDiGraphErrs<N::Tech> > {
    // Possible Errors:
    //   - TechNotFound(T) > A node with tech_id 'tech: N::Tech' cannot be found
    self.get_node_index(&tech).and(Ok(()) ).or(Err(TechDiGraphErrs::TechNotFound(tech)) )
  }

  fn get_node_index(&self, tech: &N::Tech) -> Result<usize, usize> {
    self.tech_list.binary_search_by_key(tech, |t| t.get_tech_id() )
  }
}

impl<N> TechDiGraph<N>
  where N: MasterNode
{
  pub fn destroy_link(&mut self, t1: N::Tech, t2: N::Tech) -> Result<(), TechDiGraphErrs<N::Tech> > {
    // Possible Errors
    //   - NoLinkToDestroy(N::Tech, N::Tech) >
    //   - TechNotFound(N::Tech)             >
    self.get_node_index(&t1).
    or(Err(TechDiGraphErrs::TechNotFound(t1)) ).
    and_then(|i| {
      self.get_node_index(&t2).
      or(Err(TechDiGraphErrs::TechNotFound(t2)) ).
      and_then(|j| {
        let rc1 = self.tech_list[i].remove_out_link(t2);
        let rc2 = self.tech_list[j].remove_in_link(t1);
        if rc1 != rc2 {panic!()} else {rc1.map_err(|e| TechDiGraphErrs::from(e) )}
      })
    })
  }

  pub fn get_in_edges(&self, tech: N::Tech) -> Option<&[N::Tech]> {
    self.get_node_index(&tech).ok().
    and_then(|i| self.tech_list[i].get_in_edges_ref().as_ref().map(|rv| &**rv) )
  }

  pub fn get_out_edges(&self, tech: N::Tech) -> Option<&[N::Tech]> {
    self.get_node_index(&tech).ok().
    and_then(|i| self.tech_list[i].get_out_edges_ref().as_ref().map(|rv| &**rv) )
  }
}

impl<N> TechDiGraph<N>
  where N: PlayerNode
{
  pub fn mark_researched(&mut self, tech: N::Tech) -> Result<(), TechDiGraphErrs<N::Tech> > {
    // Possible Errors:
    //   - TechAlreadyResearched(N::Tech)                  > The node with tech_id 'tech: N::Tech'
    //                                                       has already been researched and possible
    //                                                       updates to nodes to which this node links
    //                                                       were performed
    //   - TechNotAvailable(N::Tech)                       > This node for 'tech N::Tech' cannot be
    //                                                       researched as it's unavailable, that is
    //                                                       there are unfulfilled prereqs
    //   - TechNotFound(N::Tech)                           > There is no node in this graph corresponding
    //                                                       to 'tech: N::Tech'

    // Note, contains example of path disambiguator for enum variant!
    self.get_node_index(&tech).
    or(Err(TechDiGraphErrs::TechNotFound::<N::Tech>(tech)) ).
    and_then(|i| {
      let result = self.tech_list[i].mark_researched().
                   map(|_| Ok(()) ).
                   or_else(|e| {
                     let e = TechDiGraphErrs::from(e);
                     if let TechDiGraphErrs::TechAlreadyResearched(_) = e {Ok(Err(e) )}
                     else {Err(e)}
                   })?;
      self.tech_list[i].get_out_edges_ref().as_ref().map(|rv| rv.clone() ).
      map(|v| {
        for t in v {
          self.get_node_index(&t).map(|j| {
            self.tech_list[j].move_link_acquired(tech).map_err(|e| TechDiGraphErrs::from(e) );
          });
        }
      });
      result
    })
  }
}


#[cfg(test)]
mod tests;
