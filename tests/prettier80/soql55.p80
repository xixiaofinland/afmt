class UTIL_SortContact_TEST {
  {
    [
      SELECT Id
      FROM npe03__Recurring_Donation__c
      WHERE
        CALENDAR_YEAR(npe03__Date_Established__c) = :filterYear
        AND CALENDAR_MONTH(npe03__Date_Established__c) >= :startMonth
        AND CALENDAR_MONTH(npe03__Date_Established__c) <= :endMonth
    ];
  }
}
