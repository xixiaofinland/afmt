class SafeNavigationOperator {
  public List<SObject> selectSObjectsById(Set<Id> idSet) {
        return [
      SELECT COUNT()
      FROM LookupRollupSummaryScheduleItems__c
      WHERE LookupRollupSummary2__c = :lookupID AND LastModifiedDate < YESTERDAY
    ];

  }
}
