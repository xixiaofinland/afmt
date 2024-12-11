class SOSLClass {
  void check() {
    searchResult = [
      FIND 'test'
      RETURNING Account(id WHERE createddate = THIS_FISCAL_QUARTER)
    ];
  }
}
