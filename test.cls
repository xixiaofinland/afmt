class Ternary {
  void ternaryWithMethodCall() {
    veryLongVariableNameThatWillSurelyBreak == 'Yes' ? VeryLongClassNameThatWillForSureBeTooLongForThisMethodToNotBreak.someMethod() : VeryLongClassNameThatWillForSureBeTooLongForThisMethodToNotBreak.anotherMethod();

      String msg1 = num > 10 ? 'Number is greater than 10' : 'Number is equal to 10';

      String msg2 = num > 10 ? 'Number is greater than 10' : 'Number is less than or equal to 10';

      List<String> a = (veryLongVariableNameThatWillSurelyBreak == 'Yes' ? new List<String>{ 'First One', 'Second One', 'Third One', 'Fourth One', 'Fifth One' } : new List<String>{ 'First One', 'Second One', 'Third One', 'Fourth One', 'Fifth One', 'Sixth One' });
  }
}
