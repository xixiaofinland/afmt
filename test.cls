class SOSLClass {

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
}
