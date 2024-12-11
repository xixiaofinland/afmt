class SOSLClass {
  void withSnippet() {
    List<List<SObject>> searchResult;
    String a = 'Local';
    searchResult = [FIND 'Hello' WITH SNIPPET];
    searchResult = [FIND 'Hello' WITH SNIPPET(target_length = 120)];
    searchResult = [FIND 'Hello' WITH SNIPPET(TARGET_LENGTH = 120)];
  }
  void withNetwork() {
    List<List<SObject>> searchResult;
    searchResult = [FIND 'Hello' WITH Network = 'NetworkId'];
    searchResult = [
      FIND 'Hello'
      WITH Network IN ('NetworkId1', 'NetworkId2', 'NetworkId3')
    ];
    searchResult = [FIND 'Hello' WITH Network = '00000000000000'];
    searchResult = [
      FIND 'Hello'
      WITH Network IN (
        'NetworkId1',
        'NetworkId2',
        'NetworkId3',
        'NetworkId4',
        'NetworkId5',
        'NetworkId6'
      )
    ];
  }
  void withMetadata() {
    List<List<SObject>> searchResult;
    searchResult = [FIND 'Hello' WITH METADATA = 'LABELS'];
  }
  void withHighlight() {
    List<List<SObject>> searchResult;
    searchResult = [FIND 'Hello' WITH HIGHLIGHT];
  }
  void withSpellCorrection() {
    List<List<SObject>> searchResult;
    searchResult = [FIND 'Hello' WITH SPELL_CORRECTION = true];
    searchResult = [FIND 'Hello' WITH SPELL_CORRECTION = false];
  }
  void returning() {
    List<List<SObject>> searchResult;
    String a = 'World';
    List<String> multiple = new List<String>{ 'Hi', 'World' };
    searchResult = [FIND 'Hello' RETURNING Contact];
    searchResult = [FIND 'Hello' RETURNING Contact, Lead];
    searchResult = [FIND 'Hello' RETURNING Account(Name)];
    searchResult = [FIND 'Hello' RETURNING Contact(FirstName, LastName)];
    searchResult = [FIND 'Hello' RETURNING CustomObject__c(CustomField__c)];
    searchResult = [
      FIND 'Hello'
      RETURNING Contact(FirstName, LastName LIMIT 10), Account(Name, Industry)
    ];
    searchResult = [
      FIND 'Hello'
      RETURNING
        Contact(FirstName, LastName ORDER BY FistName ASC),
        Account,
        Lead(FirstName)
    ];
    searchResult = [
      FIND 'Hello'
      RETURNING
        Contact(FirstName, LastName ORDER BY LastName DESC LIMIT 20),
        Account(Name, Industry LIMIT 10),
        Opportunity
      LIMIT 50
    ];
    searchResult = [
      FIND 'Hello'
      RETURNING Contact(FirstName, LastName OFFSET 10)
    ];
    searchResult = [
      FIND 'Hello'
      IN ALL FIELDS
      RETURNING Account(Id, Name USING ListView = ListViewName)
    ];
    searchResult = [
      FIND 'test'
      RETURNING Account(id WHERE createddate = THIS_FISCAL_QUARTER)
    ];
    searchResult = [FIND 'test' RETURNING Account(id, Name WHERE Name = :a)];
    searchResult = [FIND 'test' RETURNING Account(id WHERE NAME IN :multiple)];
    searchResult = [
      FIND 'San Francisco'
      RETURNING
        My_Custom_Object__c(
          Id
          WHERE
            DISTANCE(My_Location_Field__c, GEOLOCATION(37, 122), 'mi') < 100)
    ];
  }
  void updateStat() {
    List<List<String>> searchResult;
    searchResult = [
      FIND 'Title'
      RETURNING
        FAQ__kav(
          Title
          WHERE
            PublishStatus = 'Online'
            AND language = 'en_US'
            AND KnowledgeArticleVersion = 'ka230000000PCiy')
      UPDATE VIEWSTAT
    ];
    searchResult = [
      FIND 'Keyword'
      RETURNING
        KnowledgeArticleVersion(
          Title
          WHERE PublishStatus = 'Online' AND language = 'en_US')
      UPDATE TRACKING
    ];
    searchResult = [
      FIND 'Keyword'
      RETURNING
        KnowledgeArticleVersion(
          Title
          WHERE PublishStatus = 'Online' AND language = 'en_US')
      UPDATE TRACKING, VIEWSTAT
    ];
  }
}
