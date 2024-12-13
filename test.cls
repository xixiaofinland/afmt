class Chaining {
  {

    [Select Id, Name From acc].a;

    String g1 = [
      SELECT Id, Name, One_Field__c, Another_Field__c
      FROM Account
      WHERE Id = :accId
    ];

    String g2 = [
      SELECT Id
      FROM Account
    ].a;

    String g3 = [
      SELECT Id, Name, One_Field__c, Another_Field__c
      FROM Account
      WHERE Id = :accId
    ]
    .Name().b.c.aa();

    String g3 = [
      SELECT Id, Name, One_Field__c, Another_Field__c
      FROM Account
      WHERE Id = :accId
    ]
    .Name;

    SObjectType.Attribute__c.fields.Name.isAccessible();

    return generator.objectBuilder
      .setExternalId(account.personContactId)
      .getInstance();
  }
}
