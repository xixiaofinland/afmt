class SafeNavigationOperator {
  public List<SObject> selectSObjectsById(Set<Id> idSet) {
    return (List<SObject>) mocks.mockNonVoidMethod(
      new List<Type>{ Set<Id>.class },
      new List<Object>{ idSet }
    );
  }
}
