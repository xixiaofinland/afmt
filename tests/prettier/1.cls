class SafeNavigationOperator {
  public TriggerEvent enableBeforeInsert() {
    for (SObjectField field : getSObjectFieldList())
      queryFactory.selectField(
        relationshipFieldPath + '.' + field.getDescribe().getName()
      );
  }
}
