class A {
  {
    Account[] account =
      new Account[]{
        new Account(Id = accountId, Name = 'Inner Acme One'),
        new Account(Id = accountId, Name = 'Inner Acme Two')
      };

    List<Account> accounts =
      [SELECT Id FROM Account WHERE CreatedDate = LAST_N_DAYS:60];

    List<Account> accounts =
      [
        SELECT Id, Name, Date, ABC__c
        FROM Account
        WHERE CreatedDate = LAST_N_DAYS:60
      ];

    Integer[] b = new List<Integer>{};

    Id[] accountIds = new Id[]{ accountId };

    Map<String, Integer > intsBymap
                = new Map<String, Integer>
                 {    'hello'    => 1  ,
                  'world' =>    2    }   ;

  }
}
