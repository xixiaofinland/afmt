public class A {
  {
    return [
      SELECT
        Id 
      FROM property__c
      WHERE //test123
        (title__c /*t*/ LIKE :key
        OR city__c LIKE :key)
        AND (title__c LIKE :visualKey
        OR tags__c LIKE :visualKey)
        AND price__c >= :minPrice
      WITH SECURITY_ENFORCED
      ORDER BY price__c
      LIMIT 100
    ];
  }
}
