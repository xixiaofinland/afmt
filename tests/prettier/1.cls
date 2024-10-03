class SafeNavigationOperator {
  public TriggerEvent enableBeforeInsert() {
    for (Schema.SObjectType sObjectType : m_sObjectTypes) {
      // register the type
      handleRegisterType(sObjectType);
    }
  }
}
