class UTIL_SortContact_TEST {
  {
    [SELECT OpportunityId FROM OpportunityContactRole GROUP BY OpportunityId HAVING COUNT(Id) > 1];
  }
}
