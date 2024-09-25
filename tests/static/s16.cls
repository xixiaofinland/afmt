public class Me {
  {
    List<Account> accounts = [SELECT Id FROM Account WHERE CreatedDate = LAST_N_DAYS:60];
  }
}
