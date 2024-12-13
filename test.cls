class Chaining {
  {
    a[0].super.c();
    super.a.b();

    super.a?.b();
    super?.a?.b();

    this?.a.b();
    a.this?.b();

    c = veryLongVariableThatWillHelpBreakTheExpression[insideFieldThatIsLong]
      ?.aVeryLongMethod()
      .anotherVeryLongField;

    d = veryLongVariableThatWillHelpBreakTheExpression[insideFieldThatIsLong]
      .aVeryLongMethod()
      ?.anotherVeryLongField;

    String g = [
      SELECT Id, Name, One_Field__c, Another_Field__c
      FROM Account
      WHERE Id = :accId
    ]
    ?.Name;

    SObjectType.Attribute__c.fields.Name.isAccessible();
  }
}
