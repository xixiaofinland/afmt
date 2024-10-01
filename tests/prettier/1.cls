class LoopClass {
  void forLoop() {
    for (Contact a: contacts) {
      System.debug(a);
    }
    for (Contact a: contacts);
    for (;; voidCall());
    for (String s: stringSet);
  }

}
