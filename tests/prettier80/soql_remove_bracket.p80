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

    String s2 = [
      SELECT AccountId
      FROM Contact
      WHERE
        AccountId IN :accountIdsToUpdate
        OR Deceased__c != TRUE
        OR Exclude_from_Household_Name__c != TRUE
    ];

    String s2 = [
      SELECT AccountId
      FROM Contact
      WHERE
        AccountId IN :accountIdsToUpdate
        OR (Deceased__c != TRUE
        AND Exclude_from_Household_Name__c != TRUE)
    ];

    String s3 = [
      SELECT AccountId
      FROM Contact
      WHERE
        AccountId IN :accountIdsToUpdate
        AND (Deceased__c != TRUE
        OR Exclude_from_Household_Name__c != TRUE)
        AND AccountId != NULL
    ];
  }
}
