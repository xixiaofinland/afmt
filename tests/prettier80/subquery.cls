class SafeNavigationOperator {
  public List<SObject> selectSObjectsById(Set<Id> idSet) {
    PermissionSet ps = [
      SELECT Id
      FROM PermissionSet
      WHERE Id IN (SELECT Id FROM P)
      LIMIT 1
    ];
  }
}
