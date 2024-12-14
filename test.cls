class A {
  {
    [SELECT MAX(Amount) as CAmount, Max(A) CA, Id FROM Opportunity GROUP BY Id];
  }
}
