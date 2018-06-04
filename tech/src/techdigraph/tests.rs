use ::Tech;
use super::*;


#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
enum TechTest {
  T1,
  T2,
  T3,
  T4,
  T5,
  T6,
  T7,
  T8,
  T9,
  T10,
}

impl Tech for TechTest {
  fn init_size() -> usize {
    5
  }
}


#[test]
fn techdigraph_test_1() {
// tests add_prereq()
  let mut techdigraph = PlayerGraph::<TechTest>::new();

  techdigraph.add_prereq(TechTest::T1).unwrap();
  techdigraph.add_prereq(TechTest::T2).unwrap();

  assert!(techdigraph.get_node_index(&TechTest::T1).is_ok() );
  assert!(techdigraph.get_node_index(&TechTest::T2).is_ok() );
}

#[test]
#[should_panic]
fn techdigraph_test_2() {
// fails with TechDiGraphErrs::PrereqAlreadyInsertedToGraph
  let mut techdigraph = PlayerGraph::<TechTest>::new();

  techdigraph.add_prereq(TechTest::T1).unwrap();
  techdigraph.add_prereq(TechTest::T1).unwrap();
}

#[test]
fn techdigraph_test_3() {
// tests add_advanced_link()
  let mut techdigraph = PlayerGraph::<TechTest>::new();

  techdigraph.add_prereq(TechTest::T1).unwrap();
  techdigraph.add_advanced_link(TechTest::T1, TechTest::T2).unwrap();

  assert_eq!(
    &**techdigraph.get_node_index(&TechTest::T1).
    map(|i| &techdigraph.tech_list[i]).unwrap().
    get_out_edges_ref().as_ref().
    unwrap(), &[TechTest::T2]);

  assert_eq!(
    &**techdigraph.get_node_index(&TechTest::T2).
    map(|i| &techdigraph.tech_list[i]).unwrap().
    get_unacquired_in_edges_ref().as_ref().
    unwrap(), &[TechTest::T1]);
}

#[test]
#[should_panic]
fn techdigraph_test_4() {
// fails with TechDiGraphErrs::IllegalLink
  let mut techdigraph = PlayerGraph::<TechTest>::new();

  techdigraph.add_prereq(TechTest::T1).unwrap();
  techdigraph.add_prereq(TechTest::T2).unwrap();
  techdigraph.add_advanced_link(TechTest::T1, TechTest::T2).unwrap();
}

#[test]
#[should_panic]
fn techdigraph_test_5() {
// fails with TechDiGraphErrs::TechNotFound
  let mut techdigraph = PlayerGraph::<TechTest>::new();

  techdigraph.add_prereq(TechTest::T1).unwrap();
  techdigraph.add_advanced_link(TechTest::T2, TechTest::T3).unwrap()
}

#[test]
fn techdigraph_test_6() {
// tests add_advanced_link() and checks state
  let mut techdigraph = PlayerGraph::<TechTest>::new();

  techdigraph.add_prereq(TechTest::T1).unwrap();
  techdigraph.add_advanced_link(TechTest::T1, TechTest::T2).unwrap();
  techdigraph.add_advanced_link(TechTest::T2, TechTest::T3).unwrap();

  assert_eq!(
    &**techdigraph.get_node_index(&TechTest::T1).
    map(|i| &techdigraph.tech_list[i]).unwrap().
    get_out_edges_ref().as_ref().
    unwrap(), &[TechTest::T2]);

  assert_eq!(
    &**techdigraph.get_node_index(&TechTest::T2).
    map(|i| &techdigraph.tech_list[i]).unwrap().
    get_unacquired_in_edges_ref().as_ref().
    unwrap(), &[TechTest::T1]);

  assert_eq!(
    &**techdigraph.get_node_index(&TechTest::T3).
    map(|i| &techdigraph.tech_list[i]).unwrap().
    get_unacquired_in_edges_ref().as_ref().
    unwrap(), &[TechTest::T2]);
}

#[test]
#[should_panic]
fn techdigraph_test_7() {
// fails with TechDiGraphErrs::IllegalLink
  let mut techdigraph = PlayerGraph::<TechTest>::new();

  techdigraph.add_prereq(TechTest::T1).unwrap();
  techdigraph.add_advanced_link(TechTest::T1, TechTest::T1).unwrap();
}

#[test]
#[should_panic]
fn techdigraph_test_8() {
// fails with TechDiGraphErrs::LinkToTechAlreadyInsertedToGraph
  let mut techdigraph = PlayerGraph::<TechTest>::new();

  techdigraph.add_prereq(TechTest::T1).unwrap();
  techdigraph.add_advanced_link(TechTest::T1, TechTest::T2).unwrap();
  techdigraph.add_advanced_link(TechTest::T1, TechTest::T2).unwrap();
}

#[test]
#[should_panic]
fn techdigraph_test_9() {
// fails with TechDiGraphErrs::LinkToTechAlreadyInsertedToGraph
  let mut techdigraph = PlayerGraph::<TechTest>::new();

  techdigraph.add_prereq(TechTest::T1).unwrap();
  techdigraph.add_advanced_link(TechTest::T1, TechTest::T2).unwrap();
  techdigraph.add_advanced_link(TechTest::T2, TechTest::T3).unwrap();
  techdigraph.add_advanced_link(TechTest::T2, TechTest::T3).unwrap();
}

#[test]
fn techdigraph_test_10() {
// tests add_advanced_link() and checks state
  let mut techdigraph = PlayerGraph::<TechTest>::new();

  techdigraph.add_prereq(TechTest::T1).unwrap();
  techdigraph.add_prereq(TechTest::T2).unwrap();
  techdigraph.add_prereq(TechTest::T3).unwrap();

  techdigraph.add_advanced_link(TechTest::T2, TechTest::T4).unwrap();
  techdigraph.add_advanced_link(TechTest::T1, TechTest::T4).unwrap();
  techdigraph.add_advanced_link(TechTest::T3, TechTest::T4).unwrap();

  for t in &techdigraph.tech_list[0..3] {
    assert_eq!(
      &**t.get_out_edges_ref().as_ref().unwrap(),
      &[TechTest::T4]);
  }

  assert_eq!(
    &**techdigraph.tech_list[3].get_unacquired_in_edges_ref().as_ref().unwrap(),
    &[TechTest::T1, TechTest::T2, TechTest::T3]);
}

#[test]
fn techdigraph_test_11() {
// tests mark_researched()
  let mut techdigraph = PlayerGraph::<TechTest>::new();

  assert!(techdigraph.get_node_index(&TechTest::T1).is_err() );

  techdigraph.add_prereq(TechTest::T1).unwrap();
  assert!(!techdigraph.get_node_index(&TechTest::T1).map(|i| &techdigraph.tech_list[i]).unwrap().is_researched() );

  techdigraph.mark_researched(TechTest::T1).unwrap();
  assert!(techdigraph.get_node_index(&TechTest::T1).map(|i| &techdigraph.tech_list[i]).unwrap().is_researched() );
}

#[test]
#[should_panic]
fn techdigraph_test_12() {
// fails with TechDiGraphErrs::TechNotFound
  let mut techdigraph = PlayerGraph::<TechTest>::new();

  techdigraph.mark_researched(TechTest::T1).unwrap();
}

#[test]
fn techdigraph_test_13() {
// tests mark_researched()
  let mut techdigraph = PlayerGraph::<TechTest>::new();

  techdigraph.add_prereq(TechTest::T1).unwrap();
  techdigraph.add_advanced_link(TechTest::T1, TechTest::T2).unwrap();

  assert!(!techdigraph.get_node_index(&TechTest::T1).map(|i| &techdigraph.tech_list[i]).unwrap().is_researched() );
  assert!(!techdigraph.get_node_index(&TechTest::T2).map(|i| &techdigraph.tech_list[i]).unwrap().is_researched() );

  assert_eq!(
    &**techdigraph.get_node_index(&TechTest::T1).
    map(|i| &techdigraph.tech_list[i]).unwrap().
    get_out_edges_ref().as_ref().
    unwrap(), &[TechTest::T2]);
  assert_eq!(
    &**techdigraph.get_node_index(&TechTest::T2).
    map(|i| &techdigraph.tech_list[i]).unwrap().
    get_unacquired_in_edges_ref().as_ref().
    unwrap(), &[TechTest::T1]);

  techdigraph.mark_researched(TechTest::T1).unwrap();

  assert!(techdigraph.get_node_index(&TechTest::T1).map(|i| &techdigraph.tech_list[i]).unwrap().is_researched() );
  assert!(!techdigraph.get_node_index(&TechTest::T2).map(|i| &techdigraph.tech_list[i]).unwrap().is_researched() );

  assert_eq!(
    &**techdigraph.get_node_index(&TechTest::T1).
    map(|i| &techdigraph.tech_list[i]).unwrap().
    get_out_edges_ref().as_ref().
    unwrap(), &[TechTest::T2]);
  assert_eq!(
    &**techdigraph.get_node_index(&TechTest::T2).
    map(|i| &techdigraph.tech_list[i]).unwrap().
    get_acquired_in_edges_ref().as_ref().
    unwrap(), &[TechTest::T1]);

  assert!(techdigraph.get_node_index(&TechTest::T2).
    map(|i| &techdigraph.tech_list[i]).unwrap().
    get_unacquired_in_edges_ref().
    is_none() );
}

#[test]
#[should_panic]
fn techdigraph_test_14() {
// fails with TechDiGraphErrs::TechAlreadyResearched
  let mut techdigraph = PlayerGraph::<TechTest>::new();

  techdigraph.add_prereq(TechTest::T1).unwrap();

  techdigraph.mark_researched(TechTest::T1).unwrap();
  techdigraph.mark_researched(TechTest::T1).unwrap();
}

#[test]
#[should_panic]
fn techdigraph_test_15() {
// fails with TechDiGraphErrs::TechAlreadyResearched
  let mut techdigraph = PlayerGraph::<TechTest>::new();

  techdigraph.add_prereq(TechTest::T1).unwrap();
  techdigraph.add_advanced_link(TechTest::T1, TechTest::T2).unwrap();

  techdigraph.mark_researched(TechTest::T1).unwrap();
  techdigraph.mark_researched(TechTest::T1).unwrap();
}

#[test]
fn techdigraph_test_16() {
// tests mark_researched()
  let mut techdigraph = PlayerGraph::<TechTest>::new();

  techdigraph.add_prereq(TechTest::T1).unwrap();
  techdigraph.add_advanced_link(TechTest::T1, TechTest::T2).unwrap();

  assert_eq!(
    &**techdigraph.get_node_index(&TechTest::T1).
    map(|i| &techdigraph.tech_list[i]).unwrap().
    get_out_edges_ref().as_ref().
    unwrap(), &[TechTest::T2]);
  assert_eq!(
    &**techdigraph.get_node_index(&TechTest::T2).
    map(|i| &techdigraph.tech_list[i]).unwrap().
    get_unacquired_in_edges_ref().as_ref().
    unwrap(), &[TechTest::T1]);

  techdigraph.mark_researched(TechTest::T1).unwrap();

  assert_eq!(
    &**techdigraph.get_node_index(&TechTest::T1).
    map(|i| &techdigraph.tech_list[i]).unwrap().
    get_out_edges_ref().as_ref().
    unwrap(), &[TechTest::T2]);
  assert_eq!(
    &**techdigraph.get_node_index(&TechTest::T2).
    map(|i| &techdigraph.tech_list[i]).unwrap().
    get_acquired_in_edges_ref().as_ref().
    unwrap(), &[TechTest::T1]);

  techdigraph.add_advanced_link(TechTest::T1, TechTest::T3).unwrap();

  assert_eq!(
    &**techdigraph.get_node_index(&TechTest::T1).
    map(|i| &techdigraph.tech_list[i]).unwrap().
    get_out_edges_ref().as_ref().
    unwrap(), &[TechTest::T2, TechTest::T3]);
  assert_eq!(
    &**techdigraph.get_node_index(&TechTest::T3).
    map(|i| &techdigraph.tech_list[i]).unwrap().
    get_acquired_in_edges_ref().as_ref().
    unwrap(), &[TechTest::T1]);
}

#[test]
#[should_panic]
fn techdigraph_test_17() {
// fails with TechDiGraphErrs::TechNotAvailable
  let mut techdigraph = PlayerGraph::<TechTest>::new();

  techdigraph.add_prereq(TechTest::T1).unwrap();
  techdigraph.add_advanced_link(TechTest::T1, TechTest::T2).unwrap();

  techdigraph.mark_researched(TechTest::T2).unwrap();
}

#[test]
fn techdigraph_test_18() {
// tests mark_researched()
  let mut techdigraph = PlayerGraph::<TechTest>::new();

  techdigraph.add_prereq(TechTest::T1).unwrap();
  techdigraph.add_advanced_link(TechTest::T1, TechTest::T2).unwrap();

  techdigraph.mark_researched(TechTest::T1).unwrap();
  techdigraph.mark_researched(TechTest::T2).unwrap();

  techdigraph.add_advanced_link(TechTest::T2, TechTest::T3).unwrap();

  assert_eq!(
    &**techdigraph.get_node_index(&TechTest::T3).
    map(|i| &techdigraph.tech_list[i]).unwrap().
    get_acquired_in_edges_ref().as_ref().
    unwrap(), &[TechTest::T2]);

  assert!(
    techdigraph.get_node_index(&TechTest::T3).
    map(|i| &techdigraph.tech_list[i]).unwrap().
    is_available()
  );
}

#[test]
fn techdigraph_test_19() {
// comprehensive PlayerGraph<T> and MasterGraph<T> test
  let mut playergraph = PlayerGraph::<TechTest>::new();
  let mut mastergraph = MasterGraph::<TechTest>::new();

  mastergraph.add_prereq(TechTest::T1).unwrap();
  mastergraph.add_prereq(TechTest::T2).unwrap();
  mastergraph.add_prereq(TechTest::T3).unwrap();
  mastergraph.add_prereq(TechTest::T4).unwrap();

  mastergraph.add_advanced_link(TechTest::T1, TechTest::T5).unwrap();
  mastergraph.add_advanced_link(TechTest::T1, TechTest::T7).unwrap();
  mastergraph.add_advanced_link(TechTest::T2, TechTest::T5).unwrap();
  mastergraph.add_advanced_link(TechTest::T2, TechTest::T6).unwrap();
  mastergraph.add_advanced_link(TechTest::T3, TechTest::T5).unwrap();
  mastergraph.add_advanced_link(TechTest::T4, TechTest::T6).unwrap();
  mastergraph.add_advanced_link(TechTest::T4, TechTest::T7).unwrap();
  mastergraph.add_advanced_link(TechTest::T4, TechTest::T8).unwrap();

  mastergraph.add_advanced_link(TechTest::T5, TechTest::T9);
  mastergraph.add_advanced_link(TechTest::T6, TechTest::T9);
  mastergraph.add_advanced_link(TechTest::T6, TechTest::T10);
  mastergraph.add_advanced_link(TechTest::T7, TechTest::T10);
  mastergraph.add_advanced_link(TechTest::T8, TechTest::T9);
  mastergraph.add_advanced_link(TechTest::T8, TechTest::T10);

  assert_eq!(
    &**mastergraph.get_node_index(&TechTest::T1).
    map(|i| &mastergraph.tech_list[i]).unwrap().
    get_out_edges_ref().as_ref().
    unwrap(), &[TechTest::T5, TechTest::T7]);

  assert_eq!(
    &**mastergraph.get_node_index(&TechTest::T2).
    map(|i| &mastergraph.tech_list[i]).unwrap().
    get_out_edges_ref().as_ref().
    unwrap(), &[TechTest::T5, TechTest::T6]);

  assert_eq!(
    &**mastergraph.get_node_index(&TechTest::T3).
    map(|i| &mastergraph.tech_list[i]).unwrap().
    get_out_edges_ref().as_ref().
    unwrap(), &[TechTest::T5]);

  assert_eq!(
    &**mastergraph.get_node_index(&TechTest::T4).
    map(|i| &mastergraph.tech_list[i]).unwrap().
    get_out_edges_ref().as_ref().
    unwrap(), &[TechTest::T6, TechTest::T7, TechTest::T8]);

  assert_eq!(
    &**mastergraph.get_node_index(&TechTest::T5).
    map(|i| &mastergraph.tech_list[i]).unwrap().
    get_in_edges_ref().as_ref().
    unwrap(), &[TechTest::T1, TechTest::T2, TechTest::T3]);

  assert_eq!(
    &**mastergraph.get_node_index(&TechTest::T6).
    map(|i| &mastergraph.tech_list[i]).unwrap().
    get_in_edges_ref().as_ref().
    unwrap(), &[TechTest::T2, TechTest::T4]);

  assert_eq!(
    &**mastergraph.get_node_index(&TechTest::T7).
    map(|i| &mastergraph.tech_list[i]).unwrap().
    get_in_edges_ref().as_ref().
    unwrap(), &[TechTest::T1, TechTest::T4]);

  assert_eq!(
    &**mastergraph.get_node_index(&TechTest::T8).
    map(|i| &mastergraph.tech_list[i]).unwrap().
    get_in_edges_ref().as_ref().
    unwrap(), &[TechTest::T4]);

  assert_eq!(
    &**mastergraph.get_node_index(&TechTest::T5).
    map(|i| &mastergraph.tech_list[i]).unwrap().
    get_out_edges_ref().as_ref().
    unwrap(), &[TechTest::T9]);

  assert_eq!(
    &**mastergraph.get_node_index(&TechTest::T6).
    map(|i| &mastergraph.tech_list[i]).unwrap().
    get_out_edges_ref().as_ref().
    unwrap(), &[TechTest::T9, TechTest::T10]);

  assert_eq!(
    &**mastergraph.get_node_index(&TechTest::T7).
    map(|i| &mastergraph.tech_list[i]).unwrap().
    get_out_edges_ref().as_ref().
    unwrap(), &[TechTest::T10]);

  assert_eq!(
    &**mastergraph.get_node_index(&TechTest::T8).
    map(|i| &mastergraph.tech_list[i]).unwrap().
    get_out_edges_ref().as_ref().
    unwrap(), &[TechTest::T9, TechTest::T10]);

  assert_eq!(
    &**mastergraph.get_node_index(&TechTest::T9).
    map(|i| &mastergraph.tech_list[i]).unwrap().
    get_in_edges_ref().as_ref().
    unwrap(), &[TechTest::T5, TechTest::T6, TechTest::T8]);

  assert_eq!(
    &**mastergraph.get_node_index(&TechTest::T10).
    map(|i| &mastergraph.tech_list[i]).unwrap().
    get_in_edges_ref().as_ref().
    unwrap(), &[TechTest::T6, TechTest::T7, TechTest::T8]);

  assert!(
    mastergraph.get_node_index(&TechTest::T1).
    map(|i| &mastergraph.tech_list[i]).unwrap().
    get_in_edges_ref().is_none()
  );

  assert!(
    mastergraph.get_node_index(&TechTest::T2).
    map(|i| &mastergraph.tech_list[i]).unwrap().
    get_in_edges_ref().is_none()
  );

  assert!(
    mastergraph.get_node_index(&TechTest::T3).
    map(|i| &mastergraph.tech_list[i]).unwrap().
    get_in_edges_ref().is_none()
  );

  assert!(
    mastergraph.get_node_index(&TechTest::T4).
    map(|i| &mastergraph.tech_list[i]).unwrap().
    get_in_edges_ref().is_none()
  );

  assert!(
    mastergraph.get_node_index(&TechTest::T9).
    map(|i| &mastergraph.tech_list[i]).unwrap().
    get_out_edges_ref().is_none()
  );

  assert!(
    mastergraph.get_node_index(&TechTest::T10).
    map(|i| &mastergraph.tech_list[i]).unwrap().
    get_out_edges_ref().is_none()
  );

  playergraph.add_prereq(TechTest::T4).unwrap();
  playergraph.add_advanced_link(TechTest::T4, TechTest::T8).unwrap();
  playergraph.mark_researched(TechTest::T4).unwrap();
  playergraph.add_prereq(TechTest::T1).unwrap();
  playergraph.add_advanced_link(TechTest::T1, TechTest::T7).unwrap();
  playergraph.add_advanced_link(TechTest::T4, TechTest::T7).unwrap();

  assert_eq!(
    &**playergraph.get_node_index(&TechTest::T1).
    map(|i| &playergraph.tech_list[i]).unwrap().
    get_out_edges_ref().as_ref().
    unwrap(), &[TechTest::T7]);

  assert_eq!(
    &**playergraph.get_node_index(&TechTest::T4).
    map(|i| &playergraph.tech_list[i]).unwrap().
    get_out_edges_ref().as_ref().
    unwrap(), &[TechTest::T7, TechTest::T8]);

  assert_eq!(
    &**playergraph.get_node_index(&TechTest::T7).
    map(|i| &playergraph.tech_list[i]).unwrap().
    get_unacquired_in_edges_ref().as_ref().
    unwrap(), &[TechTest::T1]);

  assert_eq!(
    &**playergraph.get_node_index(&TechTest::T7).
    map(|i| &playergraph.tech_list[i]).unwrap().
    get_acquired_in_edges_ref().as_ref().
    unwrap(), &[TechTest::T4]);

  assert_eq!(
    &**playergraph.get_node_index(&TechTest::T8).
    map(|i| &playergraph.tech_list[i]).unwrap().
    get_acquired_in_edges_ref().as_ref().
    unwrap(), &[TechTest::T4]);

  assert!(
    playergraph.get_node_index(&TechTest::T8).
    map(|i| &playergraph.tech_list[i]).unwrap().
    get_unacquired_in_edges_ref().is_none()
  );

  assert!(
    !playergraph.get_node_index(&TechTest::T7).
    map(|i| &playergraph.tech_list[i]).unwrap().
    is_available()
  );

  assert!(
    playergraph.get_node_index(&TechTest::T8).
    map(|i| &playergraph.tech_list[i]).unwrap().
    is_available()
  );

  playergraph.add_prereq(TechTest::T2).unwrap();
  playergraph.add_advanced_link(TechTest::T2, TechTest::T6).unwrap();
  playergraph.add_advanced_link(TechTest::T4, TechTest::T6).unwrap();
  playergraph.add_advanced_link(TechTest::T6, TechTest::T10).unwrap();
  playergraph.add_advanced_link(TechTest::T7, TechTest::T10).unwrap();
  playergraph.add_advanced_link(TechTest::T8, TechTest::T10).unwrap();
  playergraph.mark_researched(TechTest::T8).unwrap();
  playergraph.mark_researched(TechTest::T2).unwrap();

  assert_eq!(
    &**playergraph.get_node_index(&TechTest::T2).
    map(|i| &playergraph.tech_list[i]).unwrap().
    get_out_edges_ref().as_ref().
    unwrap(), &[TechTest::T6]);

  assert_eq!(
    &**playergraph.get_node_index(&TechTest::T4).
    map(|i| &playergraph.tech_list[i]).unwrap().
    get_out_edges_ref().as_ref().
    unwrap(), &[TechTest::T6, TechTest::T7, TechTest::T8]);

  assert_eq!(
    &**playergraph.get_node_index(&TechTest::T6).
    map(|i| &playergraph.tech_list[i]).unwrap().
    get_acquired_in_edges_ref().as_ref().
    unwrap(), &[TechTest::T2, TechTest::T4]);

  assert!(
    playergraph.get_node_index(&TechTest::T6).
    map(|i| &playergraph.tech_list[i]).unwrap().
    get_unacquired_in_edges_ref().is_none()
  );

  assert!(
    playergraph.get_node_index(&TechTest::T6).
    map(|i| &playergraph.tech_list[i]).unwrap().
    is_available()
  );

  assert_eq!(
    &**playergraph.get_node_index(&TechTest::T6).
    map(|i| &playergraph.tech_list[i]).unwrap().
    get_out_edges_ref().as_ref().
    unwrap(), &[TechTest::T10]);

  assert_eq!(
    &**playergraph.get_node_index(&TechTest::T7).
    map(|i| &playergraph.tech_list[i]).unwrap().
    get_out_edges_ref().as_ref().
    unwrap(), &[TechTest::T10]);

  assert_eq!(
    &**playergraph.get_node_index(&TechTest::T8).
    map(|i| &playergraph.tech_list[i]).unwrap().
    get_out_edges_ref().as_ref().
    unwrap(), &[TechTest::T10]);

  assert_eq!(
    &**playergraph.get_node_index(&TechTest::T10).
    map(|i| &playergraph.tech_list[i]).unwrap().
    get_acquired_in_edges_ref().as_ref().
    unwrap(), &[TechTest::T8]);

  assert_eq!(
    &**playergraph.get_node_index(&TechTest::T10).
    map(|i| &playergraph.tech_list[i]).unwrap().
    get_unacquired_in_edges_ref().as_ref().
    unwrap(), &[TechTest::T6, TechTest::T7]);

  playergraph.add_prereq(TechTest::T3).unwrap();
  playergraph.add_advanced_link(TechTest::T1, TechTest::T5).unwrap();
  playergraph.add_advanced_link(TechTest::T2, TechTest::T5).unwrap();
  playergraph.add_advanced_link(TechTest::T3, TechTest::T5).unwrap();
  playergraph.add_advanced_link(TechTest::T5, TechTest::T9).unwrap();
  playergraph.add_advanced_link(TechTest::T6, TechTest::T9).unwrap();
  playergraph.add_advanced_link(TechTest::T8, TechTest::T9).unwrap();

  assert_eq!(
    &**playergraph.get_node_index(&TechTest::T5).
    map(|i| &playergraph.tech_list[i]).unwrap().
    get_acquired_in_edges_ref().as_ref().
    unwrap(), &[TechTest::T2]);

  assert_eq!(
    &**playergraph.get_node_index(&TechTest::T9).
    map(|i| &playergraph.tech_list[i]).unwrap().
    get_acquired_in_edges_ref().as_ref().
    unwrap(), &[TechTest::T8]);

  playergraph.mark_researched(TechTest::T1).unwrap();

  // This will produce an error but have no effect on state
  playergraph.mark_researched(TechTest::T2);

  playergraph.mark_researched(TechTest::T3).unwrap();
  playergraph.mark_researched(TechTest::T5).unwrap();
  playergraph.mark_researched(TechTest::T6).unwrap();
  playergraph.mark_researched(TechTest::T7).unwrap();
  playergraph.mark_researched(TechTest::T9).unwrap();
  playergraph.mark_researched(TechTest::T10).unwrap();

  assert_eq!(
    &**mastergraph.get_node_index(&TechTest::T1).
    map(|i| &mastergraph.tech_list[i]).unwrap().
    get_out_edges_ref().as_ref().unwrap(),
    &**playergraph.get_node_index(&TechTest::T1).
    map(|i| &playergraph.tech_list[i]).unwrap().
    get_out_edges_ref().as_ref().unwrap()
  );

  assert_eq!(
    &**mastergraph.get_node_index(&TechTest::T2).
    map(|i| &mastergraph.tech_list[i]).unwrap().
    get_out_edges_ref().as_ref().unwrap(),
    &**playergraph.get_node_index(&TechTest::T2).
    map(|i| &playergraph.tech_list[i]).unwrap().
    get_out_edges_ref().as_ref().unwrap()
  );

  assert_eq!(
    &**mastergraph.get_node_index(&TechTest::T3).
    map(|i| &mastergraph.tech_list[i]).unwrap().
    get_out_edges_ref().as_ref().unwrap(),
    &**playergraph.get_node_index(&TechTest::T3).
    map(|i| &playergraph.tech_list[i]).unwrap().
    get_out_edges_ref().as_ref().unwrap()
  );

  assert_eq!(
    &**mastergraph.get_node_index(&TechTest::T4).
    map(|i| &mastergraph.tech_list[i]).unwrap().
    get_out_edges_ref().as_ref().unwrap(),
    &**playergraph.get_node_index(&TechTest::T4).
    map(|i| &playergraph.tech_list[i]).unwrap().
    get_out_edges_ref().as_ref().unwrap()
  );

  assert_eq!(
    &**mastergraph.get_node_index(&TechTest::T5).
    map(|i| &mastergraph.tech_list[i]).unwrap().
    get_in_edges_ref().as_ref().unwrap(),
    &**playergraph.get_node_index(&TechTest::T5).
    map(|i| &playergraph.tech_list[i]).unwrap().
    get_acquired_in_edges_ref().as_ref().unwrap()
  );

  assert_eq!(
    &**mastergraph.get_node_index(&TechTest::T6).
    map(|i| &mastergraph.tech_list[i]).unwrap().
    get_in_edges_ref().as_ref().unwrap(),
    &**playergraph.get_node_index(&TechTest::T6).
    map(|i| &playergraph.tech_list[i]).unwrap().
    get_acquired_in_edges_ref().as_ref().unwrap()
  );

  assert_eq!(
    &**mastergraph.get_node_index(&TechTest::T7).
    map(|i| &mastergraph.tech_list[i]).unwrap().
    get_in_edges_ref().as_ref().unwrap(),
    &**playergraph.get_node_index(&TechTest::T7).
    map(|i| &playergraph.tech_list[i]).unwrap().
    get_acquired_in_edges_ref().as_ref().unwrap()
  );

  assert_eq!(
    &**mastergraph.get_node_index(&TechTest::T8).
    map(|i| &mastergraph.tech_list[i]).unwrap().
    get_in_edges_ref().as_ref().unwrap(),
    &**playergraph.get_node_index(&TechTest::T8).
    map(|i| &playergraph.tech_list[i]).unwrap().
    get_acquired_in_edges_ref().as_ref().unwrap()
  );

  assert_eq!(
    &**mastergraph.get_node_index(&TechTest::T5).
    map(|i| &mastergraph.tech_list[i]).unwrap().
    get_out_edges_ref().as_ref().unwrap(),
    &**playergraph.get_node_index(&TechTest::T5).
    map(|i| &playergraph.tech_list[i]).unwrap().
    get_out_edges_ref().as_ref().unwrap()
  );

  assert_eq!(
    &**mastergraph.get_node_index(&TechTest::T6).
    map(|i| &mastergraph.tech_list[i]).unwrap().
    get_out_edges_ref().as_ref().unwrap(),
    &**playergraph.get_node_index(&TechTest::T6).
    map(|i| &playergraph.tech_list[i]).unwrap().
    get_out_edges_ref().as_ref().unwrap()
  );

  assert_eq!(
    &**mastergraph.get_node_index(&TechTest::T7).
    map(|i| &mastergraph.tech_list[i]).unwrap().
    get_out_edges_ref().as_ref().unwrap(),
    &**playergraph.get_node_index(&TechTest::T7).
    map(|i| &playergraph.tech_list[i]).unwrap().
    get_out_edges_ref().as_ref().unwrap()
  );

  assert_eq!(
    &**mastergraph.get_node_index(&TechTest::T8).
    map(|i| &mastergraph.tech_list[i]).unwrap().
    get_out_edges_ref().as_ref().unwrap(),
    &**playergraph.get_node_index(&TechTest::T8).
    map(|i| &playergraph.tech_list[i]).unwrap().
    get_out_edges_ref().as_ref().unwrap()
  );

  assert_eq!(
    &**mastergraph.get_node_index(&TechTest::T9).
    map(|i| &mastergraph.tech_list[i]).unwrap().
    get_in_edges_ref().as_ref().unwrap(),
    &**playergraph.get_node_index(&TechTest::T9).
    map(|i| &playergraph.tech_list[i]).unwrap().
    get_acquired_in_edges_ref().as_ref().unwrap()
  );

  assert_eq!(
    &**mastergraph.get_node_index(&TechTest::T10).
    map(|i| &mastergraph.tech_list[i]).unwrap().
    get_in_edges_ref().as_ref().unwrap(),
    &**playergraph.get_node_index(&TechTest::T10).
    map(|i| &playergraph.tech_list[i]).unwrap().
    get_acquired_in_edges_ref().as_ref().unwrap()
  );
}
