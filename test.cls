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
}
