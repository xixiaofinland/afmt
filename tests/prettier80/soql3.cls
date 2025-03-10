@isTest
private class UTIL_SortContact_TEST {
  @isTest
  private static void sortContactsLastFirstName() {
    List<Schema.ApexTrigger> expectedResults = [
      SELECT Name
      FROM ApexTrigger
      WHERE Name IN :targetApexTriggerNames
      ORDER BY NamespacePrefix NULLS LAST
    ];
  }
}
