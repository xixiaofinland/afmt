class Chaining {
  {
    [SELECT Id, Name FROM acc].super.a;

    a?.super.Set();

    super.a.set();
  }
}
