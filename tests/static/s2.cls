public class M {
  {
    List<Account> accs = [SELECT Id FROM Account WHERE Name = :('x' + 'xx')];
  }
}
