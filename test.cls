class SOSLClass {
  void inClause() {
    List<List<SObject>> searchResult;
    searchResult = [FIND 'Hello' IN ALL FIELDS];
    searchResult = [FIND 'Hello' IN ALL FIELDS];
    searchResult = [FIND 'Hello' IN EMAIL FIELDS];
    searchResult = [FIND 'Hello' IN NAME FIELDS];
    searchResult = [FIND 'Hello' IN PHONE FIELDS];
    searchResult = [FIND 'Hello' IN SIDEBAR FIELDS];
  }
  void withDivisionClause() {
    List<List<SObject>> searchResult;
    String a = 'Local';
    searchResult = [FIND 'Hello' WITH DIVISION = 'Global'];
    searchResult = [FIND 'Hello' WITH DIVISION = :a];
    searchResult = [
      FIND 'foobar'
      RETURNING Account
      WITH DIVISION = 'global'
    ];
  }
  void withDataCategory() {
    List<List<SObject>> searchResult;
    String a = 'Local';
    searchResult = [FIND 'Hello' WITH DATA CATEGORY Location__c AT America__c];
    searchResult = [
      FIND 'Hello'
      WITH DATA CATEGORY
        Geography__c ABOVE France__c
        AND Product__c AT mobile_phones__c
    ];
    searchResult = [
      FIND 'Hello'
      WITH DATA CATEGORY Geography__c BELOW Europe__c
    ];
  }
  void withSnippet() {
    List<List<SObject>> searchResult;
    String a = 'Local';
    searchResult = [FIND 'Hello' WITH SNIPPET];
    searchResult = [FIND 'Hello' WITH SNIPPET(target_length = 120)];
    searchResult = [FIND 'Hello' WITH SNIPPET(TARGET_LENGTH = 120)];
  }
  void withNetwork() {
    List<List<SObject>> searchResult;
    searchResult = [FIND 'Hello' WITH Network = 'NetworkId'];
    searchResult = [
      FIND 'Hello'
      WITH Network IN ('NetworkId1', 'NetworkId2', 'NetworkId3')
    ];
    searchResult = [FIND 'Hello' WITH Network = '00000000000000'];
    searchResult = [
      FIND 'Hello'
      WITH Network IN (
        'NetworkId1',
        'NetworkId2',
        'NetworkId3',
        'NetworkId4',
        'NetworkId5',
        'NetworkId6'
      )
    ];
  }
}
