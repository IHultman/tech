use ::Tech;
use ::techdigraph::node::*;
use super::PlayerNode as PNode;


#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
enum TechTest {
  T1,
  T2,
  T3,
  T4,
  T5,
}

impl Tech for TechTest {
  fn init_size() -> usize {
    5
  }
}


#[test]
fn playernode_test_1() {
// tests add_in_edge() and add_out_edge()
  let mut t1 = PNode::<TechTest>::new(TechTest::T1, true);
  let mut t2 = PNode::<TechTest>::new(TechTest::T2, false);

  assert!(t1.available);
  assert!(t2.available);

  t1.add_out_edge(TechTest::T2).unwrap();
  t2.add_in_edge((TechTest::T1, false) ).unwrap();

  assert!(t1.available);
  assert!(!t2.available);
}

#[test]
#[should_panic]
fn playernode_test_2() {
// fails with NodeErrs::AttemptToLinkToPrereq
  let mut t1 = PNode::<TechTest>::new(TechTest::T1, true);

  t1.add_in_edge((TechTest::T2, false) ).unwrap();
}

#[test]
#[should_panic]
fn playernode_test_3() {
// fails with NodeErrs::AttemptToLinkToSelf
  let mut t1 = PNode::<TechTest>::new(TechTest::T1, true);

  t1.add_out_edge(TechTest::T1).unwrap();
}

#[test]
#[should_panic]
fn playernode_test_4() {
// fails with NodeErrs::AttemptToLinkToSelf
  let mut t1 = PNode::<TechTest>::new(TechTest::T1, false);

  t1.add_in_edge((TechTest::T1, false) ).unwrap();
}

#[test]
fn playernode_test_5() {
// tests get_unacquired_in_edges() and get_acquired_in_edges()
  let mut t1 = PNode::<TechTest>::new(TechTest::T1, false);

  t1.add_in_edge((TechTest::T2, false) ).unwrap();
  t1.add_in_edge((TechTest::T3, false) ).unwrap();
  t1.add_in_edge((TechTest::T5, false) ).unwrap();

  assert_eq!(&**t1.get_unacquired_in_edges_ref().as_ref().unwrap(), &[TechTest::T2, TechTest::T3, TechTest::T5]);
  assert_eq!(t1.get_acquired_in_edges_ref().as_ref(), None);
}

#[test]
fn playernode_test_6() {
// tests get_unacquired_in_edges(), get_acquired_in_edges(), and move_link_acquired()
  let mut t1 = PNode::<TechTest>::new(TechTest::T1, false);

  t1.add_in_edge((TechTest::T2, false) ).unwrap();
  t1.add_in_edge((TechTest::T3, false) ).unwrap();
  t1.add_in_edge((TechTest::T5, false) ).unwrap();

  t1.move_link_acquired(TechTest::T2);
  t1.move_link_acquired(TechTest::T5);

  assert_eq!(&**t1.get_unacquired_in_edges_ref().as_ref().unwrap(), &[TechTest::T3]);
  assert_eq!(&**t1.get_acquired_in_edges_ref().as_ref().unwrap(), &[TechTest::T2, TechTest::T5]);
}

#[test]
fn playernode_test_7() {
// tests get_unacquired_in_edges(), get_acquired_in_edges(), and move_link_acquired()
  let mut t1 = PNode::<TechTest>::new(TechTest::T1, false);

  t1.add_in_edge((TechTest::T3, false) ).unwrap();
  t1.add_in_edge((TechTest::T5, false) ).unwrap();

  t1.move_link_acquired(TechTest::T3);
  t1.move_link_acquired(TechTest::T5);

  assert_eq!(t1.get_unacquired_in_edges_ref().as_ref(), None);
  assert_eq!(&**t1.get_acquired_in_edges_ref().as_ref().unwrap(), &[TechTest::T3, TechTest::T5]);
}

#[test]
#[should_panic]
fn playernode_test_8() {
// fails with NodeErrs::LinkToTechNotFound
  let mut t1 = PNode::<TechTest>::new(TechTest::T1, false);

  t1.add_in_edge((TechTest::T2, false) ).unwrap();

  t1.move_link_acquired(TechTest::T3).unwrap();
}

#[test]
fn playernode_test_9() {
// tests move_link_acquired()
  let mut t1 = PNode::<TechTest>::new(TechTest::T1, false);

  t1.add_in_edge((TechTest::T2, false) ).unwrap();

  t1.move_link_acquired(TechTest::T2).unwrap();

  assert_eq!(t1.get_unacquired_in_edges_ref().as_ref(), None);
}

#[test]
#[should_panic]
fn playernode_test_10() {
// fails with NodeErrs::LinkTechNotFound
  let mut t1 = PNode::<TechTest>::new(TechTest::T1, false);

  t1.add_in_edge((TechTest::T2, false) ).unwrap();

  t1.move_link_acquired(TechTest::T2).unwrap();
  t1.move_link_acquired(TechTest::T3).unwrap();
}

#[test]
#[should_panic]
fn playernode_test_11() {
// fails with NodeErrs::LinkAlreadyAcquired
  let mut t1 = PNode::<TechTest>::new(TechTest::T1, false);

  t1.add_in_edge((TechTest::T2, false) ).unwrap();

  t1.move_link_acquired(TechTest::T2).unwrap();
  t1.move_link_acquired(TechTest::T2).unwrap();
}

#[test]
#[should_panic]
fn playernode_test_12() {
// fails with NodeErrs::LinkAlreadyAcquired
  let mut t1 = PNode::<TechTest>::new(TechTest::T1, false);

  t1.add_in_edge((TechTest::T2, false) ).unwrap();
  t1.add_in_edge((TechTest::T3, false) ).unwrap();

  t1.move_link_acquired(TechTest::T2).unwrap();
  t1.move_link_acquired(TechTest::T2).unwrap();
}

#[test]
#[should_panic]
fn playernode_test_13() {
// fails with NodeErrs::InEdgesTechAlreadyExists
  let mut t1 = PNode::<TechTest>::new(TechTest::T1, false);

  t1.add_in_edge((TechTest::T2, false) ).unwrap();
  t1.add_in_edge((TechTest::T2, false) ).unwrap();
}

#[test]
#[should_panic]
fn playernode_test_14() {
// fails with NodeErrs::InEdgesTechAlreadyExists
  let mut t1 = PNode::<TechTest>::new(TechTest::T1, false);

  {
    let rmv = t1.acquired_in_edges.get_or_insert(Vec::new() );
    rmv.push(TechTest::T2);
  }

  assert_eq!(t1.get_unacquired_in_edges_ref().as_ref(), None);
  assert_eq!(&**t1.get_acquired_in_edges_ref().as_ref().unwrap(), &[TechTest::T2]);

  t1.add_in_edge((TechTest::T2, false) ).unwrap();
}

#[test]
#[should_panic]
fn playernode_test_15() {
// fails with NodeErrs::OutEdgesTechAlreadyExists
  let mut t1 = PNode::<TechTest>::new(TechTest::T1, true);

  t1.add_out_edge(TechTest::T2).unwrap();
  t1.add_out_edge(TechTest::T2).unwrap();
}

#[test]
fn playernode_test_16() {
// tests move_link_acquired()
  let mut t1 = PNode::<TechTest>::new(TechTest::T1, false);

  {
    let rmv = t1.acquired_in_edges.get_or_insert(Vec::new() );
    rmv.push(TechTest::T2);

    let rmv = t1.unacquired_in_edges.get_or_insert(Vec::new() );
    rmv.push(TechTest::T2);
  }

  assert_eq!(&**t1.get_unacquired_in_edges_ref().as_ref().unwrap(), &[TechTest::T2]);
  assert_eq!(&**t1.get_acquired_in_edges_ref().as_ref().unwrap(), &[TechTest::T2]);

  // produces Err(NodeErrs::LinkAlreadyAcquired) but fixes problem
  t1.move_link_acquired(TechTest::T2);

  assert_eq!(t1.get_unacquired_in_edges_ref().as_ref(), None);
  assert_eq!(&**t1.get_acquired_in_edges_ref().as_ref().unwrap(), &[TechTest::T2]);
}

#[test]
fn playernode_test_17() {
// tests mark_researched()
  let mut t1 = PNode::<TechTest>::new(TechTest::T1, true);

  assert!(t1.available);
  assert!(!t1.researched);

  t1.mark_researched().unwrap();

  assert!(t1.researched);
}

#[test]
#[should_panic]
fn playernode_test_18() {
// fails with NodeErrs::TechAlreadyResearched
  let mut t1 = PNode::<TechTest>::new(TechTest::T1, true);

  t1.mark_researched().unwrap();
  t1.mark_researched().unwrap();
}

#[test]
#[should_panic]
fn playernode_test_19() {
// fails with NodeErrs::TechNotAvailable
  let mut t1 = PNode::<TechTest>::new(TechTest::T1, false);

  t1.add_in_edge((TechTest::T2, false) ).unwrap();
  t1.mark_researched().unwrap();
}

#[test]
fn playernode_test_20() {
// tests mark_researched()
  let mut t1 = PNode::<TechTest>::new(TechTest::T1, false);

  t1.add_in_edge((TechTest::T2, false) ).unwrap();
  t1.move_link_acquired(TechTest::T2).unwrap();
  t1.mark_researched().unwrap();
}

#[test]
fn playernode_test_21() {
// tests mark_researched()
  let mut t1 = PNode::<TechTest>::new(TechTest::T1, false);
  {
    let rmv = t1.unacquired_in_edges.get_or_insert(Vec::new() );
  }

  assert_eq!(&**t1.get_unacquired_in_edges_ref().as_ref().unwrap(), &[]);

  t1.mark_researched().unwrap();

  assert_eq!(t1.get_unacquired_in_edges_ref().as_ref(), None);
}

#[test]
fn playernode_test_22() {
// tests mark_researched()
  let mut t1 = PNode::<TechTest>::new(TechTest::T1, false);
  {
    let rmv = t1.unacquired_in_edges.get_or_insert(Vec::new() );
    rmv.push(TechTest::T2);
  }

  assert_eq!(&**t1.get_unacquired_in_edges_ref().as_ref().unwrap(), &[TechTest::T2]);
  assert!(t1.available);

  t1.mark_researched();

  assert_eq!(&**t1.get_unacquired_in_edges_ref().as_ref().unwrap(), &[TechTest::T2]);
  assert!(!t1.available);
}

#[test]
fn playernode_test_23() {
// tests mark_researched(); fixes error with empty 'unacquired' list
  let mut t1 = PNode::<TechTest>::new(TechTest::T1, false);
  {
    let rmv = t1.unacquired_in_edges.get_or_insert(Vec::new() );
  }

  t1.mark_researched().unwrap();
}

#[test]
#[should_panic]
fn playernode_test_24() {
// fails with NodeErrs::IllegallyMarkedAvailable
  let mut t1 = PNode::<TechTest>::new(TechTest::T1, false);
  {
    let rmv = t1.unacquired_in_edges.get_or_insert(Vec::new() );
    rmv.push(TechTest::T2);
  }

  assert_eq!(&**t1.get_unacquired_in_edges_ref().as_ref().unwrap(), &[TechTest::T2]);
  assert!(t1.available);

  t1.mark_researched().unwrap();
}

#[test]
fn playernode_test_25() {
// compares add_in_edge_acquired() vs add_in_edge_unacquired()
  let mut t1 = PNode::<TechTest>::new(TechTest::T1, false);
  let mut t3 = PNode::<TechTest>::new(TechTest::T3, false);

  assert!(t1.available);
  t1.add_in_edge_acquired(TechTest::T2).unwrap();
  assert!(t1.available);

  assert!(t3.available);
  t3.add_in_edge_unacquired(TechTest::T4).unwrap();
  assert!(!t3.available);
}

#[test]
fn playernode_test_26() {
// tests add_in_edge_acquired()
  let mut t1 = PNode::<TechTest>::new(TechTest::T1, false);

  assert_eq!(t1.get_unacquired_in_edges_ref().as_ref(), None);
  assert_eq!(t1.get_acquired_in_edges_ref().as_ref(), None);

  t1.add_in_edge_acquired(TechTest::T2).unwrap();

  assert_eq!(t1.get_unacquired_in_edges_ref().as_ref(), None);
  assert_eq!(&**t1.get_acquired_in_edges_ref().as_ref().unwrap(), &[TechTest::T2]);
}

#[test]
#[should_panic]
fn playernode_test_27() {
// fails with NodeErrs::InEdgesTechAlreadyExists
  let mut t1 = PNode::<TechTest>::new(TechTest::T1, false);

  t1.add_in_edge_unacquired(TechTest::T2).unwrap();
  t1.add_in_edge_acquired(TechTest::T2).unwrap();
}

#[test]
fn playernode_test_28() {
// tests add_in_edge_acquired(); demonstrates how it moves tech from 'unacquired' to 'acquired' list
  let mut t1 = PNode::<TechTest>::new(TechTest::T1, false);

  assert_eq!(t1.get_unacquired_in_edges_ref().as_ref(), None);
  assert_eq!(t1.get_acquired_in_edges_ref().as_ref(), None);
  assert!(t1.available);

  t1.add_in_edge_unacquired(TechTest::T2).unwrap();

  assert_eq!(&**t1.get_unacquired_in_edges_ref().as_ref().unwrap(), &[TechTest::T2]);
  assert_eq!(t1.get_acquired_in_edges_ref().as_ref(), None);
  assert!(!t1.available);

  t1.add_in_edge_acquired(TechTest::T2);

  assert_eq!(t1.get_unacquired_in_edges_ref().as_ref(), None);
  assert_eq!(&**t1.get_acquired_in_edges_ref().as_ref().unwrap(), &[TechTest::T2]);
  assert!(t1.available);
}

#[test]
fn playernode_test_29() {
// tests add_in_edge() in same manner as previous test
  let mut t1 = PNode::<TechTest>::new(TechTest::T1, false);

  assert_eq!(t1.get_unacquired_in_edges_ref().as_ref(), None);
  assert_eq!(t1.get_acquired_in_edges_ref().as_ref(), None);
  assert!(t1.available);

  t1.add_in_edge((TechTest::T2, false) ).unwrap();

  assert_eq!(&**t1.get_unacquired_in_edges_ref().as_ref().unwrap(), &[TechTest::T2]);
  assert_eq!(t1.get_acquired_in_edges_ref().as_ref(), None);
  assert!(!t1.available);

  t1.add_in_edge((TechTest::T2, true) );

  assert_eq!(t1.get_unacquired_in_edges_ref().as_ref(), None);
  assert_eq!(&**t1.get_acquired_in_edges_ref().as_ref().unwrap(), &[TechTest::T2]);
  assert!(t1.available);
}
