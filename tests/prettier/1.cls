class UTIL_SortContact_TEST {
  {
                            List<AutoNumber__c> autoNumbers = [
                SELECT
                        Display_Format__c,
                        Field_API_Name__c,
                        Object_API_Name__c,
                        Max_Used_Number__c,
                        Starting_Number__c
                FROM AutoNumber__c
                WHERE Object_API_Name__c = :String.valueOf(sObjType)
                AND IsActive__c = true
                LIMIT 1
                FOR UPDATE
        ];

}
}
