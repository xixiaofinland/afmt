public class Me {
  {
    for (List<Account> accs : [SELECT Id FROM Account]) {
      for (Account a : accs) {
        System.debug(a);
      }
    }
  }
}
