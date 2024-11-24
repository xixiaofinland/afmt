class A {
  {
    Integer[] b = new List<Integer>{};

    Account[] account = new Account[]{
      new Account(Id = accountId, Name = 'Inner Acme One'),
      new Account(Id = accountId, Name = 'Inner Acme Two')
    };

    Id[] accountIds = new Id[]{ accountId };
  }
}
