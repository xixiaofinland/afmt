class SafeNavigationOperator {
  {
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
  }
}
