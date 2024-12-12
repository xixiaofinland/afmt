class Ternary {
  void ternaryWithMethodCall() {
      String msg1 = num > 10 ? 'Number is greater than 10' : 'Number is equal to 10';

      String msg2 = num > 10 ? 'Number is greater than 10' : 'Number is less than or equal to 10';

      List<String> a = (veryLongVariableNameThatWillSurelyBreak == 'Yes' ? new List<String>{ 'First One', 'Second One', 'Third One', 'Fourth One', 'Fifth One' } : new List<String>{ 'First One', 'Second One', 'Third One', 'Fourth One', 'Fifth One', 'Sixth One' });
  }

  void ternaryWithSoql() {
    List<Account> accounts = true
      ? [
          SELECT Id
          FROM Account
          WHERE
            Name = 'a name'
            OR Name = 'another name'
            OR Name = 'one more name'
        ]
      : [
          SELECT Id
          FROM Account
          WHERE
            Name = 'a name'
            OR Name = 'another name'
            OR Name = 'one more name'
        ];
  }
}
