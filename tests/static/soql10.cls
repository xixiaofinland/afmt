class UTIL_SortContact_TEST {
  {
    [SELECT Name FROM ApexClass WHERE Name LIKE :addrLikeClause AND (NOT Name LIKE :testLikeClause) AND (NOT Name IN :validatorExclussions)];
  }
}
