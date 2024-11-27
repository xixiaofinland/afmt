class T {
  {
    [SELECT Id FROM Account GROUP BY Id];
    [SELECT Id FROM Account GROUP BY Id, Name];
    [SELECT Id FROM Account GROUP BY Id, Name HAVING COUNT(Id) > 2];
  }
}
