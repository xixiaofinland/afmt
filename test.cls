public class Me {
  {
    List<Account> accounts = [
      SELECT Id
      FROM Account
      WHERE CreatedDate = LAST_N_DAYS:60
    ];
    [
      SELECT Id, Name, Good_c__c
      FROM Account
      WHERE CreatedDate = LAST_N_DAYS:60
    ];
  }
}
