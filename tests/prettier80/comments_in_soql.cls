class TestClass {
  {
    List<SObject> records = [
      // test comment 1
      SELECT
        // test comment 2
        // test comment 3
        Id,
        Name, // test comment 4
        // test comment 5
        Phone // test comment 6
      // test comment 7
      FROM // test comment 8
      Account // test comment 9
    ];
  }
}
