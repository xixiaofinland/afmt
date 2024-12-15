class A {
    {
        [
            SELECT
                Undeliverable_Address__c,
                ( SELECT is_Address_Override__c, is_B__c, Undeliverable_Address__c FROM Contacts)
            FROM Account
            WHERE Id IN :accountIds
        ];
    }
}
