class A {
  {
    [SELECT MAX(Amount) ConvertedAmount, Id FROM Opportunity GROUP BY Id];

    [SELECT MAX(Amount) CAmount, Max(A) CA, Id FROM Opportunity GROUP BY Id];

    [SELECT ID, (SELECT Id, Name, Address FROM RollupOrderBys__r) FROM Account];

    [SELECT c.Id, c.Name FROM Contact c];

    [SELECT FIELDS(STANDARD) FROM Organization LIMIT 1];
  }
}
