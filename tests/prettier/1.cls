class SafeNavigationOperator {
  public List<SObject> selectSObjectsById(Set<Id> idSet) {
        List<Profile> profiles = [
      SELECT id, Name
      FROM Profile
      WHERE Name = 'Read Only' OR Name = 'ReadOnly' OR Name = 'Standard User'
      ORDER BY Name ASC
    ];

  }
}
