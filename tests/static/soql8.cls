class UTIL_SortContact_TEST {
  {
    List<AutoNumber__c> autoNumbers = [SELECT Starting_Number__c FROM AutoNumber__c WHERE IsActive__c = TRUE LIMIT 1 FOR UPDATE];
  }
}
