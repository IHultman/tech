use ::Tech;
use ::techdigraph::node::*;
use super::MasterNode as MNode;


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
  let mut t1 = MNode::<TechTest>::new(TechTest::T1, true);
  let mut t2 = MNode::<TechTest>::new(TechTest::T2, false);

  t1.add_out_edge(TechTest::T2).unwrap();
  t2.add_in_edge(TechTest::T1).unwrap();
}

#[test]
#[should_panic]
fn playernode_test_2() {
// fails with NodeErrs::AttemptToLinkToPrereq
  let mut t1 = MNode::<TechTest>::new(TechTest::T1, true);

  t1.add_in_edge(TechTest::T2).unwrap();
}

#[test]
#[should_panic]
fn playernode_test_3() {
// fails with NodeErrs::AttemptToLinkToSelf
  let mut t1 = MNode::<TechTest>::new(TechTest::T1, true);

  t1.add_out_edge(TechTest::T1).unwrap();
}

#[test]
#[should_panic]
fn playernode_test_4() {
// fails with NodeErrs::AttemptToLinkToSelf
  let mut t1 = MNode::<TechTest>::new(TechTest::T1, false);

  t1.add_in_edge(TechTest::T1).unwrap();
}

#[test]
fn playernode_test_5() {
// tests get_unacquired_in_edges() and get_acquired_in_edges()
  let mut t1 = MNode::<TechTest>::new(TechTest::T1, false);

  t1.add_in_edge(TechTest::T2).unwrap();
  t1.add_in_edge(TechTest::T3).unwrap();
  t1.add_in_edge(TechTest::T5).unwrap();

  assert_eq!(&**t1.get_in_edges_ref().as_ref().unwrap(), &[TechTest::T2, TechTest::T3, TechTest::T5]);
}

#[test]
#[should_panic]
fn playernode_test_6() {
// fails with NodeErrs::InEdgesTechAlreadyExists
  let mut t1 = MNode::<TechTest>::new(TechTest::T1, false);

  t1.add_in_edge(TechTest::T2).unwrap();
  t1.add_in_edge(TechTest::T2).unwrap();
}

#[test]
#[should_panic]
fn playernode_test_7() {
// fails with NodeErrs::OutEdgesTechAlreadyExists
  let mut t1 = MNode::<TechTest>::new(TechTest::T1, true);

  t1.add_out_edge(TechTest::T2).unwrap();
  t1.add_out_edge(TechTest::T2).unwrap();
}
