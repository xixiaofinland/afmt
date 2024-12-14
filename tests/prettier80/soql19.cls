class A {
  {
    [SELECT MAX(Amount) ConvertedAmount, Id FROM Opportunity GROUP BY Id];

    [SELECT MAX(Amount) CAmount, Max(A) CA, Id FROM Opportunity GROUP BY Id];
  }
}
