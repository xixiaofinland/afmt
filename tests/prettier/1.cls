public class A {
  {
    String s = [
      SELECT AccountId
      FROM Contact
      WHERE
        AccountId IN :accountIdsToUpdate
        AND Deceased__c != TRUE
        AND Exclude_from_Household_Name__c != TRUE
    ];
    [
      SELECT Body__c
      FROM Banner__c
      WHERE
        Language__c = :langCode
        AND Visible__c = TRUE
        AND Channel__c INCLUDES (:channel)
        AND Market__c INCLUDES (:marketCode)
      ORDER BY Order__c
    ];
  }
}
