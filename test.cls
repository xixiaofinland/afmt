@isTest
class RunAsClass {
  static testmethod void runAsTest() {
    User userTwo = [SELECT Id FROM User LIMIT 1 OFFSET 1];
}

