public class Me {
  {
    List<Account> a1 = [
        SELECT
          Id,Name,Good__c,Bad__c,hello__c,WhatTTTTTTTTTTTT__c,goodstuff__c
           FROM Account
           WHERE Name = 'hellooooooooooo'
      ];

    List<Account> a2 = [
        SELECT
          Id,Name
           FROM Account
           WHERE Name = 'hellooooooooooo'
      ];
  }
}

