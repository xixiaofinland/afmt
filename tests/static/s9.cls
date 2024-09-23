public class Me {
  {
    String a = 'A';
    List<Account> accs = [SELECT Id FROM Account WHERE Name = :'N' + a + 'M' + 'E' AND Name = 'NOPE'];
  }
}
