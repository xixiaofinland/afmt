public class Me {
  {
    Integer i = 1;
    List<Account> accs = [SELECT Id FROM Account OFFSET :i];
  }
}
