class UTIL_SortContact_TEST {
  {
    [SELECT Id FROM AuthSession WHERE UsersId IN :userIds AND IsCurrent = TRUE ORDER BY ParentId NULLS FIRST];
  }
}
