class SOSLClass {
  void returning() {
    searchResult = [FIND 'Hello' IN ALL FIELDS RETURNING Account(Id, Name USING ListView=ListViewName)];
  }
}
