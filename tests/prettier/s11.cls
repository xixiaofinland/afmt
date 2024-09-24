public class Me {
  {
    for (Account a : [SELECT Id FROM Account]) {
      System.debug(a);
    }
  }
}
