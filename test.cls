class T {
  {
    [SELECT Id FROM Account GROUP BY Id, Name HAVING COUNT(Id) > 2];
  }
}
