use ::Tech;


#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NodeErrs<T>
  where T: Tech
{
  AttemptToLinkToPrereq(T, T),
  AttemptToLinkToSelf(T),
  IllegallyMarkedAvailable(T),
  InEdgesTechAlreadyExists(T, T),
  LinkAlreadyAcquired(T, T),
  LinkToTechNotFound(T, T),
  NoLinkToRemove(T, T),
  OutEdgesTechAlreadyExists(T, T),
  TechAlreadyResearched(T),
  TechNotAvailable(T),
}


pub trait Node {
  type Tech: Tech;
  type InEdgeArgs;

  fn new(tech: Self::Tech, prereq: bool) -> Self;
  fn add_in_edge(&mut self, args: Self::InEdgeArgs) -> Result<(), NodeErrs<Self::Tech> >;
  fn add_out_edge(&mut self, tech: Self::Tech) -> Result<(), NodeErrs<Self::Tech> >;
  fn get_in_edge_args(&self) -> Self::InEdgeArgs;
  fn get_out_edges_ref(&self) -> &Option<Vec<Self::Tech> >;
  fn get_tech_id(&self) -> Self::Tech;
  fn is_prereq(&self) -> bool;
}


pub trait MasterNode: Node {
  fn get_in_edges_ref(&self) -> &Option<Vec<Self::Tech> >;
  fn remove_in_link(&mut self, t: Self::Tech) -> Result<(), NodeErrs<Self::Tech> >;
  fn remove_out_link(&mut self, t: Self::Tech) -> Result<(), NodeErrs<Self::Tech> >;
}


pub trait PlayerNode: Node {
/*
  fn add_in_edge_acquired(&mut self, tech: Self::Tech) -> Result<(), NodeErrs<Self::Tech> >;
  fn add_in_edge_unacquired(&mut self, tech: Self::Tech) -> Result<(), NodeErrs<Self::Tech> >;
*/
  fn is_researched(&self) -> bool;
  fn mark_researched(&mut self) -> Result<(), NodeErrs<Self::Tech> >;
  fn move_link_acquired(&mut self, tech: Self::Tech) -> Result<(), NodeErrs<Self::Tech> >;
}
