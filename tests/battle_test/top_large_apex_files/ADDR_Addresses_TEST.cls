/*
    Copyright (c) 2014, Salesforce.org
    All rights reserved.
    
    Redistribution and use in source and binary forms, with or without
    modification, are permitted provided that the following conditions are met:
    
    * Redistributions of source code must retain the above copyright
      notice, this list of conditions and the following disclaimer.
    * Redistributions in binary form must reproduce the above copyright
      notice, this list of conditions and the following disclaimer in the
      documentation and/or other materials provided with the distribution.
    * Neither the name of Salesforce.org nor the names of
      its contributors may be used to endorse or promote products derived
      from this software without specific prior written permission.
 
    THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
    "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT 
    LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS 
    FOR A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE 
    COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, 
    INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, 
    BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; 
    LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER 
    CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT 
    LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN 
    ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE 
    POSSIBILITY OF SUCH DAMAGE.
*/
/**
* @author Salesforce.org
* @date 2013
* @group Address Management
* @description Tests specific to Address Management. 
*/

@IsTest
public with sharing class ADDR_Addresses_TEST {

    /*********************************************************************************************************
    * @description Holds the number of Households to create in tests.
    */
    private static final Integer totalAccountsToCreate = 2;

    /*********************************************************************************************************
    * @description Holds the number of Contacts per Household to create in tests.
    */
    static Integer totalContactsToCreate = 2;

    /*********************************************************************************************************
    * @description The list of created test Contacts.
    */
    public static List<Contact> testContacts;

    /*********************************************************************************************************
    * @description The list of created test Accounts.
    */
    public static List<Account> testAccounts;

    /*********************************************************************************************************
    * @description The list of created test Addresses.
    */
    public static List<Address__c> testAddresses;

    private static OrgConfig orgConfig = new OrgConfig();

    /*********************************************************************************************************
    @description
        create N new default Addresses for N Households (with M Contacts)
    verify:
        N addresses created
        N household billing address fields updated
        M contact Mailing address fields updated
    **********************************************************************************************************/
    @IsTest
    private static void newDefaultAddress() {
        // this creates a default Address for each HH
        createTestData(totalAccountsToCreate, totalContactsToCreate);
        Test.startTest();
        Test.stopTest();

        // verify that the HH and Contacts share the same address
        Map<Id, Account> mapAccIdAcc = new Map<Id, Account>([SELECT Id, Name, BillingStreet, BillingCity, BillingState, BillingPostalCode, BillingCountry, BillingLatitude, BillingLongitude FROM Account]);
        List<Contact> listCon = [SELECT Id, Name, AccountId, is_Address_Override__c, Current_Address__c, MailingStreet, MailingCity, MailingState, MailingPostalCode, MailingCountry, MailingLatitude, MailingLongitude FROM Contact];

        for (Contact con : listCon) {
            Account acc = mapAccIdAcc.get(con.AccountId);
            System.assertEquals(true, isMatchAddressAccCon(acc, con));
            System.assertEquals(false, con.is_Address_Override__c);
            System.assertNotEquals(null, con.Current_Address__c);
        }
    }

    @IsTest
    private static void newAccountUndeliverableStatusShouldSyncToNewDefaultAddress() {
        //skip the test if Advancement is installed
        if (ADV_PackageInfo_SVC.useAdv()) {
            return;
        }

        Account testAccount = UTIL_UnitTestData_TEST.buildHouseholdAccount();
        testAccount.BillingStreet = 'Undeliverable Street';
        testAccount.BillingCity = 'Undeliverable City';
        testAccount.BillingCountry = 'United States';
        testAccount.BillingPostalCode = 'Undeliverable Zip';
        testAccount.BillingState = 'Washington';
        testAccount.Undeliverable_Address__c = true;

        Test.startTest();
            insert testAccount;
        Test.stopTest();

        Address__c updatedAddress = [SELECT Undeliverable__c FROM Address__c WHERE Household_Account__c =
                :testAccount.Id];
        Account updatedAccount = [SELECT Undeliverable_Address__c FROM Account WHERE Id = :testAccount.Id];

        System.assert(updatedAccount.Undeliverable_Address__c, 'The billing address for the account should be ' +
                'marked undeliverable.');
        System.assertEquals(updatedAccount.Undeliverable_Address__c, updatedAddress.Undeliverable__c, 'The new ' +
                'address undeliverable status should match the new account undeliverable status.');
    }

    @IsTest
    private static void updatedAccountUndeliverableStatusShouldSyncToExistingDefaultAddressAndContacts() {
        //skip the test if Advancement is installed
        if (ADV_PackageInfo_SVC.useAdv()) {
            return;
        }

        Account testAccount = UTIL_UnitTestData_TEST.buildHouseholdAccount();
        testAccount.BillingStreet = 'Undeliverable Street';
        testAccount.BillingCity = 'Undeliverable City';
        testAccount.BillingCountry = 'United States';
        testAccount.BillingPostalCode = 'Undeliverable Zip';
        testAccount.BillingState = 'Washington';
        testAccount.Undeliverable_Address__c = false;
        insert testAccount;

        List<Contact> testContacts = UTIL_UnitTestData_TEST.createMultipleTestContacts(3);
        for (Contact testContact : testContacts) {
            testContact.AccountId = testAccount.Id;
        }
        insert testContacts;

        Test.startTest();
            Account accountToUpdate = [SELECT Undeliverable_Address__c FROM Account WHERE Id = :testAccount.Id];
            accountToUpdate.Undeliverable_Address__c = true;

            update accountToUpdate;
        Test.stopTest();

        Address__c updatedAddress = [SELECT Undeliverable__c
                                     FROM Address__c
                                     WHERE Household_Account__c = :testAccount.Id];

        Account updatedAccount = [SELECT Undeliverable_Address__c,
                                    (SELECT Undeliverable_Address__c FROM Contacts)
                                  FROM Account WHERE Id = :testAccount.Id];

        System.assert(updatedAccount.Undeliverable_Address__c, 'The billing address for the account should be ' +
                'marked undeliverable.');
        System.assertEquals(updatedAccount.Undeliverable_Address__c, updatedAddress.Undeliverable__c, 'The ' +
                'address undeliverable status should match the updated account undeliverable status.');

        for (Contact contact : updatedAccount.Contacts) {
            System.assertEquals(updatedAccount.Undeliverable_Address__c, contact.Undeliverable_Address__c, 'The ' +
                    'contact undeliverable address should match the house account undeliverable address.');
        }
    }

    /*********************************************************************************************************
    @description
        insert contact w/ address
    verify:
        new default Address created
        contact address lookup updated to new address
        the contact gets the new Address
    **********************************************************************************************************/
    @IsTest
    private static void insertContactWithAddress() {
        //skip the test if Advancement is installed
        if (ADV_PackageInfo_SVC.useAdv()) {
            return;
        }

        npe01__Contacts_And_Orgs_Settings__c contactSettingsForTests = UTIL_CustomSettingsFacade.getContactsSettings();

        testContacts = UTIL_UnitTestData_TEST.createMultipleTestContacts(totalContactsToCreate);
        for (Contact con : testContacts) {
            con.MailingStreet = 'new street';
            con.MailingCity = 'new city';
            con.npe01__Primary_Address_Type__c = 'Other';
        }
        Test.startTest();
        insert testContacts;
        Test.stopTest();

        // verify that the HH and Contacts share the same address
        Map<Id, Account> mapAccIdAcc = new Map<Id, Account>([SELECT Id, Name, BillingStreet, BillingCity, BillingState, BillingPostalCode, BillingCountry, BillingLatitude, BillingLongitude FROM Account]);
        List<Contact> listCon = [SELECT Id, Name, AccountId, is_Address_Override__c, Current_Address__c, npe01__Primary_Address_Type__c, MailingStreet, MailingCity, MailingState, MailingPostalCode, MailingCountry, MailingLatitude, MailingLongitude FROM Contact];

        for (Contact con : listCon) {
            Account acc = mapAccIdAcc.get(con.AccountId);
            System.assertEquals(true, isMatchAddressAccCon(acc, con));
            System.assertEquals(false, con.is_Address_Override__c);
            System.assertNotEquals(null, con.Current_Address__c);
            System.assertEquals('Other', con.npe01__Primary_Address_Type__c);
        }

        List<Address__c> listAddr = [
            SELECT Id, Default_Address__c, MailingStreet__c, Household_Account__c, Address_Type__c
            FROM Address__c
            WHERE Default_Address__c = TRUE
        ];
        System.assertEquals(totalContactsToCreate, listAddr.size());
        for (Address__c addr : listAddr) {
            System.assertEquals('Other', addr.Address_Type__c);
        }
    }

    /*********************************************************************************************************
    @description
        insert contact w/ address, and force recreate of hh account
    verify:
        new default Address created
        contact address lookup updated to new address
        the contact gets the new Address
        the household account gets the new Address
    **********************************************************************************************************/
    @IsTest
    private static void insertContactWithAddressRecreateAccount() {
        //skip the test if Advancement is installed
        if (ADV_PackageInfo_SVC.useAdv()) return;

        TDTM_DefaultConfig.startTestDataInit();
        npe01__Contacts_And_Orgs_Settings__c contactSettingsForTests = UTIL_CustomSettingsFacade.getContactsSettings();

        testContacts = UTIL_UnitTestData_TEST.createMultipleTestContacts(totalContactsToCreate);
        for (Contact con : testContacts) {
            con.MailingStreet = 'new street';
            con.MailingCity = 'new city';
        }
        insert testContacts;
        TDTM_DefaultConfig.stopTestDataInit();

        // null out account, so a new account will be created.
        Test.startTest();
        for (Contact con : testContacts) {
            con.AccountId = null;
        }
        update testContacts;
        Test.stopTest();

        // verify that the HH and Contacts share the same address
        Map<Id, Account> mapAccIdAcc = new Map<Id, Account>([SELECT Id, Name, BillingStreet, BillingCity, BillingState, BillingPostalCode, BillingCountry, BillingLatitude, BillingLongitude FROM Account]);
        List<Contact> listCon = [SELECT Id, Name, AccountId, is_Address_Override__c, Current_Address__c, MailingStreet, MailingCity, MailingState, MailingPostalCode, MailingCountry, MailingLatitude, MailingLongitude FROM Contact];

        for (Contact con : listCon) {
            Account acc = mapAccIdAcc.get(con.AccountId);
            System.assertEquals(true, isMatchAddressAccCon(acc, con));
            System.assertEquals(false, con.is_Address_Override__c);
            System.assertNotEquals(null, con.Current_Address__c);
        }
    }

    /*********************************************************************************************************
    @description
        insert contact w/ new address override
    verify:
        new non-default Address created
        contact address lookup updated to new address
        the contact gets the new Address
    **********************************************************************************************************/
    @IsTest
    private static void insertContactWithAddressOverride() {
        //skip the test if Advancement is installed
        if (ADV_PackageInfo_SVC.useAdv()) return;

        npe01__Contacts_And_Orgs_Settings__c contactSettingsForTests = UTIL_CustomSettingsFacade.getContactsSettings();

        testContacts = UTIL_UnitTestData_TEST.createMultipleTestContacts(totalContactsToCreate);
        Integer i = 1;
        for (Contact con : testContacts) {
            con.MailingStreet = 'new street' + i;
            con.MailingCity = 'new city' + i;
            con.is_Address_Override__c = true;
            i++;
        }
        Test.startTest();
        insert testContacts;
        Test.stopTest();

        // verify that the HH and Contacts share the same address
        Map<Id, Account> mapAccIdAcc = new Map<Id, Account>([SELECT Id, Name, BillingStreet, BillingCity, BillingState, BillingPostalCode, BillingCountry, BillingLatitude, BillingLongitude FROM Account]);
        List<Contact> listCon = [SELECT Id, Name, AccountId, is_Address_Override__c, Current_Address__c, MailingStreet, MailingCity, MailingState, MailingPostalCode, MailingCountry, MailingLatitude, MailingLongitude FROM Contact];

        for (Contact con : listCon) {
            Account acc = mapAccIdAcc.get(con.AccountId);
            System.assertEquals(false, isMatchAddressAccCon(acc, con));
            System.assertEquals(null, acc.BillingStreet);
            System.assertEquals(true, con.is_Address_Override__c);
            System.assertNotEquals(null, con.Current_Address__c);
        }
    }

    /*********************************************************************************************************
    @description
        update existing default Addresses
    verify:
        household billing address fields updated
        contact Mailing address fields updated
    **********************************************************************************************************/
    @IsTest
    private static void updateDefaultAddr() {

        // this creates a default Address for each HH
        createTestData(totalAccountsToCreate, totalContactsToCreate);

        // now let's update the Addresses
        for (Integer i = 0; i < totalAccountsToCreate; i++) {
            Address__c addr = testAddresses[i];
            addr.MailingStreet__c = 'New Street' + i;
            addr.MailingCity__c = 'New City' + i;
        }
        Test.startTest();
        update testAddresses;
        Test.stopTest();

        // verify that the HH and Contacts share the same address and it's new!
        Map<Id, Account> mapAccIdAcc = new Map<Id, Account>([SELECT Id, Name, BillingStreet, BillingCity, BillingState, BillingPostalCode, BillingCountry, BillingLatitude, BillingLongitude FROM Account]);
        List<Contact> listCon = [SELECT Id, Name, AccountId, is_Address_Override__c, Current_Address__c, MailingStreet, MailingCity, MailingState, MailingPostalCode, MailingCountry, MailingLatitude, MailingLongitude FROM Contact];
        System.assertEquals(totalAccountsToCreate * totalContactsToCreate, listCon.size());

        for (Contact con : listCon) {
            Account acc = mapAccIdAcc.get(con.AccountId);
            System.assertEquals(true, isMatchAddressAccCon(acc, con));
            System.assert(con.MailingStreet.contains('New Street'));
            System.assert(con.MailingCity.contains('New City'));
            System.assertEquals(false, con.is_Address_Override__c);
            System.assertNotEquals(null, con.Current_Address__c);
        }
    }

    /*********************************************************************************************************
    @description
        delete existing default Addresses
    verify:
        household billing address fields cleared
        contact Mailing address fields cleared
    **********************************************************************************************************/
    @IsTest
    private static void deleteDefaultAddr() {

        // this creates a default Address for each HH
        createTestData(totalAccountsToCreate, totalContactsToCreate);

        // now let's delete the Addresses
        Test.startTest();
        delete testAddresses;
        Test.stopTest();

        // verify that the HH and Contacts' address fields are cleared
        Map<Id, Account> mapAccIdAcc = new Map<Id, Account>([SELECT Id, Name, BillingStreet, BillingCity, BillingState, BillingPostalCode, BillingCountry, BillingLatitude, BillingLongitude FROM Account]);
        List<Contact> listCon = [SELECT Id, Name, AccountId, is_Address_Override__c, Current_Address__c, MailingStreet, MailingCity, MailingState, MailingPostalCode, MailingCountry, MailingLatitude, MailingLongitude FROM Contact];
        System.assertEquals(totalAccountsToCreate * totalContactsToCreate, listCon.size());

        for (Contact con : listCon) {
            Account acc = mapAccIdAcc.get(con.AccountId);
            System.assertEquals(true, isMatchAddressAccCon(acc, con));
            System.assertEquals(null, con.MailingStreet);
            System.assertEquals(null, con.MailingCity);
            System.assertEquals(false, con.is_Address_Override__c);
            System.assertEquals(null, con.Current_Address__c);
            System.assertEquals(null, acc.BillingStreet);
            System.assertEquals(null, acc.BillingCity);
        }
    }

    /*********************************************************************************************************
    @description
        delete existing non-default override Addresses
    verify:
        household billing address fields not changed
        contact Mailing address fields reset to default
    **********************************************************************************************************/
    @IsTest
    private static void deleteNonDefaultOverrideAddr() {

        // this creates a default Address for each HH
        createTestData(totalAccountsToCreate, totalContactsToCreate);

        // create additional addresses
        buildTestAddress(totalAccountsToCreate);
        for (Integer i = 0; i < totalAccountsToCreate; i++) {
            testAddresses[i].Household_Account__c = testAccounts[i].Id;
            testAddresses[i].Default_Address__c = false;
            testAddresses[i].MailingStreet__c = 'override' + i;
            testAddresses[i].MailingCity__c = 'override' + i;
        }
        insert testAddresses;

        // set the contacts' address overrides
        List<Contact> listCon = new List<Contact>();
        for (Integer i = 0; i < totalAccountsToCreate; i++) {
            Contact con = testContacts[i * totalAccountsToCreate + i];
            con.Current_Address__c = testAddresses[i].Id;
            con.is_Address_Override__c = true;
            listCon.add(con);
        }
        update listCon;

        // now let's delete the override Addresses
        Test.startTest();
        delete testAddresses;
        Test.stopTest();

        // verify that the HH and Contacts' address fields the default
        Map<Id, Account> mapAccIdAcc = new Map<Id, Account>([SELECT Id, Name, BillingStreet, BillingCity, BillingState, BillingPostalCode, BillingCountry, BillingLatitude, BillingLongitude FROM Account]);
        listCon = [SELECT Id, Name, AccountId, is_Address_Override__c, Current_Address__c, MailingStreet, MailingCity, MailingState, MailingPostalCode, MailingCountry, MailingLatitude, MailingLongitude FROM Contact];
        System.assertEquals(totalAccountsToCreate * totalContactsToCreate, listCon.size());

        for (Contact con : listCon) {
            Account acc = mapAccIdAcc.get(con.AccountId);
            System.assertEquals(true, isMatchAddressAccCon(acc, con));
            System.assertEquals(false, con.is_Address_Override__c);
            System.assertNotEquals(null, con.Current_Address__c);
            System.assert(con.MailingStreet.contains('Street'));
            System.assert(con.MailingCity.contains('City'));
        }
    }

    /*********************************************************************************************************
    @description
        update existing non-default Addresses
    verify:
        no change to HH and contact addresses
    **********************************************************************************************************/
    @IsTest
    private static void updateNonDefaultAddr() {
        // this creates a default Address for each HH
        createTestData(totalAccountsToCreate, totalContactsToCreate);

        // now let's update the Addresses
        for (Integer i = 0; i < totalAccountsToCreate; i++) {
            Address__c addr = testAddresses[i];
            addr.Default_Address__c = false;
            addr.MailingStreet__c = 'New Street' + i;
            addr.MailingCity__c = 'New City' + i;
        }
        Test.startTest();
        update testAddresses;
        Test.stopTest();

        // verify that the HH and Contacts share the same address and it's new!
        Map<Id, Account> mapAccIdAcc = new Map<Id, Account>([SELECT Id, Name, BillingStreet, BillingCity, BillingState, BillingPostalCode, BillingCountry, BillingLatitude, BillingLongitude FROM Account]);
        List<Contact> listCon = [SELECT Id, Name, AccountId, is_Address_Override__c, Current_Address__c, MailingStreet, MailingCity, MailingState, MailingPostalCode, MailingCountry, MailingLatitude, MailingLongitude FROM Contact];
        System.assertEquals(totalAccountsToCreate * totalContactsToCreate, listCon.size());

        for (Contact con : listCon) {
            Account acc = mapAccIdAcc.get(con.AccountId);
            System.assertEquals(true, isMatchAddressAccCon(acc, con));
            System.assert(!con.MailingStreet.contains('New Street'));
            System.assert(!con.MailingCity.contains('New City'));
            System.assertEquals(false, con.is_Address_Override__c);
            System.assertNotEquals(null, con.Current_Address__c);
        }
    }

    /*********************************************************************************************************
    @description
        set contact address override
    verify:
        contact's address matches override
    **********************************************************************************************************/
    @IsTest
    private static void setConAddrOverride() {

        // this creates a default Address for each HH
        createTestData(totalAccountsToCreate, totalContactsToCreate);

        // create a new non-default address
        Address__c addrNew = new Address__c();
        addrNew.MailingStreet__c = 'street override';
        addrNew.MailingCity__c = 'city override';
        addrNew.Default_Address__c = false;
        addrNew.Household_Account__c = testAccounts[0].Id;
        insert addrNew;

        // set a contact's address override
        testContacts[0].Current_Address__c = addrNew.Id;
        testContacts[0].is_Address_Override__c = true;
        update testContacts[0];

        // now let's update the default Addresses
        for (Integer i = 0; i < totalAccountsToCreate; i++) {
            Address__c addr = testAddresses[i];
            addr.MailingStreet__c = 'New Street' + i;
            addr.MailingCity__c = 'New City' + i;
        }
        Test.startTest();
        update testAddresses;
        Test.stopTest();

        // verify that the HH and Contacts share the same address and it's new!
        Map<Id, Account> mapAccIdAcc = new Map<Id, Account>([SELECT Id, Name, BillingStreet, BillingCity, BillingState, BillingPostalCode, BillingCountry, BillingLatitude, BillingLongitude FROM Account]);
        List<Contact> listCon = [SELECT Id, Name, AccountId, is_Address_Override__c, Current_Address__c, MailingStreet, MailingCity, MailingState, MailingPostalCode, MailingCountry, MailingLatitude, MailingLongitude FROM Contact];
        System.assertEquals(totalAccountsToCreate * totalContactsToCreate, listCon.size());

        for (Contact con : listCon) {
            Account acc = mapAccIdAcc.get(con.AccountId);
            if (con.is_Address_Override__c) {
                System.assertEquals(con.Current_Address__c, addrNew.Id);
                System.assertEquals(false, isMatchAddressAccCon(acc, con), 'The Household address should not have been updated from the Contact');
                System.assert(!con.MailingStreet.contains('New Street'));
                System.assert(!con.MailingCity.contains('New City'));
            } else {
                System.assertEquals(true, isMatchAddressAccCon(acc, con), 'The Household address should have been updated from the Contact');
                System.assert(con.MailingStreet.contains('New Street'));
                System.assert(con.MailingCity.contains('New City'));
                System.assertEquals(false, con.is_Address_Override__c);
                System.assertNotEquals(null, con.Current_Address__c);
            }
        }
    }

    /*********************************************************************************************************
    @description
        set contact current address, but not override
    verify:
        contact's address updated, but not overriden; the household accounts address remains unchanged
    **********************************************************************************************************/
    @IsTest
    private static void setConAddrNoOverride() {
        //skip the test if Advancement is installed
        if (ADV_PackageInfo_SVC.useAdv()) return;

        // this creates a default Address for each HH
        createTestData(totalAccountsToCreate, totalContactsToCreate);

        // create additional addresses
        buildTestAddress(totalAccountsToCreate);
        for (Integer i = 0; i < totalAccountsToCreate; i++) {
            testAddresses[i].Household_Account__c = testAccounts[i].Id;
            testAddresses[i].Default_Address__c = false;
            testAddresses[i].MailingStreet__c = 'not override' + i;
            testAddresses[i].MailingCity__c = 'not override' + i;
        }
        insert testAddresses;

        // set the contacts' "Current Address" to the new 'not override' address, but don't check "Is Address Override"
        List<Contact> listCon = new List<Contact>();
        for (Integer i = 0; i < totalAccountsToCreate; i++) {
            Contact con = testContacts[i * totalAccountsToCreate + i];
            con.Current_Address__c = testAddresses[i].Id;
            con.is_Address_Override__c = false;
            listCon.add(con);
        }
        Test.startTest();
        update listCon;
        Test.stopTest();

        Map<Id, Account> mapAccIdAcc = new Map<Id, Account>([SELECT Id, Name, BillingStreet, BillingCity, BillingState, BillingPostalCode, BillingCountry, BillingLatitude, BillingLongitude FROM Account]);
        listCon = [SELECT Id, Name, AccountId, is_Address_Override__c, Current_Address__c, Current_Address__r.Default_Address__c, MailingStreet, MailingCity, MailingState, MailingPostalCode, MailingCountry, MailingLatitude, MailingLongitude FROM Contact];

        Integer nonOverrideAddressCounter = 0;

        for (Address__c address : [SELECT MailingCity__c FROM Address__c WHERE Default_Address__c = FALSE]) {
            if (address.MailingCity__c.startsWith('not')) {
                nonOverrideAddressCounter++;
            }
        }
        
        System.assertEquals(totalAccountsToCreate, nonOverrideAddressCounter,
            'There should be one "not override" address per Household not set as default address');

        Integer cNotOverride = 0;
        for (Contact con : listCon) {
            Account acc = mapAccIdAcc.get(con.AccountId);
            if (con.MailingStreet.contains('not override') == true) {
                cNotOverride++;
                System.assertEquals(false, con.Current_Address__r.Default_Address__c);
                System.assertEquals(false, isMatchAddressAccCon(acc, con), 'The Household address should not have been updated from the Contact');
                System.assert(con.MailingStreet.contains('not override'));
                System.assert(con.MailingCity.contains('not override'));
                System.assertEquals(false, con.is_Address_Override__c);
                System.assertNotEquals(null, con.Current_Address__c);
            } else {
                System.assertEquals(true, con.Current_Address__r.Default_Address__c);
                System.assertEquals(true, isMatchAddressAccCon(acc, con), 'The Household address should remain unchanged from before and match the Contact that was not updated');
                System.assert(con.MailingStreet.contains('Street'));
                System.assert(con.MailingCity.contains('City'));
                System.assertEquals(false, con.is_Address_Override__c);
                System.assertNotEquals(null, con.Current_Address__c);
            }
        }
        System.assertEquals(totalAccountsToCreate, cNotOverride, 'There should be one Contact per Household with the "not override" address');
    }

    @isTest
    private static void newUndeliverableAddressShouldSyncToAccount() {
        List<Account> accounts = UTIL_UnitTestData_TEST.createMultipleTestAccounts(5, CAO_Constants.HH_ACCOUNT_TYPE);
        insert accounts;

        buildTestAddress(5);

            for (Integer i=0; i < testAddresses.size(); i++) {
                testAddresses[i].Household_Account__c = accounts[i].Id;
                testAddresses[i].Default_Address__c = true;
                testAddresses[i].Undeliverable__c = true;
            }


        Test.startTest();
            insert testAddresses;
        Test.stopTest();

        Set<Id> accountIds = (new Map<Id,Account>(accounts)).keySet();
        Map<Id, Account> updatedAccountsById = new Map<Id, Account>(
            [SELECT Undeliverable_Address__c FROM Account WHERE Id IN :accountIds]);

        for (Address__c address: testAddresses) {
            System.assertEquals(address.Undeliverable__c,
                    updatedAccountsById.get(address.Household_Account__c).Undeliverable_Address__c, 'account address' +
                    ' undeliverable status should match the address undeliverable status if the address is a default ' +
                            'address.');
        }
    }

    @isTest
    private static void updatedUndeliverableAddressShouldSyncToAccount() {
        List<Account> accounts = UTIL_UnitTestData_TEST.createMultipleTestAccounts(5, CAO_Constants.HH_ACCOUNT_TYPE);
        insert accounts;

        buildTestAddress(5);

        for (Integer i=0; i < testAddresses.size(); i++) {
            testAddresses[i].Household_Account__c = accounts[i].Id;
            testAddresses[i].Default_Address__c = true;
        }
        insert testAddresses;

        Test.startTest();
            for (Address__c address : testAddresses) {
                address.Undeliverable__c = true;
            }
            update testAddresses;
        Test.stopTest();

        Set<Id> accountIds = (new Map<Id,Account>(accounts)).keySet();
        Map<Id, Account> updatedAccountsById = new Map<Id, Account>(
            [SELECT Undeliverable_Address__c FROM Account WHERE Id IN :accountIds]);

        for (Address__c address: testAddresses) {
            System.assertEquals(address.Undeliverable__c,
                    updatedAccountsById.get(address.Household_Account__c).Undeliverable_Address__c, 'account address' +
                            ' undeliverable status should match the address undeliverable status if the address is a ' +
                            'default address.');
        }
    }

    @isTest
    private static void newUndeliverableAddressShouldNotSyncToAccountWhenNotDefault() {
        List<Account> accounts = UTIL_UnitTestData_TEST.createMultipleTestAccounts(2, CAO_Constants.HH_ACCOUNT_TYPE);
        insert accounts;

        buildTestAddress(2);

        for (Integer i=0; i < testAddresses.size(); i++) {
            testAddresses[i].Household_Account__c = accounts[i].Id;
            testAddresses[i].Undeliverable__c = true;
        }

        Test.startTest();
            insert testAddresses;
        Test.stopTest();

        Set<Id> accountIds = (new Map<Id,Account>(accounts)).keySet();
        Map<Id, Account> updatedAccountsById = new Map<Id, Account>(
        [SELECT Undeliverable_Address__c FROM Account WHERE Id IN :accountIds]);

        for (Address__c address: testAddresses) {
            System.assertNotEquals(address.Undeliverable__c,
                    updatedAccountsById.get(address.Household_Account__c).Undeliverable_Address__c, 'account address' +
                            ' undeliverable status should not match the address undeliverable status when the address' +
                            ' is not marked as the default address.');
        }
    }

    @isTest
    private static void updatedUndeliverableAddressShouldNotSyncToAccountWhenNotDefault() {
        List<Account> accounts = UTIL_UnitTestData_TEST.createMultipleTestAccounts(2, CAO_Constants.HH_ACCOUNT_TYPE);
        insert accounts;

        buildTestAddress(2);

        for (Integer i=0; i < testAddresses.size(); i++) {
            testAddresses[i].Household_Account__c = accounts[i].Id;
        }
        insert testAddresses;

        Test.startTest();
            for (Address__c address : testAddresses) {
                address.Undeliverable__c = true;
            }
            update testAddresses;
        Test.stopTest();

        Set<Id> accountIds = (new Map<Id,Account>(accounts)).keySet();
        Map<Id, Account> updatedAccountsById = new Map<Id, Account>(
        [SELECT Undeliverable_Address__c FROM Account WHERE Id IN :accountIds]);

        for (Address__c address: testAddresses) {
            System.assertNotEquals(address.Undeliverable__c,
                    updatedAccountsById.get(address.Household_Account__c).Undeliverable_Address__c, 'account address' +
                            ' undeliverable status should not match the address undeliverable status when the address' +
                            ' is not marked as the default address.');
        }
    }

    @isTest
    private static void newUndeliverableAddressShouldSyncToAccountAndContacts() {

        Test.startTest();
            createTestData(2, 2, true);
        Test.stopTest();

        Set<Id> accountIds = (new Map<Id,Account>(testAccounts)).keySet();
        Map<Id, Account> updatedAccountsById = new Map<Id, Account>(
            [SELECT Undeliverable_Address__c, (SELECT is_Address_Override__c, Undeliverable_Address__c FROM Contacts)
             FROM Account
             WHERE Id IN :accountIds]);

        for (Address__c address: testAddresses) {
            Account updatedAccount = updatedAccountsById.get(address.Household_Account__c);
            System.assertEquals(address.Undeliverable__c,
                    updatedAccount.Undeliverable_Address__c, 'account address' +
                            ' undeliverable status should not match the address undeliverable status when the address' +
                            ' is not marked as the default address.');
            for (Contact contact : updatedAccount.Contacts) {
                System.assert(contact.is_Address_Override__c == false, 'The contact should not be marked with an ' +
                        'address override since that will cause inaccuracies in this test scenario.');
                System.assertEquals(address.Undeliverable__c, contact.Undeliverable_Address__c, 'The contact ' +
                        'undeliverable address status should match the default address undeliverable status when the ' +
                        'contact address override value is false.');
            }
        }
    }

    @isTest
    private static void updatedUndeliverableAddressShouldSyncToAccountAndContacts() {
        createTestData(3, 3);

        Test.startTest();
            for (Address__c address : testAddresses) {
                address.Undeliverable__c = true;
            }
            update testAddresses;
        Test.stopTest();

        Set<Id> accountIds = (new Map<Id,Account>(testAccounts)).keySet();
        Map<Id, Account> updatedAccountsById = new Map<Id, Account>(
        [SELECT Undeliverable_Address__c, (SELECT is_Address_Override__c, Undeliverable_Address__c FROM Contacts)
            FROM Account
            WHERE Id IN :accountIds]);

        for (Address__c address: testAddresses) {
            Account updatedAccount = updatedAccountsById.get(address.Household_Account__c);
            System.assert(address.Default_Address__c);
            System.assertEquals(address.Undeliverable__c, updatedAccount.Undeliverable_Address__c, 'account address' +
                            ' undeliverable status should not match the address undeliverable status.');
            for (Contact contact : updatedAccount.Contacts) {
                System.assert(contact.is_Address_Override__c == false, 'The contact should not be marked with an ' +
                        'address override.');
                System.assertEquals(address.Undeliverable__c, contact.Undeliverable_Address__c, 'The contact ' +
                        'undeliverable address status should match the default address undeliverable status.');
            }
        }
    }

    @isTest
    private static void updatedUndeliverableAddressShouldSyncToAccountAndHouseholdContactOverrides() {
        //skip the test if Advancement is installed
        if (ADV_PackageInfo_SVC.useAdv()) {
            return;
        }

        createTestData(2, 2);
        Address__c overrideAddressToCreate = new Address__c();
            overrideAddressToCreate.Address_Type__c = 'Home';
            overrideAddressToCreate.MailingStreet__c = 'Street Override';
            overrideAddressToCreate.MailingCity__c = 'City Override';
            overrideAddressToCreate.MailingState__c = 'Pennsylvania';
            overrideAddressToCreate.MailingPostalCode__c = 'Zip Override';
            overrideAddressToCreate.MailingCountry__c = 'United States';
            overrideAddressToCreate.Household_Account__c = testAccounts[0].Id;
        insert overrideAddressToCreate;

        Contact contact = new Contact(
            FirstName = 'Test Contact',
            LastName = 'Override',
            is_Address_Override__c = true,
            AccountId = testAccounts[0].Id,
            MailingState = 'Pennsylvania',
            MailingCity = 'City Override',
            MailingStreet = 'Street Override',
            MailingPostalCode = 'Zip Override',
            MailingCountry = 'United States',
            npe01__Primary_Address_Type__c = 'Home'
        );
        insert contact;

        Address__c overrideAddress;
        for (Address__c addr : [SELECT Id, MailingState__c, Undeliverable__c FROM Address__c]) {
            if (addr.MailingState__c == 'Pennsylvania') {
                overrideAddress = addr;
                break;
            }
        }
        
        Test.startTest();
            overrideAddress.Undeliverable__c = true;
            update overrideAddress;
        Test.stopTest();

        Contact updatedContact = [SELECT is_Address_Override__c, Current_Address__c, Current_Address__r.MailingStreet__c, Undeliverable_Address__c
                                  FROM Contact
                                  WHERE Id = :contact.Id];
        System.assert(updatedContact.Current_Address__c == overrideAddress.Id, 'Contact should have the override ' +
                'address as its current address.');

        System.assert(updatedContact.is_Address_Override__c == true, 'The contact address override should be true.');
        System.assertEquals(overrideAddress.Undeliverable__c, updatedContact.Undeliverable_Address__c, 'The contact ' +
                'undeliverable address status should match the override address undeliverable status.');
    }


    @isTest
    private static void updatedUndeliverableNonDefaultAddressShouldSyncToOrganizationContact() {
        npe01__Contacts_And_Orgs_Settings__c settings = UTIL_CustomSettingsFacade.getContactsSettingsForTests( new
                npe01__Contacts_And_Orgs_Settings__c(Organizational_Account_Addresses_Enabled__c = true));

        List<Account> organizationAccounts = UTIL_UnitTestData_TEST.createMultipleTestAccounts(1, null);
        insert organizationAccounts;

        List<Contact> testContacts = UTIL_UnitTestData_TEST.createMultipleTestContacts(1);

        buildTestAddress(3);
        for (Integer i = 0; i < testAddresses.size(); i++) {
            Address__c address = testAddresses[i];
            address.Household_Account__c = organizationAccounts[0].Id;
        }
        insert testAddresses;

        for (Contact contact : testContacts) {
            populateContactAddress(contact, testAddresses[0]);
            contact.is_Address_Override__c = true;
            contact.Current_Address__c = testAddresses[0].Id;
        }
        insert testContacts;

        Test.startTest();
            testAddresses[0].Undeliverable__c = true;
            update testAddresses;
        Test.stopTest();

        Contact updatedContact = [SELECT Undeliverable_Address__c
                                  FROM Contact
                                  WHERE Id = :testContacts[0].Id];
        Address__c updatedAddress = [SELECT Undeliverable__c FROM Address__c WHERE Id = :testAddresses[0].Id];
        System.assertEquals(updatedAddress.Undeliverable__c, updatedContact.Undeliverable_Address__c, 'The ' +
                'non-default organization address undeliverable status should match the organization contact ' +
                'undeliverable status using that address as an override.');
    }

    @isTest
    private static void updatedUndeliverableDefaultAddressDoesNotCauseMultipleOverrideUpdates() {
        createTestData(2, 2);
        Address__c overrideAddressToCreate = new Address__c();
        overrideAddressToCreate.Address_Type__c = 'Home';
        overrideAddressToCreate.MailingStreet__c = 'Street Override';
        overrideAddressToCreate.MailingCity__c = 'City Override';
        overrideAddressToCreate.MailingState__c = 'Washington';
        overrideAddressToCreate.MailingPostalCode__c = 'Zip Override';
        overrideAddressToCreate.MailingCountry__c = 'United States';
        overrideAddressToCreate.Default_Address__c = true;
        overrideAddressToCreate.Household_Account__c = testAccounts[0].Id;
        insert overrideAddressToCreate;

        Contact contact = new Contact(
                FirstName = 'Test Contact',
                LastName = 'Override',
                is_Address_Override__c = true,
                AccountId = testAccounts[0].Id,
                MailingState = 'Washington',
                MailingCity = 'City Override',
                MailingStreet = 'Street Override',
                MailingPostalCode = 'Zip Override',
                MailingCountry = 'United States',
                npe01__Primary_Address_Type__c = 'Home'
        );
        insert contact;

        DmlException updateError;
        Test.startTest();
            overrideAddressToCreate.Undeliverable__c = true;
            try {
                update overrideAddressToCreate;
            } catch (DmlException ex) {
                updateError = ex;
            }
        Test.stopTest();

        Address__c updatedAddress = [SELECT Default_Address__c, Undeliverable__c FROM Address__c WHERE Id = :overrideAddressToCreate.Id];
        Contact updatedContact = [SELECT Undeliverable_Address__c FROM Contact WHERE Id = :contact.Id];

        System.assert(updatedAddress.Default_Address__c, 'The test address should be a default address.');
        System.assertEquals(null, updateError, 'Updating the address undeliverable status' +
            'should not result in an error.');
        System.assertEquals(updatedAddress.Undeliverable__c, updatedContact.Undeliverable_Address__c, 'The contact ' +
                'undeliverable address status should match the address undeliverable status.');

    }

    /*********************************************************************************************************
    @description
        insert new default addresses to HH w/ existing default addresses
    verify:
        contact's && hh address matches new default
        old default addresses no longer marked default
    **********************************************************************************************************/
    @IsTest
    private static void insertNewDefaultAddr() {

        // this creates a default Address for each HH
        createTestData(totalAccountsToCreate, totalContactsToCreate);

        // create additional addresses
        buildTestAddress(totalAccountsToCreate);
        for (Integer i = 0; i < totalAccountsToCreate; i++) {
            testAddresses[i].Household_Account__c = testAccounts[i].Id;
            testAddresses[i].Default_Address__c = true;
            testAddresses[i].MailingStreet__c = 'New Default Street' + i;
            testAddresses[i].MailingCity__c = 'New Default City' + i;
        }
        Test.startTest();
        insert testAddresses;
        Test.stopTest();

        // verify that the HH and Contacts share the same address and it's new!
        Map<Id, Account> mapAccIdAcc = new Map<Id, Account>([SELECT Id, Name, BillingStreet, BillingCity, BillingState, BillingPostalCode, BillingCountry, BillingLatitude, BillingLongitude FROM Account]);
        List<Contact> listCon = [SELECT Id, Name, AccountId, is_Address_Override__c, Current_Address__c, MailingStreet, MailingCity, MailingState, MailingPostalCode, MailingCountry, MailingLatitude, MailingLongitude FROM Contact];
        System.assertEquals(totalAccountsToCreate * totalContactsToCreate, listCon.size());

        for (Contact con : listCon) {
            Account acc = mapAccIdAcc.get(con.AccountId);
            System.assertEquals(true, isMatchAddressAccCon(acc, con));
            System.assert(con.MailingStreet.contains('New Default Street'));
            System.assert(con.MailingCity.contains('New Default City'));
            System.assertEquals(false, con.is_Address_Override__c);
            System.assertNotEquals(null, con.Current_Address__c);
        }

        // verify the previous addresses got Default cleared.
        // and verify latest start date and latest end date appropriately set.
        List<Address__c> listAddr = [SELECT Id, Default_Address__c, MailingStreet__c, Household_Account__c, Latest_Start_Date__c, Latest_End_Date__c FROM Address__c];
        System.assertEquals(totalAccountsToCreate * 2, listAddr.size());
        for (Address__c addr : listAddr) {
            Boolean fNewDefault = (addr.MailingStreet__c.contains('New Default Street'));
            System.assertEquals(fNewDefault, addr.Default_Address__c);
            if (fNewDefault) {
                System.assertEquals(System.today(), addr.Latest_Start_Date__c);
                System.assertEquals(null, addr.Latest_End_Date__c);
            } else {
                System.assertEquals(System.today(), addr.Latest_End_Date__c);
            }
        }
    }

    /*********************************************************************************************************
    @description
        insert new default addresses to HH w/ existing default addresses, and then go back
    verify:
        contact's && hh address matches original default
        new default addresses no longer marked default
    **********************************************************************************************************/
    @IsTest
    private static void insertNewDefaultAddrAndReset() {

        // this creates a default Address for each HH
        createTestData(totalAccountsToCreate, totalContactsToCreate);
        List<Address__c> listAddrOriginal = testAddresses;

        // create additional addresses
        buildTestAddress(totalAccountsToCreate);
        for (Integer i = 0; i < totalAccountsToCreate; i++) {
            testAddresses[i].Household_Account__c = testAccounts[i].Id;
            testAddresses[i].Default_Address__c = true;
            testAddresses[i].MailingStreet__c = 'New Default Street' + i;
            testAddresses[i].MailingCity__c = 'New Default City' + i;
        }
        insert testAddresses;

        // go back to original default addresses
        // added this extra set to test fix where we didn't use the correct default
        for (Integer i = 0; i < totalAccountsToCreate; i++) {
            listAddrOriginal[i].Default_Address__c = true;
            listAddrOriginal[i].MailingStreet__c = 'Original Default Street' + i;
            listAddrOriginal[i].MailingCity__c = 'Original Default City' + i;
        }
        Test.startTest();
        update listAddrOriginal;
        Test.stopTest();

        // verify that the HH and Contacts share the same address and it's new!
        Map<Id, Account> mapAccIdAcc = new Map<Id, Account>([SELECT Id, Name, BillingStreet, BillingCity, BillingState, BillingPostalCode, BillingCountry, BillingLatitude, BillingLongitude FROM Account]);
        List<Contact> listCon = [SELECT Id, Name, AccountId, is_Address_Override__c, Current_Address__c, MailingStreet, MailingCity, MailingState, MailingPostalCode, MailingCountry, MailingLatitude, MailingLongitude FROM Contact];
        System.assertEquals(totalAccountsToCreate * totalContactsToCreate, listCon.size());

        for (Contact con : listCon) {
            Account acc = mapAccIdAcc.get(con.AccountId);
            System.assertEquals(true, isMatchAddressAccCon(acc, con));
            System.assert(con.MailingStreet.contains('Original Default Street'));
            System.assert(con.MailingCity.contains('Original Default City'));
            System.assertEquals(false, con.is_Address_Override__c);
            System.assertNotEquals(null, con.Current_Address__c);
        }

        // verify the previous addresses got Default cleared.
        // and verify latest start date and latest end date appropriately set.
        List<Address__c> listAddr = [SELECT Id, Default_Address__c, MailingStreet__c, Household_Account__c, Latest_Start_Date__c, Latest_End_Date__c FROM Address__c];
        System.assertEquals(totalAccountsToCreate * 2, listAddr.size());
        for (Address__c addr : listAddr) {
            Boolean fNewDefault = (addr.MailingStreet__c.contains('Original Default Street'));
            System.assertEquals(fNewDefault, addr.Default_Address__c);
            if (fNewDefault) {
                System.assertEquals(System.today(), addr.Latest_Start_Date__c);
                System.assertEquals(null, addr.Latest_End_Date__c);
            } else {
                System.assertEquals(System.today(), addr.Latest_End_Date__c);
            }
        }
    }

    /*********************************************************************************************************
    @description
        update contacts' Address Override
    verify:
        contacts' Mailing address fields updated to the Address Override's fields.
    **********************************************************************************************************/
    @IsTest
    private static void updateAddrOverride() {
        //skip the test if Advancement is installed
        if (ADV_PackageInfo_SVC.useAdv()) return;

        // this creates a default Address for each HH
        createTestData(totalAccountsToCreate, totalContactsToCreate);

        // create additional addresses
        buildTestAddress(totalAccountsToCreate);
        for (Integer i = 0; i < totalAccountsToCreate; i++) {
            testAddresses[i].Household_Account__c = testAccounts[i].Id;
            testAddresses[i].Default_Address__c = false;
            testAddresses[i].MailingStreet__c = 'override' + i;
            testAddresses[i].MailingCity__c = 'override' + i;
        }
        insert testAddresses;

        // set the contacts' address overrides
        List<Contact> listCon = new List<Contact>();
        for (Integer i = 0; i < totalAccountsToCreate; i++) {
            Contact con = testContacts[i * totalAccountsToCreate + i];
            con.Current_Address__c = testAddresses[i].Id;
            con.is_Address_Override__c = true;
            listCon.add(con);
        }
        Test.startTest();
        update listCon;
        Test.stopTest();

        // verify that the HH and Contacts share the same address
        Map<Id, Account> mapAccIdAcc = new Map<Id, Account>([SELECT Id, Name, BillingStreet, BillingCity, BillingState, BillingPostalCode, BillingCountry, BillingLatitude, BillingLongitude FROM Account]);
        listCon = [SELECT Id, Name, AccountId, is_Address_Override__c, Current_Address__c, MailingStreet, MailingCity, MailingState, MailingPostalCode, MailingCountry, MailingLatitude, MailingLongitude FROM Contact];

        for (Contact con : listCon) {
            Account acc = mapAccIdAcc.get(con.AccountId);
            if (con.is_Address_Override__c) {
                System.assertNotEquals(null, con.Current_Address__c);
                System.assertEquals(false, isMatchAddressAccCon(acc, con));
                System.assert(con.MailingStreet.contains('override'));
                System.assert(con.MailingCity.contains('override'));
            } else {
                System.assertEquals(true, isMatchAddressAccCon(acc, con));
                System.assert(con.MailingStreet.contains('Street'));
                System.assert(con.MailingCity.contains('City'));
                System.assertEquals(false, con.is_Address_Override__c);
                System.assertNotEquals(null, con.Current_Address__c);
            }
        }

        // verify address overrides have latest start date and latest end date appropriately set.
        List<Address__c> listAddr = [SELECT Id, Latest_Start_Date__c, Latest_End_Date__c FROM Address__c WHERE Id IN :testAddresses];
        System.assertEquals(totalAccountsToCreate, listAddr.size());
        for (Address__c addr : listAddr) {
            System.assertEquals(System.today(), addr.Latest_Start_Date__c);
            System.assertEquals(null, addr.Latest_End_Date__c);
        }
    }

    /*********************************************************************************************************
    @description
        clear contacts' Address Override
    verify:
        contacts' Mailing address fields updated to the HH default address
    **********************************************************************************************************/
    @IsTest
    private static void clearAddrOverride() {
        //skip the test if Advancement is installed
        if (ADV_PackageInfo_SVC.useAdv()) return;

        // this creates a default Address for each HH
        createTestData(totalAccountsToCreate, totalContactsToCreate);

        // create additional addresses
        buildTestAddress(totalAccountsToCreate);
        for (Integer i = 0; i < totalAccountsToCreate; i++) {
            testAddresses[i].Household_Account__c = testAccounts[i].Id;
            testAddresses[i].Default_Address__c = false;
            testAddresses[i].MailingStreet__c = 'override' + i;
            testAddresses[i].MailingCity__c = 'override' + i;
        }
        insert testAddresses;

        // set the contacts' address overrides
        List<Contact> listCon = new List<Contact>();
        for (Integer i = 0; i < totalAccountsToCreate; i++) {
            Contact con = testContacts[i * totalAccountsToCreate + i];
            con.Current_Address__c = testAddresses[i].Id;
            con.is_Address_Override__c = true;
            listCon.add(con);
        }
        update listCon;

        // now clear out the contacts' address overrides
        for (Contact con : listCon) {
            con.is_Address_Override__c = false;
        }
        Test.startTest();
        update listCon;
        Test.stopTest();

        // verify that the HH and Contacts share the same address
        Map<Id, Account> mapAccIdAcc = new Map<Id, Account>([SELECT Id, Name, BillingStreet, BillingCity, BillingState, BillingPostalCode, BillingCountry, BillingLatitude, BillingLongitude FROM Account]);
        listCon = [SELECT Id, Name, AccountId, is_Address_Override__c, Current_Address__c, MailingStreet, MailingCity, MailingState, MailingPostalCode, MailingCountry, MailingLatitude, MailingLongitude FROM Contact];

        for (Contact con : listCon) {
            Account acc = mapAccIdAcc.get(con.AccountId);
            System.assertEquals(false, con.is_Address_Override__c);
            System.assertNotEquals(null, con.Current_Address__c);
            System.assertEquals(true, isMatchAddressAccCon(acc, con));
            System.assert(con.MailingStreet.contains('Street'));
            System.assert(con.MailingCity.contains('City'));
        }

        // verify address overrides have latest start date and latest end date appropriately set.
        List<Address__c> listAddr = [SELECT Id, Latest_Start_Date__c, Latest_End_Date__c FROM Address__c WHERE Id IN :testAddresses];
        System.assertEquals(totalAccountsToCreate, listAddr.size());
        for (Address__c addr : listAddr) {
            System.assertEquals(System.today(), addr.Latest_Start_Date__c);
            System.assertEquals(System.today(), addr.Latest_End_Date__c);
        }
    }

    /*********************************************************************************************************
     @description
         clear contacts' Address Override and modify a new address
     verify:
         new default address created
     **********************************************************************************************************/
    @IsTest
    private static void clearAddrOverrideNewAddr() {
        //skip the test if Advancement is installed
        if (ADV_PackageInfo_SVC.useAdv()) return;

        // this creates a default Address for each HH
        createTestData(totalAccountsToCreate, totalContactsToCreate);

        // create additional addresses
        buildTestAddress(totalAccountsToCreate);
        for (Integer i = 0; i < totalAccountsToCreate; i++) {
            testAddresses[i].Household_Account__c = testAccounts[i].Id;
            testAddresses[i].Default_Address__c = false;
            testAddresses[i].MailingStreet__c = 'override' + i;
            testAddresses[i].MailingCity__c = 'override' + i;
        }
        insert testAddresses;

        // set the contacts' address overrides
        List<Contact> listCon = new List<Contact>();
        for (Integer i = 0; i < totalAccountsToCreate; i++) {
            Contact con = testContacts[i * totalAccountsToCreate + i];
            con.Current_Address__c = testAddresses[i].Id;
            con.is_Address_Override__c = true;
            listCon.add(con);
        }
        update listCon;

        // now clear out the contacts' address overrides,
        // but modify the address also!
        for (Contact con : listCon) {
            con.is_Address_Override__c = false;
            con.MailingStreet = 'New Street';
            con.MailingCity = 'New City';
        }
        Test.startTest();
        update listCon;
        Test.stopTest();

        // verify that the HH and Contacts share the same address
        Map<Id, Account> mapAccIdAcc = new Map<Id, Account>([SELECT Id, Name, BillingStreet, BillingCity, BillingState, BillingPostalCode, BillingCountry, BillingLatitude, BillingLongitude FROM Account]);
        listCon = [SELECT Id, Name, AccountId, is_Address_Override__c, Current_Address__c, MailingStreet, MailingCity, MailingState, MailingPostalCode, MailingCountry, MailingLatitude, MailingLongitude FROM Contact];

        for (Contact con : listCon) {
            Account acc = mapAccIdAcc.get(con.AccountId);
            System.assertEquals(false, con.is_Address_Override__c);
            System.assertNotEquals(null, con.Current_Address__c);
            System.assertEquals(true, isMatchAddressAccCon(acc, con));
            System.assert(con.MailingStreet.contains('Street'));
            System.assert(con.MailingCity.contains('City'));
        }

    }

    /*********************************************************************************************************
    @description
        update contacts' mailing address to cause creation of new default Address object
    verify:
        new Default Address created
        HH and Contacts get new Address
    **********************************************************************************************************/
    @IsTest
    private static void updateConAddrNew() {
        //skip the test if Advancement is installed
        if (ADV_PackageInfo_SVC.useAdv()) return;

        // this creates a default Address for each HH
        createTestData(totalAccountsToCreate, totalContactsToCreate);

        // modify some of the contact addresses directly
        List<Contact> listCon = new List<Contact>();
        for (Integer i = 0; i < totalAccountsToCreate; i++) {
            Contact con = testContacts[i * totalAccountsToCreate + i];
            con.MailingStreet = 'Direct Street Edit';
            con.MailingCity = 'Direct City Edit';
            listCon.add(con);
        }
        update listCon;

        for (Integer i = 0; i < totalAccountsToCreate; i++) {
            Contact con = testContacts[i * totalAccountsToCreate + i];
            con.MailingStreet = 'another Street Edit';
            con.MailingCity = 'another City Edit';
        }
        update listCon;

        for (Integer i = 0; i < totalAccountsToCreate; i++) {
            Contact con = testContacts[i * totalAccountsToCreate + i];
            con.MailingStreet = 'final Street Edit';
            con.MailingCity = 'final City Edit';
        }
        Test.startTest();
        update listCon;
        Test.stopTest();

        // verify that the HH and Contacts share the same address
        Map<Id, Account> mapAccIdAcc = new Map<Id, Account>([SELECT Id, Name, BillingStreet, BillingCity, BillingState, BillingPostalCode, BillingCountry, BillingLatitude, BillingLongitude FROM Account]);
        listCon = [SELECT Id, Name, AccountId, is_Address_Override__c, Current_Address__c, MailingStreet, MailingCity, MailingState, MailingPostalCode, MailingCountry, MailingLatitude, MailingLongitude FROM Contact];

        for (Contact con : listCon) {
            Account acc = mapAccIdAcc.get(con.AccountId);
            System.assertEquals(true, isMatchAddressAccCon(acc, con));
            System.assert(con.MailingStreet.contains('final Street Edit'));
            System.assert(con.MailingCity.contains('final City Edit'));
            System.assertEquals(false, con.is_Address_Override__c);
            System.assertNotEquals(null, con.Current_Address__c);
        }

        // verify the previous addresses got Default cleared.
        List<Address__c> listAddr = [SELECT Id, Default_Address__c, MailingStreet__c, Household_Account__c FROM Address__c];
        // each set of edits created a new address object
        System.assertEquals(totalAccountsToCreate * 4, listAddr.size());
        for (Address__c addr : listAddr) {
            Boolean fNewDefault = (addr.MailingStreet__c.contains('final Street Edit'));
            System.assertEquals(fNewDefault, addr.Default_Address__c);
        }
    }

    /*********************************************************************************************************
    @description
        update contacts' mailing address and mark it as override, to cause creation of new non-default Address object
    verify:
        new Address created
        HH sticks with default Address
        contact gets new address
    **********************************************************************************************************/
    @IsTest
    private static void updateConAddrMarkOverride() {
        //skip the test if Advancement is installed
        if (ADV_PackageInfo_SVC.useAdv()) return;

        // this creates a default Address for each HH
        createTestData(totalAccountsToCreate, totalContactsToCreate);

        // modify the contact addresses directly, and set override
        List<Contact> listCon = new List<Contact>();
        for (Integer i = 0; i < totalAccountsToCreate * totalContactsToCreate; i++) {
            Contact con = testContacts[i];
            con.MailingStreet = 'Direct Street Edit';
            con.MailingCity = 'Direct City Edit';
            con.is_Address_Override__c = true;
            listCon.add(con);
        }
        Test.startTest();
        update listCon;
        Test.stopTest();

        // verify that the HH and Contacts do not share the same address
        Map<Id, Account> mapAccIdAcc = new Map<Id, Account>([SELECT Id, Name, BillingStreet, BillingCity, BillingState, BillingPostalCode, BillingCountry, BillingLatitude, BillingLongitude FROM Account]);
        listCon = [SELECT Id, Name, AccountId, is_Address_Override__c, Current_Address__c, MailingStreet, MailingCity, MailingState, MailingPostalCode, MailingCountry, MailingLatitude, MailingLongitude FROM Contact];

        for (Contact con : listCon) {
            Account acc = mapAccIdAcc.get(con.AccountId);
            System.assertEquals(false, isMatchAddressAccCon(acc, con));
            System.assertEquals(true, con.MailingStreet.contains('Direct Street Edit'));
            System.assertEquals(true, con.MailingCity.contains('Direct City Edit'));
            System.assertEquals(false, acc.BillingStreet.contains('Direct Street Edit'));
            System.assertEquals(false, acc.BillingCity.contains('Direct City Edit'));
            System.assertEquals(true, con.is_Address_Override__c);
            System.assertNotEquals(null, con.Current_Address__c);
        }

        // verify the previous addresses got Default cleared.
        List<Address__c> listAddr = [SELECT Id, Default_Address__c, MailingStreet__c, Household_Account__c FROM Address__c];
        System.assertEquals(totalAccountsToCreate * totalContactsToCreate + totalAccountsToCreate, listAddr.size());
        for (Address__c addr : listAddr) {
            Boolean isNewEdit = (addr.MailingStreet__c.contains('Direct Street Edit'));
            System.assertNotEquals(isNewEdit, addr.Default_Address__c);
        }
    }

    /*********************************************************************************************************
    @description
        Update Contact's mailing address and mark it as override, to cause creation of a new non-default Address
        record with the Mailing State value set as expected when State & Country Picklists are enabled.
    verify:
        Contact gets new Address
        HH sticks with default Address
        New Address created with State value set as expected
    **********************************************************************************************************/
    @IsTest
    private static void testAddressOverrideCreatesNewAddressWhenStateCodeIsChanged() {
        if (!orgConfig.isStateCountryPicklistsEnabled()) {
            return;
        }

        Integer hhSize = 1;
        Integer contactSize = 1;
        String state = 'California';
        String stateCode = 'CA';

        // This creates a default Address for each HH
        createTestData(hhSize, contactSize);
        System.assertEquals(1, getAddresses().size());

        // Modify the Contact addresses directly, and set override
        Contact con = testContacts[0];
        con.put('MailingStateCode', stateCode);
        con.is_Address_Override__c = true;

        Test.startTest();
        update con;
        Test.stopTest();

        // Verify a new Address record was created
        List<Address__c> retrievedAddresses = getAddresses();
        System.assertEquals(2, retrievedAddresses.size());

        // Find the default Address
        Address__c defaultAddress = null;
        Address__c newAddress = null;
        for (Address__c eachAddress : retrievedAddresses) {
            if (eachAddress.Default_Address__c) {
                defaultAddress = eachAddress;
            } else {
                newAddress = eachAddress;
            }
        }

        // Verify the Account's Billing Address info matches the Default Address record
        assertEquals(retrievedAddresses[1], defaultAddress);

        // Verify the Contact's Address was updated as expected and that the Household and Contact Addresses are different.
        List<Account> retrievedAccounts = Database.query(buildAccountSoql());
        Map<Id, Account> accountIdToAccount = new Map<Id, Account>(retrievedAccounts);
        List<Contact> listCon = Database.query(buildContactSoql());

        System.assertEquals(state, listCon[0].MailingState);
        System.assertEquals(true, listCon[0].is_Address_Override__c);
        System.assertNotEquals(null, listCon[0].Current_Address__c);

        Account eachContactAccount = accountIdToAccount.get(listCon[0].AccountId);
        System.assertEquals(false, isMatchAddressAccCon(eachContactAccount, listCon[0]));

        // Verify the new Address Mailing State matches Contact's MailingStateCode and not set as the Default
        System.assertEquals(stateCode, newAddress.MailingState__c);
        System.assertEquals(false, newAddress.Default_Address__c);
    }

    /*********************************************************************************************************
    @description
        Update Address' Mailing Country when it is empty and State & Country Picklists are enabled.
    verify:
        Does not populate the Address' Mailing Country if the source sObject's CountryCode is empty
        Populates the Address' Mailing Country with the source sObject's CountryCode value if the Mailing Country is empty
    **********************************************************************************************************/
    @IsTest
    private static void testEmptyAddressCountryIsPopulatedFromCountryCode() {
        if (!orgConfig.isStateCountryPicklistsEnabled()) {
            return;
        }

        Contact con = new Contact();
        con.put('MailingCountryCode', null);
        Address__c addressEmptyCountry = new Address__c(MailingCountry__c = null);

        Test.startTest();

        String countryCode = 'US';
        String country = 'United States';

        NPSP_Address npspAddressEmptyCountry = new NPSP_Address(addressEmptyCountry);
        npspAddressEmptyCountry.populateAddressCountryFromCountryCode(con, 'Mailing');
        System.assertEquals(null, npspAddressEmptyCountry.country());

        con.put('MailingCountryCode', countryCode);
        npspAddressEmptyCountry.populateAddressCountryFromCountryCode(con, 'Mailing');
        System.assertEquals(country, npspAddressEmptyCountry.country());

        Test.stopTest();
    }


    /*********************************************************************************************************
    @description
        Update the Address' Mailing State when it is empty or does not match the source sObject's StateCode
        and State & Country Picklists are enabled.
    verify:
        Does not populate the Address' Mailing State if the source sObject's StateCode is empty
        Populate the Address' Mailing State with the source sObject's StateCode value if the Mailing State is empty
        Populate the Address' Mailing State with the source sObject's StateCode value if the Mailing State does not match the StateCode
    **********************************************************************************************************/
    @IsTest
    private static void testEmptyAddressStateIsPopulatedFromStateCode() {
        if (!orgConfig.isStateCountryPicklistsEnabled()) {
            return;
        }

        Contact con = new Contact();
        con.put('MailingStateCode', null);
        Address__c addressEmptyState = new Address__c(MailingState__c = null);

        Test.startTest();

        String stateCode = 'CA';

        NPSP_Address npspAddressEmptyState = new NPSP_Address(addressEmptyState);
        npspAddressEmptyState.populateAddressStateFromStateCode(con, 'Mailing');
        System.assertEquals(null, npspAddressEmptyState.state());

        con.put('MailingStateCode', stateCode);
        npspAddressEmptyState.populateAddressStateFromStateCode(con, 'Mailing');
        System.assertEquals(stateCode, npspAddressEmptyState.state());

        npspAddressEmptyState.state('NY');
        npspAddressEmptyState.populateAddressStateFromStateCode(con, 'Mailing');
        System.assertEquals(stateCode, npspAddressEmptyState.state());

        Test.stopTest();
    }


    /*********************************************************************************************************
    @description
        Checks if the Mailing State matches the source sObject's StateCode.
    verify:
        Returns false if the Address Mailing State is empty
        Returns false if the Address Mailing State does not match the StateCode
        Returns true if the Address Mailing State matches the StateCode
    **********************************************************************************************************/
    @IsTest
    private static void testIsMatchingStateCode() {
        if (!orgConfig.isStateCountryPicklistsEnabled()) {
            return;
        }

        Address__c addressMatch = new Address__c();

        Test.startTest();

        String stateCode = 'CA';

        NPSP_Address npspAddressMatch = new NPSP_Address(addressMatch);
        npspAddressMatch.state(null);
        System.assertEquals(false, npspAddressMatch.isMatchingStateCode(stateCode));

        npspAddressMatch.state('Hawaii');
        System.assertEquals(false, npspAddressMatch.isMatchingStateCode(stateCode));

        npspAddressMatch.state('California');
        System.assertEquals(true, npspAddressMatch.isMatchingStateCode(stateCode));

        Test.stopTest();
    }

    /*********************************************************************************************************
    @description
        update contacts' mailing address to cause an update to default Address object
    verify:
        existing Default Address updated
        HH and Contacts get updated Address
    **********************************************************************************************************/
    @IsTest
    private static void updateConAddrExisting() {
        //skip the test if Advancement is installed
        if (ADV_PackageInfo_SVC.useAdv()) return;

        npe01__Contacts_And_Orgs_Settings__c contactSettingsForTests = UTIL_CustomSettingsFacade.getContactsSettingsForTests(
            new npe01__Contacts_And_Orgs_Settings__c(
                npe01__Account_Processor__c = CAO_Constants.HH_ACCOUNT_PROCESSOR,
                Simple_Address_Change_Treated_as_Update__c = true
            ));

        // this creates a default Address for each HH
        createTestData(totalAccountsToCreate, totalContactsToCreate);

        // modify some of the contact addresses directly
        // NOTE: we only modify 1 field, so it will be treated as an update to an existing address!
        List<Contact> listCon = new List<Contact>();
        for (Integer i = 0; i < totalAccountsToCreate; i++) {
            Contact con = testContacts[i * totalAccountsToCreate + i];
            con.MailingStreet = 'Direct Street Edit';
            listCon.add(con);
        }
        update listCon;

        for (Integer i = 0; i < totalAccountsToCreate; i++) {
            Contact con = testContacts[i * totalAccountsToCreate + i];
            con.MailingState = 'Washington';
        }
        update listCon;

        for (Integer i = 0; i < totalAccountsToCreate; i++) {
            Contact con = testContacts[i * totalAccountsToCreate + i];
            con.MailingStreet = ' direct street  edit ';  // whitespace and casing should not count as a real edit.
            con.MailingState = 'Oregon';
        }
        Test.startTest();
        update listCon;

        Test.stopTest();

        // verify that the HH and Contacts share the same address
        Map<Id, Account> mapAccIdAcc = new Map<Id, Account>([SELECT Id, Name, BillingStreet, BillingCity, BillingState, BillingPostalCode, BillingCountry, BillingLatitude, BillingLongitude FROM Account]);
        listCon = [SELECT Id, Name, AccountId, is_Address_Override__c, Current_Address__c, MailingStreet, MailingCity, MailingState, MailingPostalCode, MailingCountry, MailingLatitude, MailingLongitude FROM Contact];

        for (Contact con : listCon) {
            Account acc = mapAccIdAcc.get(con.AccountId);
            System.assertEquals(true, isMatchAddressAccCon(acc, con));
            System.assert(con.MailingStreet.contains('direct street  edit'));
            System.assertEquals(false, con.is_Address_Override__c);
            System.assertNotEquals(null, con.Current_Address__c);
        }

        List<Address__c> listAddr = [
                SELECT Id,
                        Default_Address__c,
                        MailingStreet__c,
                        Household_Account__c
                FROM Address__c
        ];
        System.assertEquals(totalAccountsToCreate, listAddr.size(),
                'Whitespace and casing should not count as a real edit, so this test should' +
                        ' verify that no new address was created');
        for (Address__c addr : listAddr) {
            System.assertEquals('direct street  edit', addr.MailingStreet__c);
        }
    }

    /*********************************************************************************************************
    @description
        update contacts' address type to cause an update to default Address object
    verify:
        existing Default Address updated
        HH and Contacts get updated Address
    **********************************************************************************************************/
    @IsTest
    private static void updateConAddrExistingAddrType() {
        //skip the test if Advancement is installed
        if (ADV_PackageInfo_SVC.useAdv()) return;

        npe01__Contacts_And_Orgs_Settings__c contactSettingsForTests = UTIL_CustomSettingsFacade.getContactsSettingsForTests(
            new npe01__Contacts_And_Orgs_Settings__c(
                npe01__Account_Processor__c = CAO_Constants.HH_ACCOUNT_PROCESSOR,
                Simple_Address_Change_Treated_as_Update__c = true
            ));

        // this creates a default Address for each HH
        createTestData(totalAccountsToCreate, totalContactsToCreate);

        // modify some of the contact addresses directly
        // NOTE: we only modify 1 field, so it will be treated as an update to an existing address!
        List<Contact> listCon = new List<Contact>();
        for (Integer i = 0; i < totalAccountsToCreate; i++) {
            Contact con = testContacts[i * totalAccountsToCreate + i];
            con.npe01__Primary_Address_Type__c = 'Other';
            listCon.add(con);
        }
        update listCon;

        for (Integer i = 0; i < totalAccountsToCreate; i++) {
            Contact con = testContacts[i * totalAccountsToCreate + i];
            con.MailingStreet = ' direct street edit ';
        }
        update listCon;

        for (Integer i = 0; i < totalAccountsToCreate; i++) {
            Contact con = testContacts[i * totalAccountsToCreate + i];
            con.MailingState = 'Washington';
        }
        update listCon;

        for (Integer i = 0; i < totalAccountsToCreate; i++) {
            Contact con = testContacts[i * totalAccountsToCreate + i];
            con.MailingStreet = ' direct street  edit ';  // whitespace and casing should not count as a real edit.
            con.MailingState = 'Oregon';
        }
        Test.startTest();
        update listCon;

        Test.stopTest();

        // verify that the HH and Contacts share the same address
        Map<Id, Account> mapAccIdAcc = new Map<Id, Account>([SELECT Id, Name, BillingStreet, BillingCity, BillingState, BillingPostalCode, BillingCountry, BillingLatitude, BillingLongitude FROM Account]);
        listCon = [SELECT Id, Name, AccountId, is_Address_Override__c, Current_Address__c, npe01__Primary_Address_Type__c, MailingStreet, MailingCity, MailingState, MailingPostalCode, MailingCountry, MailingLatitude, MailingLongitude FROM Contact];

        for (Contact con : listCon) {
            Account acc = mapAccIdAcc.get(con.AccountId);
            System.assertEquals(true, isMatchAddressAccCon(acc, con));
            System.assert(con.MailingStreet.contains('direct street  edit'));
            System.assertEquals(false, con.is_Address_Override__c);
            System.assertNotEquals(null, con.Current_Address__c);
            System.assertEquals('Other', con.npe01__Primary_Address_Type__c);
        }

        // verify that no new address was created
        List<Address__c> listAddr = [SELECT Id, Default_Address__c, MailingStreet__c, Household_Account__c, Address_Type__c FROM Address__c];
        System.assertEquals(totalAccountsToCreate, listAddr.size());
        for (Address__c addr : listAddr) {
            System.assertEquals('direct street  edit', addr.MailingStreet__c);
            System.assertEquals('Other', addr.Address_Type__c);
        }
    }

    /*********************************************************************************************************
    @description
        Add an existing Contact to a Household.
        Pre-populate the Contact's address with the Household Address
        in the same way Manage Household pre-populates Contact's Mailing Address with Account's Billing Address.
    verify:
        The Contact's current Address is equal to the new Household Address.
        Contact's "Primary Address Type" is the same as the Address' "Address Type".
    **********************************************************************************************************/
    @IsTest
    private static void addExistingContactToNewHouseholdShouldSyncPrimaryAddressTypeWithAddressType() {
        //skip the test if Advancement is installed
        if (ADV_PackageInfo_SVC.useAdv()) return;

        UTIL_CustomSettingsFacade.getContactsSettingsForTests(
            new npe01__Contacts_And_Orgs_Settings__c (
                npe01__Account_Processor__c = CAO_Constants.HH_ACCOUNT_PROCESSOR,
                Simple_Address_Change_Treated_as_Update__c = true
            )
        );

        Integer hhSize = 1;
        Integer contactSize = 1;
        createTestData(hhSize, contactSize);

        Account account = new Account(Name = 'AccountWithoutAddress');
        insert account;
        Contact contact = new Contact(LastName = 'TestPrimaryAddressTypeExistingContact', AccountId = account.Id);
        insert contact;

        Test.startTest();

        contact.AccountId = testAccounts[0].Id;
        populateContactAddress(contact, testAddresses[0]);
        update contact;

        Test.stopTest();

        Map<Id, Address__c[]> addressesByHh = getAddressesByHousehold();
        System.assertEquals(hhSize, addressesByHh.size());

        for (Address__c addr : testAddresses) {
            Address__c[] addresses = addressesByHh.get(addr.Household_Account__c);
            System.assert(addresses != null && testAddresses.size() == 1, addresses);

            assertEquals(addr, addresses[0]);
        }

        Contact[] allContacts = getContacts();
        System.assertEquals(testContacts.size() + 1, allContacts.size());

        for (Contact c : allContacts) {
            System.assertEquals(testAccounts[0].Id, c.AccountId);

            Address__c[] addresses = addressesByHh.get(c.AccountId);
            System.assert(addresses != null && testAddresses.size() == 1, addresses);

            System.assertNotEquals(null, c.npe01__Primary_Address_Type__c);
            assertEqualAddresses(addresses[0], c);
        }
    }

    /*********************************************************************************************************
    @description
        Add a new Contact to a Household.
        Pre-populate the Contact's address with the Household Address
        in the same way standard Salesforce pre-populates Contact's Mailing Address with Account's Billing Address.
    verify:
        The Contact's current Address is equal to the new Household Address.
        Contact's "Primary Address Type" is the same as the Address' "Address Type".
    **********************************************************************************************************/
    @IsTest
    private static void addNewContactToHouseholdShouldSyncPrimaryAddressTypeWithAddressType() {
        //skip the test if Advancement is installed
        if (ADV_PackageInfo_SVC.useAdv()) return;

        UTIL_CustomSettingsFacade.getContactsSettingsForTests(
            new npe01__Contacts_And_Orgs_Settings__c (
                npe01__Account_Processor__c = CAO_Constants.HH_ACCOUNT_PROCESSOR,
                Simple_Address_Change_Treated_as_Update__c = true
            )
        );

        Integer hhSize = 1;
        Integer contactSize = 1;
        createTestData(hhSize, contactSize);

        Test.startTest();

        Contact contact = new Contact(LastName = 'TestPrimaryAddressTypeNewContact', AccountId = testAccounts[0].Id);
        populateContactAddress(contact, testAddresses[0]);
        insert contact;

        Test.stopTest();

        Map<Id, Address__c[]> addressesByHh = getAddressesByHousehold();
        System.assertEquals(hhSize, addressesByHh.size());

        for (Address__c addr : testAddresses) {
            Address__c[] addresses = addressesByHh.get(addr.Household_Account__c);
            System.assert(addresses != null && testAddresses.size() == 1, addresses);

            assertEquals(addr, addresses[0]);
        }

        Contact[] allContacts = getContacts();
        System.assertEquals(testContacts.size() + 1, allContacts.size());

        for (Contact c : allContacts) {
            Address__c[] addresses = addressesByHh.get(c.AccountId);
            System.assert(addresses != null && testAddresses.size() == 1, addresses);

            System.assertNotEquals(null, c.npe01__Primary_Address_Type__c);
            assertEqualAddresses(addresses[0], c);
        }
    }

    /*********************************************************************************************************
    @description
        update contacts' mailing address with just a case change, to cause an update to default Address object
    verify:
        existing Default Address updated
        HH and Contacts get updated Address
    **********************************************************************************************************/
    @IsTest
    private static void updateConAddrExistingCaseOnly() {
        //skip the test if Advancement is installed
        if (ADV_PackageInfo_SVC.useAdv()) return;

        npe01__Contacts_And_Orgs_Settings__c contactSettingsForTests = UTIL_CustomSettingsFacade.getContactsSettingsForTests(
            new npe01__Contacts_And_Orgs_Settings__c(
                npe01__Account_Processor__c = CAO_Constants.HH_ACCOUNT_PROCESSOR,
                Simple_Address_Change_Treated_as_Update__c = true
            ));

        // this creates a default Address for each HH
        createTestData(totalAccountsToCreate, totalContactsToCreate);

        // modify some of the contact addresses directly
        // NOTE: we only modify 1 field, so it will be treated as an update to an existing address!
        List<Contact> listCon = new List<Contact>();
        for (Integer i = 0; i < totalAccountsToCreate; i++) {
            Contact con = testContacts[i * totalAccountsToCreate + i];
            con.MailingStreet = 'Direct Street Edit';
            listCon.add(con);
        }
        update listCon;

        for (Integer i = 0; i < totalAccountsToCreate; i++) {
            Contact con = testContacts[i * totalAccountsToCreate + i];
            con.MailingState = 'Washington';
        }
        update listCon;

        for (Integer i = 0; i < totalAccountsToCreate; i++) {
            Contact con = testContacts[i * totalAccountsToCreate + i];
            con.MailingStreet = ' direct street  edit ';  // whitespace and casing should not count as a real edit.
            con.MailingState = 'Oregon';
        }
        update listCon;

        for (Integer i = 0; i < totalAccountsToCreate; i++) {
            Contact con = testContacts[i * totalAccountsToCreate + i];
            con.MailingStreet = ' DIRECT STREET  EDIT ';  // only change casing.
        }
        update listCon;

        // verify that the HH and Contacts share the same address
        Map<Id, Account> mapAccIdAcc = new Map<Id, Account>([SELECT Id, Name, BillingStreet, BillingCity, BillingState, BillingPostalCode, BillingCountry, BillingLatitude, BillingLongitude FROM Account]);
        listCon = [SELECT Id, Name, AccountId, is_Address_Override__c, Current_Address__c, MailingStreet, MailingCity, MailingState, MailingPostalCode, MailingCountry, MailingLatitude, MailingLongitude FROM Contact];

        for (Contact con : listCon) {
            Account acc = mapAccIdAcc.get(con.AccountId);
            System.assertEquals(true, isMatchAddressAccCon(acc, con));
            System.assert(con.MailingStreet.equals('DIRECT STREET  EDIT'));
            System.assertEquals(false, con.is_Address_Override__c);
            System.assertNotEquals(null, con.Current_Address__c);
        }

        // verify that no new address was created
        List<Address__c> listAddr = [SELECT Id, Default_Address__c, MailingStreet__c, Household_Account__c FROM Address__c];
        System.assertEquals(totalAccountsToCreate, listAddr.size());
        for (Address__c addr : listAddr) {
            System.assert(addr.MailingStreet__c.equals('DIRECT STREET  EDIT'));
        }

        // now test that changing the case from the account updates all addresses
        for (Integer i = 0; i < totalAccountsToCreate; i++) {
            Account acc = testAccounts[i];
            acc.BillingStreet = 'direct street  edit';
        }
        Test.startTest();
        update testAccounts;
        Test.stopTest();

        // verify that the HH and Contacts share the same address
        mapAccIdAcc = new Map<Id, Account>([SELECT Id, Name, BillingStreet, BillingCity, BillingState, BillingPostalCode, BillingCountry, BillingLatitude, BillingLongitude FROM Account]);
        listCon = [SELECT Id, Name, AccountId, is_Address_Override__c, Current_Address__c, MailingStreet, MailingCity, MailingState, MailingPostalCode, MailingCountry, MailingLatitude, MailingLongitude FROM Contact];

        for (Contact con : listCon) {
            Account acc = mapAccIdAcc.get(con.AccountId);
            System.assertEquals(true, isMatchAddressAccCon(acc, con));
            System.assert(con.MailingStreet.equals('direct street  edit'));
            System.assertEquals(false, con.is_Address_Override__c);
            System.assertNotEquals(null, con.Current_Address__c);
        }

        // verify that no new address was created
        listAddr = [SELECT Id, Default_Address__c, MailingStreet__c, Household_Account__c FROM Address__c];
        System.assertEquals(totalAccountsToCreate, listAddr.size());
        for (Address__c addr : listAddr) {
            System.assert(addr.MailingStreet__c.equals('direct street  edit'));
        }


    }

    /*********************************************************************************************************
    @description
        update accounts' billing address to cause creation of new default Address object
    verify:
        new Default Address created
        HH and Contacts get new Address
    **********************************************************************************************************/
    @IsTest
    private static void updateAccAddrNew() {
        //skip the test if Advancement is installed
        if (ADV_PackageInfo_SVC.useAdv()) return;

        // this creates a default Address for each HH
        createTestData(totalAccountsToCreate, totalContactsToCreate);

        // modify some of the account addresses directly
        for (Integer i = 0; i < totalAccountsToCreate; i++) {
            Account acc = testAccounts[i];
            acc.BillingStreet = 'Direct Street Edit';
            acc.BillingCity = 'Direct City Edit';
        }
        update testAccounts;

        for (Integer i = 0; i < totalAccountsToCreate; i++) {
            Account acc = testAccounts[i];
            acc.BillingStreet = 'another Street Edit';
            acc.BillingCity = 'another City Edit';
        }
        update testAccounts;

        for (Integer i = 0; i < totalAccountsToCreate; i++) {
            Account acc = testAccounts[i];
            acc.BillingStreet = 'final Street Edit';
            acc.BillingCity = 'final City Edit';
        }
        Test.startTest();
        update testAccounts;
        Test.stopTest();

        // verify that the HH and Contacts share the same address
        Map<Id, Account> mapAccIdAcc = new Map<Id, Account>([SELECT Id, Name, BillingStreet, BillingCity, BillingState, BillingPostalCode, BillingCountry, BillingLatitude, BillingLongitude FROM Account]);
        List<Contact> listCon = [SELECT Id, Name, AccountId, is_Address_Override__c, Current_Address__c, MailingStreet, MailingCity, MailingState, MailingPostalCode, MailingCountry, MailingLatitude, MailingLongitude FROM Contact];

        for (Contact con : listCon) {
            Account acc = mapAccIdAcc.get(con.AccountId);
            System.assertEquals(true, isMatchAddressAccCon(acc, con));
            System.assert(acc.BillingStreet.contains('final Street Edit'));
            System.assert(con.MailingStreet.contains('final Street Edit'));
            System.assertEquals(false, con.is_Address_Override__c);
            System.assertNotEquals(null, con.Current_Address__c);
        }

        // verify the previous addresses got Default cleared.
        List<Address__c> listAddr = [SELECT Id, Default_Address__c, MailingStreet__c, Household_Account__c FROM Address__c];
        // each set of edits created a new address object
        System.assertEquals(totalAccountsToCreate * 4, listAddr.size());
        for (Address__c addr : listAddr) {
            Boolean fNewDefault = (addr.MailingStreet__c.contains('final Street Edit'));
            System.assertEquals(fNewDefault, addr.Default_Address__c);
        }
    }

    /*********************************************************************************************************
    @description
        update accounts' billing address to cause an update of the default Address object
    verify:
        existing Default Address updated
        HH and Contacts get updated Address
    **********************************************************************************************************/
    @IsTest
    private static void updateAccAddrExisting() {
        //skip the test if Advancement is installed
        if (ADV_PackageInfo_SVC.useAdv()) return;

        npe01__Contacts_And_Orgs_Settings__c contactSettingsForTests = UTIL_CustomSettingsFacade.getContactsSettingsForTests(
            new npe01__Contacts_And_Orgs_Settings__c(
                npe01__Account_Processor__c = CAO_Constants.HH_ACCOUNT_PROCESSOR,
                Simple_Address_Change_Treated_as_Update__c = true
            ));

        // this creates a default Address for each HH
        createTestData(totalAccountsToCreate, totalContactsToCreate);

        // modify some of the account addresses directly
        // NOTE: we only modify 1 field, so it will be treated as an update to an existing address!
        for (Integer i = 0; i < totalAccountsToCreate; i++) {
            Account acc = testAccounts[i];
            acc.BillingStreet = 'Direct Street Edit';
        }
        update testAccounts;

        for (Integer i = 0; i < totalAccountsToCreate; i++) {
            Account acc = testAccounts[i];
            acc.BillingState = 'Washington';
        }
        update testAccounts;

        for (Integer i = 0; i < totalAccountsToCreate; i++) {
            Account acc = testAccounts[i];
            acc.BillingStreet = ' direct street  edit ';    // whitespace and casing should not count as real edit.
            acc.BillingState = 'Oregon';
        }
        Test.startTest();
        update testAccounts;
        Test.stopTest();

        // verify that the HH and Contacts share the same address
        Map<Id, Account> mapAccIdAcc = new Map<Id, Account>([SELECT Id, Name, BillingStreet, BillingCity, BillingState, BillingPostalCode, BillingCountry, BillingLatitude, BillingLongitude FROM Account]);
        List<Contact> listCon = [SELECT Id, Name, AccountId, is_Address_Override__c, Current_Address__c, MailingStreet, MailingCity, MailingState, MailingPostalCode, MailingCountry, MailingLatitude, MailingLongitude FROM Contact];

        for (Contact con : listCon) {
            Account acc = mapAccIdAcc.get(con.AccountId);
            System.assertEquals(true, isMatchAddressAccCon(acc, con));
            System.assert(acc.BillingStreet.contains('direct street  edit'));
            System.assert(con.MailingStreet.contains('direct street  edit'));
            System.assertEquals(false, con.is_Address_Override__c);
            System.assertNotEquals(null, con.Current_Address__c);
        }

        // verify that no new address was created
        List<Address__c> listAddr = [SELECT Id, Default_Address__c, MailingStreet__c, Household_Account__c FROM Address__c];
        System.assertEquals(totalAccountsToCreate, listAddr.size());
        for (Address__c addr : listAddr) {
            System.assertEquals('direct street  edit', addr.MailingStreet__c);
        }
    }

    /*********************************************************************************************************
    @description
        create an Address with no Account
    verify:
        runtime error
    **********************************************************************************************************/
    @IsTest
    private static void newAddrNoAccount() {
        buildTestAddress(totalAccountsToCreate);
        for (Integer i = 0; i < totalAccountsToCreate; i++) {
            System.assertEquals(null, testAddresses[i].Household_Account__c);
        }

        try {
            insert testAddresses;
        } catch (Exception ex) {
            System.assert(ex.getMessage().contains(Label.addrHHAccountOnly));
            return;
        }
        System.assert(false); //we shouldn't get here!
    }


    /*********************************************************************************************************
    @description
        create an Address with a NON-Household Account
    verify:
        runtime error
    **********************************************************************************************************/
    @IsTest
    private static void newAddrNotHHAccount() {
        testAccounts = UTIL_UnitTestData_TEST.createMultipleTestAccounts(totalAccountsToCreate, CAO_Constants.ONE_TO_ONE_ORGANIZATION_TYPE);
        insert testAccounts;

        buildTestAddress(totalAccountsToCreate);
        for (Integer i = 0; i < totalAccountsToCreate; i++) {
            System.assertEquals(null, testAddresses[i].Household_Account__c);
            testAddresses[i].Household_Account__c = testAccounts[i].Id;
        }

        try {
            Test.startTest();
            insert testAddresses;
            Test.stopTest();
        } catch (Exception ex) {
            System.assert(ex.getMessage().contains(Label.addrHHAccountOnly));
        }
        Integer insertedAddrCount = [SELECT COUNT() FROM Address__c];
        System.assertEquals(0, insertedAddrCount);

        /* We cannot verify error record was created because the usage of addError causes the whole transaction
        to be rolled back.
        List<Error__c> errors = [select Id, Full_Message__c, Stack_Trace__c, Email_Sent__c from Error__c];
        UTIL_Debug.debug('****Errors: ' + JSON.serializePretty(errors));
        System.assertEquals(1, errors.size());*/
    }

    /*********************************************************************************************************
    @description
        insert new seasonal addresses to HH w/ existing default addresses
    verify:
        contact's && hh address matches new seasonal
        old default addresses still marked default
    **********************************************************************************************************/
    @IsTest
    private static void insertNewSeasonalAddr() {
        // this creates a default Address for each HH
        createTestData(totalAccountsToCreate, totalContactsToCreate);

        // create additional addresses
        buildTestAddress(totalAccountsToCreate);
        for (Integer i = 0; i < totalAccountsToCreate; i++) {
            testAddresses[i].Household_Account__c = testAccounts[i].Id;
            testAddresses[i].Default_Address__c = false;
            testAddresses[i].Seasonal_Start_Month__c = String.valueOf(System.today().addMonths(-1).month());
            testAddresses[i].Seasonal_Start_Day__c = '1';
            testAddresses[i].Seasonal_End_Month__c = String.valueOf(System.today().addMonths(1).month());
            testAddresses[i].Seasonal_End_Day__c = '28';
            testAddresses[i].MailingStreet__c = 'New Seasonal Street' + i;
            testAddresses[i].MailingCity__c = 'New Seasonal City' + i;
        }
        Test.startTest();
        insert testAddresses;
        Test.stopTest();

        // verify that the HH and Contacts share the same address and it's new!
        Map<Id, Account> mapAccIdAcc = new Map<Id, Account>([SELECT Id, Name, BillingStreet, BillingCity, BillingState, BillingPostalCode, BillingCountry, BillingLatitude, BillingLongitude FROM Account]);
        List<Contact> listCon = [SELECT Id, Name, AccountId, is_Address_Override__c, Current_Address__c, MailingStreet, MailingCity, MailingState, MailingPostalCode, MailingCountry, MailingLatitude, MailingLongitude FROM Contact];
        System.assertEquals(totalAccountsToCreate * totalContactsToCreate, listCon.size());

        for (Contact con : listCon) {
            Account acc = mapAccIdAcc.get(con.AccountId);
            System.assertEquals(true, isMatchAddressAccCon(acc, con));
            System.assert(con.MailingStreet.contains('New Seasonal Street'));
            System.assert(con.MailingCity.contains('New Seasonal City'));
            System.assertEquals(false, con.is_Address_Override__c);
            System.assertNotEquals(null, con.Current_Address__c);
        }

        // verify the previous addresses still are Default
        List<Address__c> listAddr = [SELECT Id, Default_Address__c, MailingStreet__c, Household_Account__c, Latest_Start_Date__c, Latest_End_Date__c FROM Address__c];
        System.assertEquals(totalAccountsToCreate * 2, listAddr.size());
        for (Address__c addr : listAddr) {
            Boolean fSeasonal = (addr.MailingStreet__c.contains('New Seasonal Street'));
            System.assertEquals(!fSeasonal, addr.Default_Address__c);
        }
    }

    /*********************************************************************************************************
    @description
        insert new seasonal addresses, not current, to HH w/ existing default addresses
    verify:
        contact's && hh address stay with default
        old default addresses still marked default
    **********************************************************************************************************/
    @IsTest
    private static void insertNewSeasonalAddrNotCurrent() {
        // this creates a default Address for each HH
        createTestData(totalAccountsToCreate, totalContactsToCreate);

        // create additional addresses
        buildTestAddress(totalAccountsToCreate);
        for (Integer i = 0; i < totalAccountsToCreate; i++) {
            testAddresses[i].Household_Account__c = testAccounts[i].Id;
            testAddresses[i].Default_Address__c = false;
            testAddresses[i].Seasonal_Start_Month__c = String.valueOf(System.today().addMonths(1).month());
            testAddresses[i].Seasonal_Start_Day__c = '1';
            testAddresses[i].Seasonal_End_Month__c = String.valueOf(System.today().addMonths(2).month());
            testAddresses[i].Seasonal_End_Day__c = '28';
            testAddresses[i].MailingStreet__c = 'New Seasonal Street' + i;
            testAddresses[i].MailingCity__c = 'New Seasonal City' + i;
        }
        Test.startTest();
        insert testAddresses;
        Test.stopTest();

        // verify that the HH and Contacts share the same address and it's new!
        Map<Id, Account> mapAccIdAcc = new Map<Id, Account>([SELECT Id, Name, BillingStreet, BillingCity, BillingState, BillingPostalCode, BillingCountry, BillingLatitude, BillingLongitude FROM Account]);
        List<Contact> listCon = [SELECT Id, Name, AccountId, is_Address_Override__c, Current_Address__c, MailingStreet, MailingCity, MailingState, MailingPostalCode, MailingCountry, MailingLatitude, MailingLongitude FROM Contact];
        System.assertEquals(totalAccountsToCreate * totalContactsToCreate, listCon.size());

        for (Contact con : listCon) {
            Account acc = mapAccIdAcc.get(con.AccountId);
            System.assertEquals(true, isMatchAddressAccCon(acc, con));
            System.assert(!con.MailingStreet.contains('New Seasonal Street'));
            System.assert(!con.MailingCity.contains('New Seasonal City'));
            System.assertEquals(false, con.is_Address_Override__c);
            System.assertNotEquals(null, con.Current_Address__c);
        }

        // verify the previous addresses still are Default
        List<Address__c> listAddr = [SELECT Id, Default_Address__c, MailingStreet__c, Household_Account__c, Latest_Start_Date__c, Latest_End_Date__c FROM Address__c];
        System.assertEquals(totalAccountsToCreate * 2, listAddr.size());
        for (Address__c addr : listAddr) {
            Boolean fSeasonal = (addr.MailingStreet__c.contains('New Seasonal Street'));
            System.assertEquals(!fSeasonal, addr.Default_Address__c);
        }
    }

    /*********************************************************************************************************
    @description
        Insert more than 200 addresses to require multiple trigger calls, and to ensure our static counters
        are handled appropriately.
    verify:
        Addresses are propagated to all contacts and accounts
    **********************************************************************************************************/
    @IsTest
    private static void addressShouldBePropagatedToAccountsAndContactsOnMultipleTriggerCalls() {
        Integer totalHH = 205;
        Integer totalContactPerHH = 1;
        UTIL_Debug.disableDebugLogging();
        UTIL_UnitTestData_TEST.disableAffiliationsTriggers();
        UTIL_UnitTestData_TEST.disableRelationshipTriggers();
        UTIL_UnitTestData_TEST.disableMergeTriggers();
        UTIL_UnitTestData_TEST.disableHHNamingTriggers();
        createAccountAndContactData(totalHH, totalContactPerHH);

        // Act
        Test.startTest();
        createAddressData(totalHH, false);
        Test.stopTest();

        // Assert
        // verify that the HH and Contacts share the same address
        Map<Id, Account> accountById = new Map<Id, Account>([
            SELECT Id, Name, BillingStreet, BillingCity,
                BillingState, BillingPostalCode, BillingCountry,
                BillingLatitude, BillingLongitude
            FROM Account]);
        List<Contact> contacts = getContacts();
        System.assertEquals(totalHH * totalContactPerHH, contacts.size());

        for (Contact con : contacts) {
            Account acc = accountById.get(con.AccountId);

            // The assertions validate:
            // 1. Address on Contact matches address on Account
            // 2. Contact isAddressOverride is false
            // 3. Contact current address is populated
            System.assertEquals(true, isMatchAddressAccCon(acc, con));
            System.assertEquals(false, con.is_Address_Override__c);
            System.assertNotEquals(null, con.Current_Address__c);
        }
    }

    @IsTest
    private static void latestStartDateShouldDefaultToTodayForNewDefaultAddress() {
        if (ADV_PackageInfo_SVC.useAdv()) {
            return;
        }

        Contact testContact = UTIL_UnitTestData_TEST.getContact();
        buildTestAddress(1);
        populateContactAddress(testContact, testAddresses[0]);

        Test.startTest();
        insert testContact;
        Test.stopTest();

        Address__c testAddress;
        for (Address__c addr : [SELECT Id, MailingState__c, Latest_Start_Date__c, Default_Address__c FROM Address__c]) {
            if (addr.MailingState__c == 'Washington') {
                testAddress = addr;
                break;
            }
        }

        System.assertEquals(true, testAddress.Default_Address__c, 'Address should be set as the default address.');
        System.assertEquals(System.today(), testAddress.Latest_Start_Date__c, 'The default address latest start date ' +
                'should be set to today\'s date.');
    }

    @IsTest
    private static void latestStartDateOnUpdatedDefaultAddressSetToToday() {
        if (ADV_PackageInfo_SVC.useAdv()) {
            return;
        }
        buildTestAddress(2);
        NPSP_Contact npspContact = new NPSP_Contact(UTIL_UnitTestData_TEST.getContact());
        npspContact.mailingAddress(new NPSP_Address(testAddresses[0]));

        insert npspContact.getRecord();

        Contact testContact = [SELECT Current_Address__c, Current_Address__r.Default_Address__c, AccountId
                               FROM Contact
                               WHERE Id = :npspContact.getRecord().Id
                               LIMIT 1];
        Id defaultAddressId = testContact.Current_Address__c;
        System.assertEquals(true, testContact.Current_Address__r.Default_Address__c, 'The address should be the default address of ' +
                'the household.');

        testAddresses[1].Household_Account__c = testContact.AccountId;
        insert testAddresses[1];

        Address__c nonDefaultAddress = [SELECT Default_Address__c FROM Address__c WHERE Id = :testAddresses[1].Id];

        Test.startTest();

        nonDefaultAddress.Default_Address__c = true;
        update nonDefaultAddress;

        Test.stopTest();

        Address__c newDefaultAddress = [SELECT Default_Address__c, Latest_Start_Date__c, Latest_End_Date__c
                                        FROM Address__c
                                        WHERE Id = :nonDefaultAddress.Id
                                        LIMIT 1];
        Address__c previousDefaultAddress = [SELECT Default_Address__c, Latest_Start_Date__c, Latest_End_Date__c
                                             FROM Address__c
                                             WHERE Id = :defaultAddressId
                                             LIMIT 1];
        System.assertEquals(false, previousDefaultAddress.Default_Address__c, 'The previous default address should be' +
                ' set to false.');
        System.assertEquals(true, newDefaultAddress.Default_Address__c, 'The new default address should be' +
                ' set to true.');
        System.assertEquals(System.today(), previousDefaultAddress.Latest_End_Date__c, 'The latest end date on the ' +
                'previous default address should be set to today.');
        System.assertEquals(System.today(), previousDefaultAddress.Latest_Start_Date__c, 'The latest start date on ' +
                'the previous default address should be set to today and remain unchanged.');
        System.assertEquals(System.today(), newDefaultAddress.Latest_Start_Date__c, 'The latest start date on the ' +
                'new default address should be set to today.');
        System.assertEquals(null, newDefaultAddress.Latest_End_Date__c, 'The latest end date on the ' +
                'new default address should be set to null.');
    }

    @IsTest
    private static void latestEndDateOnUpdatedNonDefaultAddressSetToToday() {
        if (ADV_PackageInfo_SVC.useAdv()) {
            return;
        }
        buildTestAddress(1);
        NPSP_Contact npspContact = new NPSP_Contact(UTIL_UnitTestData_TEST.getContact());
        npspContact.mailingAddress(new NPSP_Address(testAddresses[0]));

        insert npspContact.getRecord();

        Contact testContact = [SELECT Current_Address__c, Current_Address__r.Default_Address__c, AccountId
                               FROM Contact
                               WHERE Id = :npspContact.getRecord().Id
                               LIMIT 1];
        System.assertEquals(true, testContact.Current_Address__r.Default_Address__c, 'The address should be the default address of ' +
                'the household.');

        Address__c defaultAddress = [SELECT Default_Address__c
                                     FROM Address__c
                                     WHERE Id = :testContact.Current_Address__c];

        Test.startTest();

        defaultAddress.Default_Address__c = false;
        update defaultAddress;

        Test.stopTest();

        Address__c updatedDefaultAddress = [SELECT Default_Address__c, Latest_Start_Date__c, Latest_End_Date__c
                                            FROM Address__c
                                            WHERE Id = :defaultAddress.Id
                                            LIMIT 1];
        System.assertEquals(false, updatedDefaultAddress.Default_Address__c, 'The previous default address should be' +
                ' set to false.');
        System.assertEquals(System.today(), updatedDefaultAddress.Latest_End_Date__c, 'The latest end date on the ' +
                'previous default address should be set to today.');
    }

    @IsTest
    private static void newNonDefaultAddressHasNullStartAndEndDate() {
        if (ADV_PackageInfo_SVC.useAdv()) {
            return;
        }
        buildTestAddress(2);
        NPSP_Contact npspContact = new NPSP_Contact(UTIL_UnitTestData_TEST.getContact());
        npspContact.mailingAddress(new NPSP_Address(testAddresses[0]));

        insert npspContact.getRecord();

        Contact testContact = [SELECT Current_Address__c, Current_Address__r.Default_Address__c, AccountId
                               FROM Contact
                               WHERE Id = :npspContact.getRecord().Id
                               LIMIT 1];
        System.assertEquals(true, testContact.Current_Address__r.Default_Address__c, 'The address should be the default address of ' +
                'the household.');

        testAddresses[1].Household_Account__c = testContact.AccountId;

        Test.startTest();

        insert testAddresses[1];

        Test.stopTest();

        Address__c updatedTestAddress = [SELECT Default_Address__c, Latest_Start_Date__c, Latest_End_Date__c
                                            FROM Address__c
                                            WHERE Id = :testAddresses[1].Id
                                            LIMIT 1];
        System.assertEquals(false, updatedTestAddress.Default_Address__c, 'The previous default address should be' +
                ' set to false.');
        System.assertEquals(null, updatedTestAddress.Latest_End_Date__c, 'The latest end date on the ' +
                'new address should be null.');
        System.assertEquals(null, updatedTestAddress.Latest_Start_Date__c, 'The latest start date on the ' +
                'new address should be null.');
    }

    @IsTest
    private static void updatedNonDefaultStartAndEndDateIsRetained() {
        if (ADV_PackageInfo_SVC.useAdv()) {
            return;
        }
        buildTestAddress(2);
        NPSP_Contact npspContact = new NPSP_Contact(UTIL_UnitTestData_TEST.getContact());
        npspContact.mailingAddress(new NPSP_Address(testAddresses[0]));

        insert npspContact.getRecord();

        Contact testContact = [SELECT AccountId FROM Contact LIMIT 1];

        testAddresses[1].Household_Account__c = testContact.AccountId;
        insert testAddresses[1];

        Address__c testAddress = [SELECT Latest_Start_Date__c, Latest_End_Date__c
                                  FROM Address__c
                                  WHERE Id = :testAddresses[1].Id
                                  LIMIT 1];
        Test.startTest();

        testAddress.Latest_Start_Date__c = System.today() - 2;
        testAddress.Latest_End_Date__c = System.today() - 1;

        update testAddress;

        Test.stopTest();

        Address__c updatedTestAddress = [SELECT Default_Address__c, Latest_Start_Date__c, Latest_End_Date__c
                                         FROM Address__c
                                         WHERE Id = :testAddresses[1].Id
                                         LIMIT 1];
        System.assertEquals(false, updatedTestAddress.Default_Address__c, 'The previous default address should be' +
                ' set to false.');
        System.assertEquals(System.today() - 1, updatedTestAddress.Latest_End_Date__c, 'The latest end date on the ' +
                'non-default address should not have changed.');
        System.assertEquals(System.today() - 2, updatedTestAddress.Latest_Start_Date__c, 'The latest start date on the ' +
                'non-default address should not have changed.');
    }

    // Helpers
    ////////////


    /*********************************************************************************************************
    * @description
    * utility to set up common test data for the Address tests.
    *    - this will create & insert N household Accounts, each with M contacts
    *    - also will create a Default Addresses for each HH Account
    *    - will also setup the default Account Processor to be HH Accounts
    * @param cHHT the number of Household Accounts to create
    * @param cConT the number of Contacts to create per Household
    **********************************************************************************************************/
    public static void createTestData(Integer cHHT, Integer cConT) {

        // we are creating a bunch of test data that relies on no async behavior.
        TDTM_DefaultConfig.startTestDataInit();

        createAccountAndContactData(cHHT, cConT);
        createAddressData(cHHT, false);

        // restore async behavior as appropriate
        TDTM_DefaultConfig.stopTestDataInit();
    }

    public static void createTestData(Integer cHHT, Integer cConT, Boolean isUndeliverableAddress) {

        // we are creating a bunch of test data that relies on no async behavior.
        TDTM_DefaultConfig.startTestDataInit();

        createAccountAndContactData(cHHT, cConT);
        createAddressData(cHHT, isUndeliverableAddress);

        // restore async behavior as appropriate
        TDTM_DefaultConfig.stopTestDataInit();
    }


    private static void createAddressData(Integer cHHT, Boolean isUndeliverableAddress) {
        buildTestAddress(cHHT);
        for (Integer i = 0; i < cHHT; i++) {
            testAddresses[i].Household_Account__c = testAccounts[i].Id;
            testAddresses[i].Default_Address__c = true;
            testAddresses[i].Undeliverable__c = isUndeliverableAddress;
        }
        insert testAddresses;
    }

    private static void createAccountAndContactData(Integer cHHT, Integer cConT) {
        testAccounts = UTIL_UnitTestData_TEST.createMultipleTestAccounts(cHHT, CAO_Constants.HH_ACCOUNT_TYPE);
        insert testAccounts;

        testContacts = UTIL_UnitTestData_TEST.createMultipleTestContacts(cHHT * cConT);
        for (Integer i = 0; i < cHHT; i++) {
            for (Integer j = 0; j < cConT; j++) {
                testContacts[(i * cConT) + j].AccountId = testAccounts[i].Id;
                testContacts[(i * cConT) + j].LastName = testContacts[(i * cConT) + j].LastName + j;
                testContacts[(i * cConT) + j].OtherCity = null;
            }
        }
        insert testContacts;
    }

    /*********************************************************************************************************
    * @description Initializes (but not inserts) N new Address objects.
    * @param totalAddressesToBuild the number of Address objects to build
    **********************************************************************************************************/
    public static void buildTestAddress(Integer totalAddressesToBuild) {
        testAddresses = new List<Address__c>();
        for (Integer i = 0; i < totalAddressesToBuild; i++) {
            Address__c addr = addressDefaults(i);
            testAddresses.add(addr);
        }
    }

    public static void buildTestAddressUndeliverable(Integer totalAddressesToBuild) {
        testAddresses = new List<Address__c>();
        for (Integer i = 0; i < totalAddressesToBuild; i++) {
            Address__c addr = addressDefaults(i);
            addr.Undeliverable__c = true;
            testAddresses.add(addr);
        }
    }

    private static Address__c addressDefaults(Integer i) {
        Address__c addr = new Address__c();
        addr.Address_Type__c = 'Home';
        addr.MailingStreet__c = 'Street' + i;
        addr.MailingCity__c = 'City' + i;
        addr.MailingState__c = 'Washington';
        addr.MailingPostalCode__c = 'Zip' + i;
        addr.MailingCountry__c = 'United States';
        addr.Geolocation__Latitude__s = Math.min(47.6097 + i, 90);
        addr.Geolocation__Longitude__s = Math.min(-122.3331 + i, 90);
        return addr;
    }


    /*********************************************************************************************************
    * @description Populates Contact's Mailing Address from the Address
    * @param c Contact
    * @param sourceAddress Address
    **********************************************************************************************************/
    static void populateContactAddress(Contact c, Address__c sourceAddress) {
        c.MailingStreet = sourceAddress.MailingStreet__c;
        c.MailingCity = sourceAddress.MailingCity__c;
        c.MailingState = sourceAddress.MailingState__c;
        c.MailingPostalCode = sourceAddress.MailingPostalCode__c;
        c.MailingCountry = sourceAddress.MailingCountry__c;
    }

    /*********************************************************************************************************
    * @description Creates mapping of Addresses by a Household
    * @return Map<Id, Address__c[]> Map of Addresses by a Household
    **********************************************************************************************************/
    static Map<Id, Address__c[]> getAddressesByHousehold() {
        Map<Id, Address__c[]> addressesByHh = new Map<Id, Address__c[]>();

        for (Address__c addr : getAddresses()) {
            if (!addressesByHh.containsKey(addr.Household_Account__c)) {
                addressesByHh.put(addr.Household_Account__c, new Address__c[0]);
            }

            addressesByHh.get(addr.Household_Account__c).add(addr);
        }

        return addressesByHh;
    }

    /*********************************************************************************************************
    * @description Retrieves all Addresses from database
    * @return Address__c[] List of all Addresses
    **********************************************************************************************************/
    static Address__c[] getAddresses() {
        return [
            SELECT Id, Default_Address__c, Household_Account__c, Address_Type__c,
                MailingStreet__c, MailingCity__c, MailingState__c,
                MailingPostalCode__c, MailingCountry__c
            FROM Address__c
        ];
    }

    /*********************************************************************************************************
    * @description Retrieves all Contacts from database
    * @return Contact[] List of all Contacts
    **********************************************************************************************************/
    static Contact[] getContacts() {
        return [
            SELECT Id, Name, AccountId, is_Address_Override__c,
                Current_Address__c, npe01__Primary_Address_Type__c,
                MailingStreet, MailingCity, MailingState, MailingPostalCode,
                MailingCountry, MailingLatitude, MailingLongitude
            FROM Contact
        ];
    }

    /*********************************************************************************************************
    * @description Asserts that an actual Address has the same values as the expected Address
    * @param expected An Address with expected values
    * @param actual An Address that needs to be verified
    **********************************************************************************************************/
    static void assertEquals(Address__c expected, Address__c actual) {
        System.assertEquals(expected.Address_Type__c, actual.Address_Type__c);
        System.assertEquals(expected.MailingStreet__c, actual.MailingStreet__c);
        System.assertEquals(expected.MailingCity__c, actual.MailingCity__c);
        System.assertEquals(expected.MailingState__c, actual.MailingState__c);
        System.assertEquals(expected.MailingPostalCode__c, actual.MailingPostalCode__c);
        System.assertEquals(expected.MailingCountry__c, actual.MailingCountry__c);
    }

    /*********************************************************************************************************
    * @description Asserts that a Contact's Address is the same as the expected Address
    * @param expected An Address with expected values
    * @param c Contact for which Mailing Address needs to be verified
    **********************************************************************************************************/
    static void assertEqualAddresses(Address__c expected, Contact c) {
        System.assertEquals(expected.Id, c.Current_Address__c);
        System.assertEquals(expected.Address_Type__c, c.npe01__Primary_Address_Type__c);
        System.assertEquals(expected.MailingStreet__c, c.MailingStreet);
        System.assertEquals(expected.MailingCity__c, c.MailingCity);
        System.assertEquals(expected.MailingState__c, c.MailingState);
        System.assertEquals(expected.MailingPostalCode__c, c.MailingPostalCode);
        System.assertEquals(expected.MailingCountry__c, c.MailingCountry);
    }

    /*********************************************************************************************************
    * @description Returns whether an Account Billing address matches a Contact Mailing address
    * @return  boolean 
    **********************************************************************************************************/
    public static Boolean isMatchAddressAccCon(Account acc, Contact con) {
        return (acc.BillingStreet == con.MailingStreet &&
            acc.BillingCity == con.MailingCity &&
            acc.BillingState == con.MailingState &&
            acc.BillingPostalCode == con.MailingPostalCode &&
            acc.BillingCountry == con.MailingCountry &&
            acc.BillingLatitude == con.MailingLatitude &&
            acc.BillingLongitude == con.MailingLongitude
        );
    }

    /*******************************************************************************************************
    * @description Builds SOQL for Contact sObject. Fields include standard Mailing fields as well as the one for State and Country Picklist enabled.
    * @return String SOQL
    */
    private static String buildContactSoql() {
        List<String> selectFields = new List<String>{
            'Id', 'AccountId', 'is_Address_Override__c', 'Current_Address__c',
            'MailingStreet', 'MailingCity', 'MailingState', 'MailingCountry', 'MailingPostalCode',
            'MailingLatitude', 'MailingLongitude', 'npe01__Primary_Address_Type__c'};

        if (orgConfig.isStateCountryPicklistsEnabled()) {

            selectFields.add('MailingCountryCode');
            selectFields.add('MailingStateCode');
            selectFields.add('OtherCountryCode');
            selectFields.add('OtherStateCode');
        }

        return new UTIL_Query()
            .withSelectFields(selectFields)
            .withFrom('Contact')
            .build();
    }


    /*******************************************************************************************************
    * @description Builds SOQL for Account sObject. Fields include standard Billing fields as well as the one for State and Country Picklist enabled.
    * @return String SOQL
    */
    private static String buildAccountSoql() {
        List<String> selectFields = new List<String>{
            'Id', 'BillingStreet', 'BillingCity', 'BillingState',  'BillingCountry',
            'BillingPostalCode', 'BillingLatitude', 'BillingLongitude'};

        if (orgConfig.isStateCountryPicklistsEnabled()) {

            selectFields.add('BillingCountryCode');
            selectFields.add('BillingStateCode');
        }

        return new UTIL_Query()
            .withSelectFields(selectFields)
            .withFrom('Account')
            .build();
    }


}
