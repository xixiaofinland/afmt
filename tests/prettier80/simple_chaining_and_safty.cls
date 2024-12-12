class Chaining {
  {
    a[0].super.c();
    super.a()?.b();
    super.a.b();

    a[0]?.super.c();
    super.a?.b();
    super?.a?.b();
  }
}
