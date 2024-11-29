public class A {
  {
    String s = [
            SELECT AccountId
            FROM Contact
            WHERE AccountId IN :accountIdsToUpdate and (Deceased__c != TRUE or
            Exclude_from_Household_Name__c != TRUE) and AccountId != null
        ];
  }
}
