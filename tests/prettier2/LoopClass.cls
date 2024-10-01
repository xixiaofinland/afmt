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
    while (i < 10)
      i++;
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

    while (
      i > 5000000 &&
      i < 34034034 &&
      i != 384294723 &&
      i != 47289347 &&
      i != 43249034
    ) {
      System.debug(i);
    }
    while (true) {
      break;
    }
    while (true);
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
    do {
    } while (
      i > 5000000 &&
      i < 34034034 &&
      i != 384294723 &&
      i != 47289347 &&
      i != 43249034
    );
  }
}
