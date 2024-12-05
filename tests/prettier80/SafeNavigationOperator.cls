class SafeNavigationOperator {
  void simpleOperator() {
    String b = a?.b;
    String c = a[x]?.aMethod().aField;
    String d = a[x].aMethod()?.aField;
    d = (a[x].aMethod())?.aField;
    Integer e = anObject?.anIntegerField;
    String f = [SELECT Name FROM Account WHERE Id = :accId]?.Name;
  }
}
