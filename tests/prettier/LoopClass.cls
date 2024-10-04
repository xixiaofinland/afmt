class LoopClass {
  void forLoop() {
    for (Contact a : contacts) {
      System.debug(a);
    }
    for (Contact a : contacts);
    for (;; voidCall());
    for (String s : stringSet);
  }

  void whileLoop() {
    Integer i = 0;
    while (i < 10) {
      System.debug(i);
      i++;
      if (i == 5) {
        break;
      } else {
        continue;
      }
    }
    while (i < 5) {
    }

    while (true) {
      break;
    }
  }

  void doWhileLoop() {
    Integer i = 0;
    do {
      System.debug(i);
      i++;
      if (i == 5) {
        break;
      } else {
        continue;
      }
    } while (count < 10);
    do {
    } while (count < 10);
  }
}
