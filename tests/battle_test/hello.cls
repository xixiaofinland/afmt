class Chaining {
  {
    [SELECT Id, Name FROM acc].super.a;

    a
      ?.super.DuperVeryInsanelyLongVariableNameThatWillSurelyBreak
      .getOneSet()
      .getAnotherSet()
      .getYetAnotherSet();

    a.super.DuperVeryInsanelyLongVariableNameThatWillSurelyBreak
      .getOneSet()
      .getAnotherSet()
      .getYetAnotherSet();

    super.DuperVeryInsanelyLongVariableNameThatWillSurelyBreak
      .getOneSet()
      .getAnotherSet()
      .getYetAnotherSet();

    super?.superDuperVeryInsanelyLongVariableNameThatWillSurelyBreak
      .getOneSet()
      .getAnotherSet()
      .getYetAnotherSet();

    a.this
      ?.superDuperVeryInsanelyLongVariableNameThatWillSurelyBreak
      .getOneSet()
      .getAnotherSet()
      .getYetAnotherSet();

    this?.superDuperVeryInsanelyLongVariableNameThatWillSurelyBreak
      .getOneSet()
      .getAnotherSet()
      .getYetAnotherSet();

    t.a().b().c().d();
  }
}
