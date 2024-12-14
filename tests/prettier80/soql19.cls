class A {
  {
    [SELECT MAX(Amount) ConvertedAmount, Id FROM Opportunity GROUP BY Id];
  }
}
